#![no_std]
#![no_main]

#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(negative_impls)]
#![feature(alloc_layout_extra)]

#[macro_use]
mod print;

mod panic;
mod start;
mod param;
mod reg;
mod memlayout;
mod uart;
mod alloc;
mod spinlock;
mod kalloc;