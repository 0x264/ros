KERNEL = target/riscv64gc-unknown-none-elf/debug/kernel

QEMU = qemu-system-riscv64

QEMUFLAGS = -machine virt\
			-m 128M\
			-smp 4\
			-nographic\
			-bios none

run: kernel
	$(QEMU) $(QEMUFLAGS) -kernel $(KERNEL)

kernel:
	cargo build

clean:
	cargo clean

.PHONY: clean kernel