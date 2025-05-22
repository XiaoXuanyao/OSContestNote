#![no_std]
#![no_main]

mod syscall;
mod panic;

#[unsafe(no_mangle)]
extern "C" fn _start() {
    syscall::exit(9);
}