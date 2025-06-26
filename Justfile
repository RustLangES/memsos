QEMU_FLAGS :=  env_var_or_default("QEMU_FLAGS", "")
OVMF_DIR := "ovmf"
ARCH := "x86_64"
PROFILE := env_var_or_default("PROFILE", "debug")
DIST_DIR := "dist"

default: run

run: ovmf build
    qemu-system-{{ARCH}} -M q35  -d int -drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-{{ARCH}}.fd,readonly=on -drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-{{ARCH}}.fd -cdrom memsos-{{ARCH}}.img {{QEMU_FLAGS}}


ovmf:
    test -d {{OVMF_DIR}} || (mkdir -p {{OVMF_DIR}} && curl -Lo {{OVMF_DIR}}/ovmf-code-{{ARCH}}.fd https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-code-{{ARCH}}.fd &&  curl -Lo {{OVMF_DIR}}/ovmf-vars-{{ARCH}}.fd https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-vars-{{ARCH}}.fd)

build: always kernel
	dd if=/dev/zero of=memsos-{{ARCH}}.img bs=1M count=125 status=progress
	sgdisk --clear --new=1:1M:10M --typecode=1:C12A7328-F81F-11D2-BA4B-00A0C93EC93B memsos-{{ARCH}}.img
	mkfs.fat memsos-{{ARCH}}.img
	mmd -i memsos-{{ARCH}}.img ::EFI
	mmd -i memsos-{{ARCH}}.img ::EFI/BOOT
	mcopy -i memsos-{{ARCH}}.img {{DIST_DIR}}/memsos.efi ::EFI/BOOT/BOOTX64.EFI

kernel: always
	RUST_PROFILE="{{PROFILE}}" cargo build --target {{ARCH}}-unknown-uefi -p memsos
	cp target/{{ARCH}}-unknown-uefi/{{PROFILE}}/memsos.efi {{DIST_DIR}}/memsos.efi

always:
	mkdir -p {{DIST_DIR}}

lint:
	cargo clippy --all-targets --all-features -- -D warnings
fmt:
	cargo fmt --all
