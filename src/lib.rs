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

pub use programs::program1; //export for linker

#[no_mangle]
pub extern "C" fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page

    unsafe {
        let rust_main_address: *const i64 = core::mem::transmute_copy(&rust_main);

        // Set model specific registers for sysenter/sysexit
        asm!("
            cli // disable interrupts

            mov ecx, 0x174 // writes KERNEL_CS to model specific registers
            mov edx, 0
            mov eax, 0x08
            wrmsr

            mov ecx, 0x175 // writes kernel ESP to model specific registers
            mov edx, 0
            mov eax, esp //here we knows the current rust_main stack
            //(kernel stack will be reseted on sysenter)
            wrmsr

            mov ecx, 0x176 // writes kernel EIP to model specific registers
            mov rdx, r10 //set to all 64b register
            shr rdx, 32  //shift value to right by 32 (because edx register will be used by wrmsr)
            mov rax, r10
            wrmsr
            "::"{r10}"(rust_main_address)::"volatile","intel");
    }


    easy_print_line(0, "this is my example output!", 0x1f);


    process::load_apt();

    easy_print_line(1, "apt created", 0x2c);

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
