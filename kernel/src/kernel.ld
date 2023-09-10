OUTPUT_ARCH(riscv)
ENTRY(_entry)

SECTIONS {
    . = 0x80000000;

    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);

    .rodata : {
        *(.srodata .srodata.*)
        *(.rodata .rodata.*)
    }

    . = ALIGN(4K);

    .data : {
        *(.sdata .sdata.*)
        *(.data .data.*)
    }

    . = ALIGN(4K);

    .bss : {
        *(.bss .bss.*)
        . = ALIGN(4K);
    }

    _kernel_end = .;
}