{
  description = "Memtest rewritten in Rust";

  inputs = {
    crane.url = "github:ipetkov/crane";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix-src.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
    limine = {
      url = "github:limine-bootloader/limine/v8.x-binary";
      flake = false;
    };
  };

  outputs = { crane, nixpkgs, fenix-src, flake-utils, limine, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        lib = nixpkgs.lib;
        pkgs = nixpkgs.legacyPackages.${system};
        fenix = fenix-src.packages.${system};
        systemToTarget = {
          "aarch64-darwin" = "aarch64-apple-darwin";
          "aarch64-linux" = "aarch64-unknown-linux-gnu";
          "i686-linux" = "i686-unknown-linux-gnu";
          "x86_64-darwin" = "x86_64-apple-darwin";
          "x86_64-linux" = "x86_64-unknown-linux-gnu";
        };

        hostTarget = systemToTarget.${system} or (throw "Unsupported system: ${system}");
        toolchain = target: fenix.combine [
          (fenix.targets.${hostTarget}.default.rust-std)
          (fenix.targets.${hostTarget}.default.toolchain)
          (fenix.targets.${target}.latest.toolchain)
        ];
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
          doCheck = false;

          RUST_PROFILE = "release";
          "CARGO_TARGET_${target_name}_LINKER" = "${pkgs.stdenv.cc.targetPrefix}cc";
          "CARGO_TARGET_${target_name}_RUNNER" = "qemu-${arch}";

          nativeBuildInputs = with pkgs; [ gnumake xorriso ];

          postInstall = ''
            LIMINE_DIR="$out/limine"
            mkdir -p $LIMINE_DIR
            cp ${limine}/* $LIMINE_DIR
            make -C "$LIMINE_DIR"

            mkdir -p $out/iso_root/boot
            cp $out/bin/memsos-boot $out/iso_root/boot/kernel

            mkdir -p $out/iso_root/boot/limine
            cp "$LIMINE_DIR/limine-bios.sys" $out/iso_root/boot/limine/
            cp "$LIMINE_DIR/limine-bios-cd.bin" $out/iso_root/boot/limine/
            cp "$LIMINE_DIR/limine-uefi-cd.bin" $out/iso_root/boot/limine/
            cp ${./limine.conf} $out/iso_root/boot/limine/

            xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
              -no-emul-boot -boot-load-size 4 -boot-info-table \
              --efi-boot boot/limine/limine-uefi-cd.bin \
              -efi-boot-part --efi-boot-image --protective-msdos-label \
              $out/iso_root -o $out/memsos-${name}.iso

            "$LIMINE_DIR/limine" bios-install $out/memsos-${name}.iso
            rm -rf $out/bin $out/iso_root $out/limine
          '';
        };

        listApps = pkgs.writeShellScriptBin "list-apps" ''
          echo "Available apps/packages:"
          ${lib.concatMapStringsSep "\n" ({ name, ... }: ''echo "  - ${name}"'') architectures}
        '';
      in {
        devShells = lib.listToAttrs (map ({ name, ... }@args: {
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

        packages = (lib.listToAttrs (map ({ name, ... }@args: {
          inherit name;
          value = mkPackage args;
        }) architectures)) // {
          # Default Package
          default = mkPackage {
            name = "x86_64";
            arch = "x86_64";
            target = "x86_64-unknown-none";
          };
        };

        apps = lib.listToAttrs (map ({ arch, name, target, ... }@args: {
          inherit name;
          value = {
            type = "app";
            program = pkgs.writeShellScriptBin "run-${name}" ''
              qemu-system-${arch} \
                -cdrom ${mkPackage args}/memsos-${name}.iso \
                -M q35 \
                -no-reboot \
                -no-shutdown \
                -d int
            '';
          };
        }) architectures) // {
          list = {
            type = "app";
            program = "${listApps}/bin/list-apps";
          };
          # Default App
          default = {
            type = "app";
            program = pkgs.writeShellScriptBin "run-default" ''
              qemu-system-x86_64 \
                -cdrom ${mkPackage { arch = "x86_64"; name = "x86_64"; target = "x86_64-unknown-none"; }}/memsos-x86_64.iso \
                -M q35 \
                -no-reboot \
                -no-shutdown \
                -d int
            '';
          };
        };
      }
    );
}
