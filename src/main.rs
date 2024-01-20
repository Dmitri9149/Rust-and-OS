#![no_std] // don't link Rust  std library
#![no_main] // disable all Rust-level entry points
// 'panic abort' are added to Cargo to eliminate the eh_personality 
// errors, it stops unwinding of stack after abort 

// has to implement our own entry point (not to use crt0 of C)

use core::panic::PanicInfo;

// the function which is called on panic
#[panic_handler]
fn panic(_info:&PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // compile should output function with name 'start', do not mangle
            // use C calling convention 'extern "C"' 
pub extern "C" fn _start () -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

