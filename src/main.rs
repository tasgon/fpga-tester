extern crate memmap;

use std::env;
use memmap::{Mmap, Protection};

#[cfg(target_arch = "arm")]
fn write(val: u8) -> bool {
    let map = Mmap::open_with_offset("/dev/mem", Protection::ReadWrite, 0xff200000, 4096).unwrap();
    unsafe { map.as_mut_slice() }.write(value).unwrap();

    true
}

#[cfg(not(target_arch = "arm"))]
fn write(val: u8) -> bool { false }

fn main() {
    let value: u8 = env::args().last().unwrap().parse::<u8>().unwrap();
    println!("Val: {}", value);
    
    match write(value) {
        true => println!("byte written"),
        false => println!("byte not written")
    }
}
