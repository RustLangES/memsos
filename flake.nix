{
  description = "Memtest rewritten in Rust";

  inputs = {
    fenix-src.url = "github:nix-community/fenix";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { fenix-src, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      lib = nixpkgs.lib;
      pkgs = nixpkgs.legacyPackages.${system};
      fenix = fenix-src.packages.${system};

      # fenix: rustup replacement for reproducible builds
      toolchain = target: fenix.targets.${target}.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-WGTJJbpV6WEv0VHPBqSIqWLCxzHivFNu0okQ2f9LrWU=";
      };

      architectures = [
        { arch = "x86_64"; name = "x86_64"; target = "x86_64-unknown-none"; }
        { arch = "aarch64"; name = "aarch64";  target = "aarch64-unknown-none"; }
        { arch = "aarch64"; name = "aarch64-uefi";  target = "aarch64-unknown-uefi"; }
        { arch = "riscv64"; name = "riscv64-imac";  target = "riscv64imac-unknown-none-elf"; }
        { arch = "riscv64"; name = "riscv64-gc";  target = "riscv64gc-unknown-none-elf"; }
      ];

      mkDevShell = { name, target, ... }: pkgs.mkShell {
        packages = with pkgs; [ qemu just libisoburn ];
        nativeBuildInputs = [ (toolchain target) ];
        shellHook = ''
          echo "DevShell for ${name} (${target})"
        '';
      };

      mkPackage = { name, target, ... }: pkgs.stdenv.mkDerivation {
        pname = "memsos-${name}";
        version = "0.1.0";
        src = lib.cleanSourceWith { src = ./..; };
        nativeBuildInputs = [ (toolchain target) ];
        buildPhase = ''
          cargo build --release --target ${target}
        '';
        installPhase = ''
          mkdir -p $out/bin
          cp target/${target}/release/memsos $out/bin/
        '';
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
