// Copyright 2015 Philipp Oppermann. See the README.md
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(asm, lang_items, naked_functions)]
#![no_std]

extern crate rlibc;

mod idt;
mod process;
mod programs;

pub use programs::program1; //export for linker

#[no_mangle]
#[naked]
#[cfg(target_arch = "x86_64")]  // ??? -- seen in Redox OS
pub unsafe extern "C" fn kint_zero() {
    asm!("push rax
       push rcx
       push rdx
       push r8
       push r9
       push r10
       push r11
       push rdi
       push rsi

       call print_for_kint_zero

       pop rsi
       pop rdi
       pop r11
       pop r10
       pop r9
       pop r8
       pop rdx
       pop rcx
       pop rax

       //hlt
       iretq" :::: "intel", "volatile");
}

#[no_mangle]
pub extern "C" fn kentry() {
    easy_print_line(24, "kentry .", 0x4f);

    unsafe {asm!("int 0"::"{rbx}"(kint_zero as *const ())::"intel");}

    easy_print_line(24, "kentry !", 0x2f);
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() {
    // ATTENTION: we have a very small stack and no guard page
    easy_print_line(0, "kmain .", 0x4f);

    unsafe {
        let mut gdt64_kcode: u64;
        asm!("mov ecx, 0x174 \n rdmsr" : "={eax}"(gdt64_kcode) ::: "intel");

        idt::IDT[0].set_func(gdt64_kcode as u16, kint_zero);
        idt::IDT[0].set_flags(0b10001110);
        idt::IDTR.set_slice(&idt::IDT);
        idt::IDTR.load();

        *((0xb8000 + 160 * 1) as *mut _) = [gdt64_kcode as u8 + '0' as u8, 0x1f as u8];
    }

    process::load_apt();

    easy_print_line(0, "kmain !", 0x2f);
    loop {}
}


/**
 * for debug purposes
 */
#[no_mangle]
pub extern "C" fn print_for_kint_zero() {
    easy_print_line(0, "kint_zero", 0xf4);
}

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
