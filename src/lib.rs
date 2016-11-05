// Copyright 2015 Philipp Oppermann. See the README.md
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(lang_items)]
#![no_std]

extern crate rlibc;

#[no_mangle]
pub extern "C" fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page


    easy_print_line(0, "this is my example output!", 0x1f);


    loop {}
}


/**
 * for debug purposes
 */
const LINE_LENGTH: usize = 80;
pub fn easy_print_line(line_number: i32, line: &str, color: u8) {

    let mut line_colored = [color; 2 * LINE_LENGTH];
    let mut i = 0;
    for char_byte in line.chars() {
        line_colored[i * 2] = char_byte as u8;
        i += 1;
    }

    // fill rest of line with spaces
    while i < LINE_LENGTH {
        line_colored[i * 2] = ' ' as u8;
        i += 1;
    }

    // write to the VGA text buffer
    let buffer_ptr = (0xb8000 + 80 * 4 * line_number) as *mut _;
    unsafe { *buffer_ptr = line_colored };

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
