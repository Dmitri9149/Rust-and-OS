#![no_std] // no std library
// 'panic abort' are added to Cargo to eliminate the eh_personality 
// errors, it stops unwinding of stack after abort 

use core::panic::PanicInfo;
// the function which is called on panic
#[panic_handler]
fn panic(_info:&PanicInfo) -> ! {
    loop {}
}
fn main() {} // eliminate println! , not working without std::
