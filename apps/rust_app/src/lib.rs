//
// Copyright 2019, DornerWorks
//
// This software may be distributed and modified according to the terms of
// the BSD 2-Clause license. Note that NO WARRANTY is provided.
// See "LICENSE_BSD2.txt" for details.
//
// @TAG(DORNERWORKS_BSD)
//

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
