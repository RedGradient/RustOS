#![no_std]
#![no_main]

mod vga_buffer;

use core::arch::x86_64::{_mm_fmadd_sd, _MM_FROUND_TO_POS_INF};
use core::panic::PanicInfo;
use core::fmt::Write;



// static HELLO: &[u8] = b"Hello, World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}
