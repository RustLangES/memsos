QEMU_FLAGS :=  env_var_or_default("QEMU_FLAGS", "")
IMAGE_NAME := "toggle-x86_64"
OVMF_DIR := "ovmf"
LIMINE_DIR := "limine"
ARCH := "x86_64"

# Run vm

run-uefi: build ovmf
  qemu-system-{{ARCH}} \
    -M q35 \
    -no-reboot \
    -no-shutdown \
    -d int \
    -drive if=pflash,unit=0,format=raw,file=ovmf/ovmf-code-{{ARCH}}.fd,readonly=on \
    -drive if=pflash,unit=1,format=raw,file=ovmf/ovmf-vars-{{ARCH}}.fd \
    -cdrom {{IMAGE_NAME}}.iso \
    {{QEMU_FLAGS}}

run-bios: build
  qemu-system-{{ARCH}} \
    -M q35 \
    -cdrom {{IMAGE_NAME}}.iso \
    -S \
    -d int \
    -no-reboot \
    -no-shutdown \
    -gdb tcp::1234 \
    -boot d \
    {{QEMU_FLAGS}}


# OVMF build

ovmf:
    test -d {{OVMF_DIR}} || (mkdir -p {{OVMF_DIR}} && curl -Lo {{OVMF_DIR}}/ovmf-code-{{ARCH}}.fd https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-code-{{ARCH}}.fd &&  curl -Lo {{OVMF_DIR}}/ovmf-vars-{{ARCH}}.fd https://github.com/osdev0/edk2-ovmf-nightly/releases/latest/download/ovmf-vars-{{ARCH}}.fd)

# Limine (bootloader) build

limine:
  test -d {{LIMINE_DIR}} || git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1
  make -C {{LIMINE_DIR}}

# Kernel build

kernel:
  just kernel/


# Image build

build: limine kernel
  rm -rf iso_root
  mkdir -p iso_root/boot
  cp -v kernel/kernel iso_root/boot/
  mkdir -p iso_root/boot/limine
  cp -v limine.conf iso_root/boot/limine/
  mkdir -p iso_root/EFI/BOOT    

  cp -v limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/
  cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
  cp -v limine/BOOTIA32.EFI iso_root/EFI/BOOT/
  xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot boot/limine/limine-uefi-cd.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    iso_root -o {{IMAGE_NAME}}.iso
 
  ./limine/limine bios-install {{IMAGE_NAME}}.iso 
  rm -rf iso_root

clean:
  just kernel/ clean
  rm -rf iso_root {{IMAGE_NAME}}.iso
  rm -rf limine ovmf

format:
  just kernel/ format

clippy:
  just kernel/ clippy

default: run-uefi

lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all -- --check

fmt-fix:
	cargo fmt

test:
	cargo test
