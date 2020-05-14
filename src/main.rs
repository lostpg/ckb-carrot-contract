#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::{ckb_constants, debug, default_alloc, entry, syscalls};

#[no_mangle]
pub fn main() -> i8 {
    let mut index = 0;
    let len = 6;
    let offset = 0;
    debug!("Let's see if you are carrying carrots.");
    while let Ok(buf) =
        syscalls::load_cell_data(len, offset, index, ckb_constants::Source::GroupOutput)
    {
        let buffer = buf;
        if buffer.starts_with("carrot".as_bytes()) {
            debug!("No!!! You have a carrot!");
            return -1;
        }
        index += 1;
    }
    return 0;
}

entry!(main);
default_alloc!();
