# https://github.com/ryanlg/Bedrock/blob/master/Makefile
target_triple     := i686-unknown-linux-gnu

lib_release  := target/$(target_triple)/release/libtinyos.a
lib_debug    := target/$(target_triple)/debug/libtinyos.a

cdrom_path   := build/kernel.iso
iso_dir_path := build/iso
kernel_elf_path := $(iso_dir_path)/boot/kernel.elf

# =================== Programs ==================
QEMU     := qemu-system-i386

qemu_iso_arg    := -cdrom $(cdrom_path)
qemu_serial_arg := -serial stdio

# =================== Targets ===================
.PHONY: clean

all: debug

grubcfg: grub.cfg
	mkdir -p $(iso_dir_path)/boot/grub/
	cp -f grub.cfg $(iso_dir_path)/boot/grub/

boot: src/boot.asm
	nasm -f elf -o build/boot.o src/boot.asm

debug: grubcfg boot linker.ld
	cargo build
	ld.lld -T linker.ld -o $(kernel_elf_path) $(lib_debug) build/boot.o
	i686-elf-grub-mkrescue -o $(cdrom_path) $(iso_dir_path)

release: grubcfg boot linker.ld
	cargo build --release
	ld.lld -T linker.ld -o $(kernel_elf_path) $(lib_release) build/boot.o
	i686-elf-grub-mkrescue -o $(cdrom_path) $(iso_dir_path)

clean:
	cargo clean
	rm -rf build/

# ---------------- Run ----------------
qemu:
	$(QEMU) $(qemu_iso_arg) $(qemu_serial_arg)

qemu-gdb:
	$(QEMU) $(qemu_iso_arg) $(qemu_serial_arg) -s -S

qemu-noui:
	$(QEMU) $(qemu_iso_arg) -nographic

qemu-noui-gdb:
	$(QEMU) $(qemu_iso_arg) -nographic -s -S
