#![feature(asm, link_llvm_intrinsics)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

extern crate rand;

use core::arch::x86_64 as x86;
use core::panic::PanicInfo;

use rand::prng::XorShiftRng;
use rand::{RngCore, SeedableRng};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut buf1 = [0u8; 1];
    let mut buf = [0u8; 20];

    let mut rng = XorShiftRng::seed_from_u64(unsafe { x86::_rdtsc() } as u64);
    let vga_buffer = 0xb8000 as *mut u8;
    loop {
        for (i, &byte) in HELLO.iter().enumerate() {
            unsafe {
                *vga_buffer.offset(i as isize * 2) = byte;
                rng.fill_bytes(&mut buf1);
                *vga_buffer.offset(i as isize * 2 + 1) = buf1[0];
            }
        }
    }
}

fn u64_to_str(mut i: u64, buf: &mut [u8]) -> &str {
    let mut j = 0;
    loop {
        buf[j] = (i % 10) as u8 + 48;
        i /= 10;
        j += 1;
        if i == 0 {
            break;
        }
    }
    for k in 0..j / 2 {
        let tmp = buf[k];
        buf[k] = buf[j - k - 1];
        buf[j - k - 1] = tmp;
    }
    core::str::from_utf8(&buf[0..j]).expect("invalid utf8 output")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn check_u64_output() {
        let mut i = core::u64::MAX;
        let mut buf = [0u8; 20];
        loop {
            assert_eq!(i.to_string(), u64_to_str(i, &mut buf));
            if i == core::u64::MIN {
                break;
            }
            i >>= 1;
        }
    }
}
