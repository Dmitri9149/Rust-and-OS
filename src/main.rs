#![no_std] // no std library
#![no_main] // will not use normal entry point
// 'panic abort' are added to Cargo to eliminate the eh_personality 
// errors, it stops unwinding of stack after abort 

// has to implement our own entry point (not to use crt0 of C)

use core::panic::PanicInfo;
// the function which is called on panic
#[panic_handler]
fn panic(_info:&PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // compile should output function with really name 'start'
            // use C calling convention 'extern "C"' 
pub extern "C" fn _start () -> ! {
    loop {}
}

