AS = nasm
ASFLAGS = -f elf32
RUSTC = cargo rustc
TARGET = i686-unknown-linux-gnu
BUILD=release
CARGOFLAGS = --target $(TARGET) --$(BUILD)
#RUSTFLAGS = --emit obj
LD = i386-elf-ld
LDFLAGS = -T link.ld -melf_i386
OBJECTS = loader.o io.o target/$(TARGET)/$(BUILD)/libtinyos.a gdt.o
QEMU = qemu-system-i386

all: kernel.elf

run: tinyos.iso
	$(QEMU) -cdrom tinyos.iso \
	    -monitor stdio \
	    -no-reboot \
	    -serial file:"log.txt"
clean:
	rm -rf *.o tinyos.iso kernel.elf
	cargo clean

%.o: %.s
	$(AS) $(ASFLAGS) $< -o $@

%.o: %.rs
	$(RUSTC) $(CARGOFLAGS) -- $(RUSTFLAGS)

target/$(TARGET)/$(BUILD)/libtinyos.a: *.rs
	$(RUSTC) $(CARGOFLAGS) -- $(RUSTFLAGS)


verify-nostd:
	$(RUSTC) --target thumbv6m-none-eabi -- $(RUSTFLAGS)

kernel.elf: $(OBJECTS)
	$(LD) $(LDFLAGS) $(OBJECTS) -o kernel.elf

tinyos.iso: kernel.elf
	cp kernel.elf iso/boot/kernel.elf
	mkisofs -R                              \
                -b boot/grub/stage2_eltorito    \
                -no-emul-boot                   \
                -boot-load-size 4               \
                -A os                           \
                -input-charset utf8             \
                -quiet                          \
                -boot-info-table                \
                -o tinyos.iso                   \
                iso
