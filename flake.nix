{
  description = "Powered Hardware test tool written in Rust";

  inputs = {
    crane.url = "github:ipetkov/crane";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks.url = "github:cachix/git-hooks.nix";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    limine = {
      url = "github:limine-bootloader/limine/v8.x-binary";
      flake = false;
    };
  };

  outputs = {
    crane,
    nixpkgs,
    flake-utils,
    rust-overlay,
    limine,
    pre-commit-hooks,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        lib = nixpkgs.lib;
        hooks = pre-commit-hooks.lib.${system};
        overlays = [(import rust-overlay)];
        variant = builtins.fromJSON (builtins.readFile ./ovmf_sources.json);
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        systemToTarget = system: let
          arch = builtins.elemAt (lib.splitString "-" system) 0;
          os = builtins.elemAt (lib.splitString "-" system) 1;
        in
          if os == "darwin"
          then "${arch}-apple-darwin"
          else if os == "linux"
          then "${arch}-unknown-none"
          else throw "Unsupported system: ${system}";

        hostTarget = systemToTarget system;
        toolchain = target:
          (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
            targets = [hostTarget target];
          };
        craneLib = target: (crane.mkLib pkgs).overrideToolchain (toolchain target);

        architectures = import ./strategies.nix;

        mkDevShell = {
          name,
          target,
          ...
        }:
          (craneLib target).devShell {
            packages = with pkgs; [qemu just libisoburn];
            buildInputs = hook.enabledPackages;
            shellHook = ''
              echo "DevShell for ${name} (${target})"
              ${hook.shellHook}
            '';
          };

        ovmf_pkg = arch: name: let
          version = "2025-02-18";
        in
          pkgs.stdenv.mkDerivation {
            inherit version;
            pname = "ovmf_${arch}";
            src = pkgs.fetchurl {
              url = "https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-${name}-${arch}.fd";
              hash = variant.Nightly.${arch}.sha256.${name};
            };

            unpackPhase = ''
              mkdir -p $out
              cp $src $out/ovmf-${name}-${arch}.fd
            '';
          };

        mkPackage = {
          arch,
          name,
          target,
          lang ? "en_US",
          debug_symbols ? false,
          ...
        }: let
          ovmf_vars = ovmf_pkg arch "vars";
          ovmf_code = ovmf_pkg arch "code";
          uefi_suffix =
            if (arch == "x86_64")
            then "X64"
            else if (arch == "aarch64")
            then "AA64"
            else if (arch == "riscv64")
            then "RISCV64"
            else "IA32";
        in
          (craneLib target).buildPackage {
            pname =
              "memsos-${name}"
              + (
                if debug_symbols
                then "-debug"
                else ""
              );
            version = "0.1.0";
            src = pkgs.lib.cleanSourceWith {
              src = ./.;
              filter = path: type:
                (pkgs.lib.hasSuffix ".json" path)
                || (pkgs.lib.hasSuffix ".ld" path)
                || ((craneLib target).filterCargoSources path type);
            };
            doCheck = false;
            cargoBuildCommand =
              "RUSTFLAGS=\"-C relocation-model=static\" "
              + "cargo build --target ${target} -p kernel"
              + (
                if (!debug_symbols)
                then " --release"
                else ""
              );

            nativeBuildInputs = with pkgs; [gnumake xorriso];
            LANG = "${lang}.UTF-8";

            postInstall = ''
              LIMINE_DIR="$out/limine"
              mkdir -p $LIMINE_DIR
              cp ${limine}/* $LIMINE_DIR
              make -C "$LIMINE_DIR"

              mkdir -p $out/iso_root/boot

              ${lib.optionalString debug_symbols ''
                ${pkgs.binutils}/bin/objcopy --only-keep-debug $out/bin/kernel $out/bin/kernel.sym
              ''}
              cp $out/bin/kernel $out/iso_root/boot/kernel

              mkdir -p $out/ovmf
              cp ${ovmf_vars}/ovmf-vars-${arch}.fd $out/ovmf/ovmf-vars-${arch}.fd
              cp ${ovmf_code}/ovmf-code-${arch}.fd $out/ovmf/ovmf-code-${arch}.fd

              mkdir -p $out/iso_root/boot/limine
              cp "$LIMINE_DIR/limine-bios.sys" $out/iso_root/boot/limine/
              cp "$LIMINE_DIR/limine-bios-cd.bin" $out/iso_root/boot/limine/
              cp "$LIMINE_DIR/limine-uefi-cd.bin" $out/iso_root/boot/limine/
              cp ${./limine.conf} $out/iso_root/boot/limine/limine.conf

              mkdir -p $out/iso_root/EFI/BOOT
              cp -v "$LIMINE_DIR/BOOT${uefi_suffix}.EFI" $out/iso_root/EFI/BOOT/

              xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
                -no-emul-boot -boot-load-size 4 -boot-info-table \
                --efi-boot boot/limine/limine-uefi-cd.bin \
                -efi-boot-part --efi-boot-image --protective-msdos-label \
                $out/iso_root -o $out/memsos-${name}${
                if debug_symbols
                then "-debug"
                else ""
              }.iso

              "$LIMINE_DIR/limine" bios-install $out/memsos-${name}${
                if debug_symbols
                then "-debug"
                else ""
              }.iso
              ${lib.optionalString (!debug_symbols) ''
                rm -rf $out/bin $out/iso_root $out/limine
              ''}
            '';
          };

        listApps = pkgs.writeShellScriptBin "list-apps" ''
          echo "Available apps/packages:"
          ${lib.concatMapStringsSep "\n" ({name, ...}: ''echo "  - ${name}"'') architectures}
          ${lib.concatMapStringsSep "\n" ({name, ...}: ''echo "  - ${name}-debug"'') architectures}
        '';

        generatedArchJson = builtins.toJSON (map ({ arch, ... }: arch) architectures);

        helpApp = pkgs.writeShellScriptBin "help" ''
          echo ""
          echo "Welcome to Memsos"
          echo ""
          echo -e "\033[0;33mAvailable commands:\033[0m"
          echo "  nix build"
          echo "  nix build .#<template>"
          echo "  nix run"
          echo "  nix run .#<template>"
          echo ""
          echo -e "\033[0;33mUse the following template:\033[0m"
          echo "  <arch>-<lang>"
          echo "  <arch>-<lang>-debug"
          echo ""
          echo -e "\033[0;32mExample:\033[0m"
          echo "  nix build .#x86_64-en_US"
          echo "  nix run .#x86_64-en_US-debug"
          echo ""
          echo -e "\033[0;35mAvailable architectures:\033[0m"
          ${lib.concatMapStringsSep "\n" ({arch, ...}: ''echo "  - ${arch}"'') architectures}
        '';

        hook = hooks.run {
          src = ./.;
          hooks = {
            actionlint.enable = true;
            check-json.enable = true;
            pretty-format-json = {
              enable = true;
              excludes = ["ovmf_sources.json"];
            };
            check-executables-have-shebangs.enable = true;
            rustfmt = {
              enable = true;
              packageOverrides = {
                cargo = toolchain hostTarget;
                rustfmt = toolchain hostTarget;
              };
            };
            clippy = {
              enable = true;
              # settings = {
              #   denyWarnings = true;
              #   extraArgs = "-Zbuild-std --workspace";
              # };
              packageOverrides = {
                cargo = toolchain hostTarget;
                clippy = toolchain hostTarget;
              };
            };
          };
        };
      in {
        checks = {
          pre-commit-check = hook;
        };

        devShells =
          lib.listToAttrs (map ({name, ...} @ args: {
              inherit name;
              value = mkDevShell args;
            })
            architectures)
          // {
            # Default Devshell
            default = mkDevShell {
              name = "x86_64";
              arch = "x86_64";
              target = "x86_64-unknown-none";
            };
          };

        formatter = pkgs.alejandra;

        packages =
          (lib.listToAttrs (map ({name, ...} @ args: {
              inherit name;
              value = mkPackage args;
            })
            architectures))
          // (lib.listToAttrs (map ({name, ...} @ args: {
            name = "${name}";
            value = mkPackage args;
          })
          architectures))
          // (lib.listToAttrs (map ({name, ...} @ args: {
            name = "${name}-debug";
            value = mkPackage (args
              // {
                debug_symbols = true;
              });
          })
          architectures))
          // {
            # Default Package
            default = mkPackage {
              name = "x86_64";
              arch = "x86_64";
              target = "x86_64-unknown-none";
            };
          };

        apps =
          (lib.listToAttrs (map ({
              arch,
              name,
              ...
            } @ args: let
              pkg = mkPackage args;
              run = pkgs.writeShellScriptBin "run-${name}" ''
                ${pkgs.qemu}/bin/qemu-system-${arch} \
                  -cdrom ${pkg}/memsos-${name}.iso \
                  -M q35 \
                  -no-reboot \
                  -no-shutdown \
                  --enable-kvm \
                  -cpu host \
                  -rtc base=localtime,clock=host \
                  -drive if=pflash,unit=0,format=raw,file=${pkg}/ovmf/ovmf-code-${arch}.fd,readonly=on \
                  -drive if=pflash,unit=1,format=raw,file=${pkg}/ovmf/ovmf-vars-${arch}.fd,readonly=on \
                  -d int
              '';
            in {
              name = "${name}";
              value = {
                type = "app";
                program = "${run}/bin/run-${name}";
              };
            })
          architectures))
          // (lib.listToAttrs (map ({
              arch,
              name,
              ...
            } @ args: let
              pkg = mkPackage (args
                // {
                  debug_symbols = true;
                });
              run = pkgs.writeShellScriptBin "run-${name}-debug" ''
                ${pkgs.qemu}/bin/qemu-system-${arch} \
                  -cdrom ${pkg}/memsos-${name}-debug.iso \
                  -M q35 \
                  --enable-kvm \
                  -cpu host \
                  -rtc base=localtime,clock=host \
                  -drive if=pflash,unit=0,format=raw,file=${pkg}/ovmf/ovmf-code-${arch}.fd,readonly=on \
                  -drive if=pflash,unit=1,format=raw,file=${pkg}/ovmf/ovmf-vars-${arch}.fd,readonly=on \
                  -d int
              '';
            in {
              name = "${name}-debug";
              value = {
                type = "app";
                program = "${run}/bin/run-${name}-debug";
              };
            })
          architectures))
          // {
            archs = {
              type = "app";
              program = toString (pkgs.writeScript "generate-arch-list" ''
                #!/bin/sh
                echo '${generatedArchJson}'
              '');
            };
            list = {
              type = "app";
              program = "${listApps}/bin/list-apps";
            };

            help = {
              type = "app";
              program = "${helpApp}/bin/help";
            };
            # Default App
            default = let
              arch = "x86_64";
              pkg = mkPackage {
                arch = arch;
                name = "x86_64";
                target = "x86_64-unknown-none";
              };
              run = pkgs.writeShellScriptBin "run-default" ''
                ${pkgs.qemu}/bin/qemu-system-x86_64 \
                  -cdrom ${pkg}/memsos-x86_64-en_US.iso \
                  -M q35 \
                  -no-reboot \
                  -no-shutdown \
                  -drive if=pflash,unit=0,format=raw,file=${pkg}/ovmf/ovmf-code-${arch}.fd,readonly=on \
                  -drive if=pflash,unit=1,format=raw,file=${pkg}/ovmf/ovmf-vars-${arch}.fd,readonly=on \
                  -d int
              '';
            in {
              type = "app";
              program = "${run}/bin/run-default";
            };
          };
      }
    );
}
