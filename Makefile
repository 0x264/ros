BUILDTYPE = debug

ifeq ($(BUILDTYPE), debug)
CARGOFLAGS =
else ifeq ($(BUILDTYPE), release)
CARGOFLAGS = --release
else
$(error "build type only support debug and release, found: $(BUILDTYPE)")
endif

KERNEL = target/riscv64gc-unknown-none-elf/$(BUILDTYPE)/kernel

QEMU = qemu-system-riscv64

QEMUFLAGS = -machine virt\
			-m 128M\
			-smp 4\
			-nographic\
			-bios none

run: kernel
	$(QEMU) $(QEMUFLAGS) -kernel $(KERNEL)

kernel:
	cargo build $(CARGOFLAGS)

clean:
	cargo clean

.PHONY: clean kernel