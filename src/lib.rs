// Copyright 2015 Philipp Oppermann. See the README.md
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(lang_items)]
#![feature(asm)]
#![no_std]

extern crate rlibc;

mod process;
mod programs;


#[no_mangle]
pub extern "C" fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page

    erase_whole_screen();

    easy_print_line(0, "this is my example output!", 0x1f);


    process::load_apt();

    easy_print_line(1, "apt created", 0x2c);

    loop {}
}


/**
 * for debug purposes
 */
const VGA_TEXT_BUFFER: u32 = 0xb8000;
const COLS: u8 = 80;
const ROWS: u8 = 25;

pub fn set_attr_char(ac: (u8, char), position: (u8, u8)) {
    let (attr, character) = ac;
    let (row, col) = position;
    let cell = (VGA_TEXT_BUFFER + 2 * (row as u32 * COLS as u32 + col as u32)) as *mut _;

    unsafe { *cell.offset(1) = attr };
    unsafe { *cell = character as u8 };
}

pub fn easy_print_line(line: u8, text: &str, attr: u8) {
    let mut i = 0;
    for char_byte in text.chars() {
        set_attr_char((attr, char_byte), (line, i));
        i += 1;
    }

    // fill rest of line with spaces
    while i < COLS {
        set_attr_char((attr, ' '), (line, i));
        i += 1;
    }
}

pub fn erase_whole_screen() {
    for row in 0..ROWS {
        for col in 0..COLS {
            set_attr_char((0x0f, ' '), (row, col));
        }
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
