{
  description = "Memtest rewritten in Rust";

  inputs = {
    crane.url = "github:ipetkov/crane";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { crane, nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      lib = nixpkgs.lib;
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };

      toolchain = target: p: (p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
        targets = [ target ];
      };
      craneLib = target: (crane.mkLib pkgs).overrideToolchain (toolchain target);

      architectures = [
        { arch = "x86_64"; name = "x86_64"; target = "x86_64-unknown-none"; }
        { arch = "aarch64"; name = "aarch64";  target = "aarch64-unknown-none"; }
        { arch = "aarch64"; name = "aarch64-uefi";  target = "aarch64-unknown-uefi"; }
        { arch = "riscv64"; name = "riscv64-imac";  target = "riscv64imac-unknown-none-elf"; }
        { arch = "riscv64"; name = "riscv64-gc";  target = "riscv64gc-unknown-none-elf"; }
      ];

      mkDevShell = { name, target, ... }: (craneLib target).devShell {
        packages = with pkgs; [ qemu just libisoburn ];
        shellHook = ''
          echo "DevShell for ${name} (${target})"
        '';
      };

      mkPackage = { arch, name, target, ... }: let
        target_name = lib.toUpper (builtins.replaceStrings [ "-" ] [ "_" ] target);
      in (craneLib target).buildPackage {
        pname = "memsos-${name}";
        version = "0.1.0";
        src = (craneLib target).cleanCargoSource ./.;
        cargoExtraArgs = "--target ${target}";
        doCheck = false;

        CARGO_BUILD_TARGET = target;
        "CARGO_TARGET_${target_name}_LINKER" = "${pkgs.stdenv.cc.targetPrefix}cc";
        "CARGO_TARGET_${target_name}_RUNNER" = "qemu-${arch}";
      };

      listApps = pkgs.writeShellScriptBin "list-apps" ''
        echo "Available apps/packages:"
        ${lib.concatMapStringsSep "\n" ({ name, ... }: ''echo "  - ${name}"'') architectures}
      '';
    in {
      devShells.${system} = lib.listToAttrs (map ({ name, ... }@args: {
        inherit name;
        value = mkDevShell args;
      }) architectures) // {
        # Default Devshell
        default = mkDevShell {
          name = "x86_64";
          arch = "x86_64";
          target = "x86_64-unknown-none";
        };
      };

      packages.${system} = (lib.listToAttrs (map ({ name, ... }@args: {
        inherit name;
        value = mkPackage args;
      }) architectures)) // ({
        # Default Package
        default = mkPackage {
          name = "x86_64";
          arch = "x86_64";
          target = "x86_64-unknown-none";
        };
      });

      apps.${system} = lib.listToAttrs (map ({ name, target, ... }@args: {
        inherit name;
        value = {
          type = "app";
          program = "${mkPackage args}/bin/memsos";
        };
      }) architectures) // {
        list = {
          type = "app";
          program = "${listApps}/bin/list-apps";
        };
        # Default App
        default = {
          type = "app";
          program = "${mkPackage { arch = "x86_64"; name = "x86_64"; target = "x86_64-unknown-none"; }}/bin/memsos";
        };
      };
    };
}
