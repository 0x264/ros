.attribute arch, "rv64gc" #maybe llvm's bug: https://github.com/rust-lang/rust/issues/80608

.section .text.entry
.balign 16
.global _entry
_entry:
    # set stack for every hart
    csrr t0, mhartid
    addi t0, t0, 1
    li t1, {STACK_SIZE}
    mul t0, t0, t1
    la sp, STACK_BASE
    add sp, sp, t0

    # jump to rust code
    call rust_start # never return

.type _entry STT_FUNC
.size _entry, . - _entry