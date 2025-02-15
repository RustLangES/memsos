{
  description = "Memtest rewritten in Rust";
  inputs = {
    crane.url = "github:ipetkov/crane";
    fenix.url = "github:nix-community/fenix";
  };
  
  outputs = { fenix, nixpkgs, ... }@inputs:
    let
      system = "x86_64-linux";
      lib = pkgs.lib;
      pkgs = nixpkgs.legacyPackages.${system};
      crane = inputs.crane.mkLib pkgs;

      # fenix: rustup replacement for reproducible builds
      toolchain = fenix.packages.${system}.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-WGTJJbpV6WEv0VHPBqSIqWLCxzHivFNu0okQ2f9LrWU=";
      };
      # crane: cargo and artifacts manager
      craneLib = crane.overrideToolchain toolchain;

      # Create the runvm binary
      runvm = pkgs.writeShellScriptBin "runvm" ''
        #!/usr/bin/env bash
        set -e

        # Paths to VM images
        if [[ -z "$out" || ! -d "$out" ]]; then
          BASE_PATH="./target"
        else
          BASE_PATH="$out"
        fi

        BIOS_IMG="$BASE_PATH/bios.img"
        UEFI_IMG="$BASE_PATH/uefi.img"

        # Check the required image exists
        if [[ ! -f "$BIOS_IMG" && ! -f "$UEFI_IMG" ]]; then
          echo "Error: No BIOS or UEFI image found."
          exit 1
        fi

        # Choose the image to boot
        IMG_TO_BOOT="$BIOS_IMG"
        if [[ "$1" == "uefi" ]]; then
          IMG_TO_BOOT="$UEFI_IMG"
        fi

        echo "Booting VM using image: $IMG_TO_BOOT"

        # QEMU command (adjust based on your VM needs)
        qemu-system-x86_64 \
          -enable-kvm \
          -m 512M \
          -cpu host \
          -drive file="$IMG_TO_BOOT",format=raw \
          "$@"
      '';

      # Base args, needed to build all crate artifacts and cache them for later builds
      commonArgs = {
        doCheck = false;
        src = lib.cleanSourceWith {
          src = craneLib.path ./..;
        };
      };

      # Build dependencies and images
      memsosDeps = craneLib.buildDepsOnly commonArgs;
      memsos = target: craneLib.buildPackage (commonArgs // {
        pname = "memsos";
        version = "0.1.0";
        cargoArtifacts = memsosDeps;
        buildPhaseCargoCommand = "cargo run -r -- dist";

        postInstall = ''
          mkdir -p $out/bin
          cp ${runvm}/bin/runvm $out/bin/

          cp target/${target}.img $out/${target}.img
        '';
      });
    in {
      packages.${system} = rec {
        default = memsosBIOS;
        memsosBIOS = memsos "bios"; # BIOS package with runvm and bios.img
        memsosUEFI = memsos "uefi"; # UEFI package with runvm and uefi.img
      };

      devShells.${system}.default = craneLib.devShell {
        packages = with pkgs; [ qemu toolchain runvm just ];
      };
    };
}
