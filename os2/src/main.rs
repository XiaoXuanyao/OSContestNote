#![no_std]
#![no_main]

mod sbi;
mod panic;
mod console;

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

core::arch::global_asm!(include_str!("entry.asm"));
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello world!");
    crate::sbi::shutdown();
}