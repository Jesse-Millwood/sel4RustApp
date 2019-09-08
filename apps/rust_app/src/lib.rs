#![no_std]

use core::panic::PanicInfo;
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_routine(a:u32, b:u32)->u32{
    let c = match a {
        1 => 0,
        _ => a
    };
    b+c
}
