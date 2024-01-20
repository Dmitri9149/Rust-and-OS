#![no_std] // no std library

use core::panic::PanicInfo;
// the function which is called on panic
#[panic_handler]
fn panic(_info:&PanicInfo) -> ! {
    loop {}
}
fn main() {} // eliminate println! , not working without std::
