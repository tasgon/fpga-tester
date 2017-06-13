extern crate memmap;

use std::env;
use memmap::{Mmap, Protection};
use std::fs::File;
use std::io::Write;

#[cfg(target_arch = "arm")]
fn write(val: u8) -> bool {
    let file = File::open("/dev/mem").unwrap();
    let mut map = Mmap::open_with_offset(&file, Protection::ReadWrite, 0xff200000, 4096).unwrap();
    let write_size = unsafe { map.as_mut_slice() }.write(&[val]).unwrap();

    if write_size > 0 { true } else { false }
}

#[cfg(not(target_arch = "arm"))]
fn write(val: u8) -> bool { false }

fn main() {
    let do_write = env::args().count() < 3; //don't write the value if there is more than one extra argument
    let value: u8 = env::args().last().unwrap().parse::<u8>().unwrap();
    println!("Val: {}", value);
    
    match do_write {
        true => match write(value) {
            true => println!("byte written"),
            false => println!("byte not written")
        },
        false => println!("ignoring data")
    }
}
