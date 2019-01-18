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

const CHARACTERS: usize = 80 * 25;
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut rng = XorShiftRng::seed_from_u64(unsafe { x86::_rdtsc() } as u64);

    let mut chars = [0u8; CHARACTERS];
    let mut colors = [0u8; CHARACTERS];

    loop {
        rng.fill_bytes(&mut chars);
        rng.fill_bytes(&mut colors);
        for (i, (&c, color)) in chars.iter().zip(colors.iter()).enumerate() {
            unsafe {
                *VGA_BUFFER.offset(i as isize * 2) = c;
                *VGA_BUFFER.offset(i as isize * 2 + 1) = *color;
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
