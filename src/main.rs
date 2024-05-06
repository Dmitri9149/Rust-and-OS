#![no_std] // don't link Rust  std library   
#![no_main] // disable all Rust-level entry points
// 'panic abort' are added to Cargo to eliminate the eh_personality 
// errors, it stops unwinding of stack after abort 

// has to implement our own entry point (not to use crt0 of C)

use core::panic::PanicInfo;

mod vga_buffer;

// static HELLO: &[u8] = b"Hello World"; // the VGA buffer is located at address 0xb8000

// compile should output function with name 'start', do not mangle 
// use C calling convention 'extern "C"'
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default 
#[no_mangle] 
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}

// the function which is called on panic
#[panic_handler]
 
fn panic(_info:&PanicInfo) -> !{
    loop {}
}

