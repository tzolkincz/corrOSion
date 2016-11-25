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
mod memory;

pub use programs::program1; //export for linker

#[no_mangle]
pub extern "C" fn kentry() {
    easy_print_line(24, "kentry .", 0x4f);

    process::dispatch_off();
    unsafe {
        asm!("
            mov r10, 0xff0fff // test register preservation
            "
            ::
            :: "intel", "volatile"
        );}
    process::dispatch_on(0);

    easy_print_line(24, "kentry !", 0x2f);
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() {
    // ATTENTION: we have a very small stack and no guard page
    easy_print_line(0, "kmain .", 0x4f);

    process::load_apt();
    process::create_prcess(0);
    process::dispatch_on(0);

    easy_print_line(0, "kmain !", 0x2f);
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
    let buffer_ptr = (0xb8000 + LINE_LENGTH as i32 * 2 * line_number) as *mut _;
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
