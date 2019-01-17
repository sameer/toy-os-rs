#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(target_os = "linux")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    main();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
