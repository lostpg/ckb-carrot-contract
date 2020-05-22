#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::{CellField, Source, SysError},
    debug, default_alloc, entry,
    syscalls::load_cell_by_field,
};
use godwoken_types::{packed::*, prelude::*};

#[no_mangle]
pub fn main() -> i8 {
    let mut index = 0;
    const LEN: usize = 1024;
    let offset = 0;
    let mut buf = [0u8; LEN];
    loop {
        match load_cell_by_field(
            &mut buf,
            offset,
            index,
            Source::GroupOutput,
            CellField::Type,
        ) {
            Ok(len) => {
                debug!("Cell {}, returns len: {}", index, len);
                debug!("{:?}", &buf[..len]);
                let type_script = Script::new_unchecked(buf[..len].into());
                let args: Vec<u8> = type_script.args().unpack();
                debug!("Cell {}, args: {:?}", index, args);
                if args.starts_with(b"carrot") {
                    debug!("Got the carrot!");
                    return -1;
                }
            }
            Err(err) => {
                debug!("Failed to invoke syscall: {:?}", err);
                if err == SysError::IndexOutOfBound {
                    break;
                }
            }
        }
        index += 1;
    }
    return 0;
}

entry!(main);
default_alloc!();
