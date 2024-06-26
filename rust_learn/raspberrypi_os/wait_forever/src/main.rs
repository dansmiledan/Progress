#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]


mod bsp;
mod console;
mod cpu;
mod panic_wait;
mod print;
mod synchronization;

unsafe fn kernel_init() -> ! {
	println!("[0]Hello from Rust!");
	println!("[1] Chars written: {}", console::console().chars_written());
    println!("[2] Stopping here.");
	cpu::wait_forever();
}