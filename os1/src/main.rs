// os1/src/main.rs
#![no_std]
#![no_main]

mod lang_items;
mod syscall;
mod print;
mod exit;

#[unsafe(no_mangle)]
extern "C" fn _start() {
    println!("Hello, world!");
    crate::exit::sys_exit(9);
}
