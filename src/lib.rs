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
mod memory;
mod syscall;
mod scheduler;

pub use programs::program1; //export for linker
pub use programs::program2; //export for linker

#[no_mangle]
#[naked]
#[cfg(target_arch = "x86_64")]  // ??? -- seen in Redox OS
pub unsafe extern "C" fn kint_zero() -> ! {
    asm!("
       push rax
       push rcx
       push rdx
       push r8
       push r9
       push r10
       push r11
       push rdi
       push rsi" :::: "intel", "volatile");

    easy_print_line(0, "kint_zero", 0xf4);

    asm!("
       pop rsi
       pop rdi
       pop r11
       pop r10
       pop r9
       pop r8
       pop rdx
       pop rcx
       pop rax" :::: "intel");

    asm!("iretq" :::: "intel");

    panic!();
}

#[no_mangle]
pub extern "C" fn kentry() -> ! {
    // easy_print_line(24, "kentry .", 0x4f);
    update_status_line();


    //    unsafe {asm!("int 0"::::"intel");}

    let pid = process::dispatch_off();
    syscall::handle_syscall(pid);


    // easy_print_line(24, "kentry !", 0x2f);
    // loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    // ATTENTION: we have a very small stack and no guard page
    erase_whole_screen();
    set_logo();
    update_status_line();
    easy_print_line(0, "kmain .", 0x4f);

    #[allow(unused_mut)]
    unsafe {
        let mut gdt64_kcode: u64;
        asm!("mov ecx, 0x174 \n rdmsr" : "={eax}"(gdt64_kcode) ::: "intel");

        idt::IDT[0].set_offset(gdt64_kcode as u16, kint_zero as usize);
        idt::IDT[0].set_flags(0b10001110);
        idt::IDTR.set_slice(&idt::IDT);
        idt::IDTR.load();

        *((0xb8000 + 160 * 1) as *mut _) = [gdt64_kcode as u8 + '0' as u8, 0x1f as u8];
    }

    // putc((0x1f, 'a'));
    // putc((0x1f, 'h'));
    // putc((0x1f, 'o'));
    // putc((0x1f, 'j'));
    // putc((0x1f, '\n'));
    // putc((0x1f, '\n'));
    // putc((0x1f, '\n'));
    // putc((0x1f, 'a'));
    // putc((0x1f, '\t'));
    // putc((0x1f, 'h'));
    // putc((0x1f, '\t'));
    // putc((0x1f, 'o'));
    // putc((0x1f, '\t'));
    // putc((0x1f, 'j'));
    // putc((0x1f, '\n'));
    // putc((0x1f, '\n'));
    // putc((0x1f, '\n'));
    // putc((0x1f, '\r'));
    // putc((0x1f, 'a'));
    // putc((0x1f, 'h'));
    // putc((0x1f, 'o'));
    // putc((0x1f, 'j'));
    // putc((0x1f, '\n'));
    // putc((0x1f, '\n'));
    // putc((0x1f, '\n'));

    process::load_apt();
    process::create_process(0);
    process::create_process(1);
    scheduler::reschedule();
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

pub fn easy_print_line(_line: i32, text: &str, attr: u8) {
    let line: u8 = _line as u8;
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

const TAB_SIZE: u8 = 8;
pub static mut CURSOR: (u8, u8) = (0, 0);

pub fn putc(ac: (u8, char)) {
    let (_, character) = ac;
    unsafe {
        let (row, col) = CURSOR;
        if character == '\t' {
            CURSOR = (row, col + TAB_SIZE);
            if col + TAB_SIZE >= COLS {
                putc((0, '\n'));
                let (row, _) = CURSOR;
                CURSOR = (row, col + TAB_SIZE - COLS);
            }
        } else if character == '\r' {
            CURSOR = (row, 0)
        } else if character == '\n' {
            CURSOR = (row + 1, col);
            if row + 1 == ROWS - 1 {
                CURSOR = (0, col) // Skip kernel status line
            }
        } else {
            set_attr_char(ac, CURSOR);
            CURSOR = (row, col + 1); // Shift to the right
            if col + 1 == COLS {
                // Wrap line
                putc((0, '\n'));
                putc((0, '\r'));
            }
        }
    }
}

pub static mut STATUS_LINE: [(u8, char); COLS as usize] = [(0x70, '#'); COLS as usize];
pub fn update_status_line() {
    unsafe {
        for col in 0..COLS {
            set_attr_char(STATUS_LINE[col as usize], (24, col));
        }
    }
}

pub fn set_logo() {
    let bg = 0xf0;
    let red = 0x0C;
    let blue = 0x09;
    unsafe {
        STATUS_LINE[(COLS - 9) as usize] = (bg | blue, 'c');
        STATUS_LINE[(COLS - 8) as usize] = (bg | blue, 'o');
        STATUS_LINE[(COLS - 7) as usize] = (bg | blue, 'r');
        STATUS_LINE[(COLS - 6) as usize] = (bg | blue, 'r');
        STATUS_LINE[(COLS - 5) as usize] = (bg | red, 'O');
        STATUS_LINE[(COLS - 4) as usize] = (bg | red, 'S');
        STATUS_LINE[(COLS - 3) as usize] = (bg | blue, 'i');
        STATUS_LINE[(COLS - 2) as usize] = (bg | blue, 'o');
        STATUS_LINE[(COLS - 1) as usize] = (bg | blue, 'n');
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    let bg = 0x40;
    let red = 0x0f;
    unsafe {
        STATUS_LINE[0] = (bg | red, 'P');
        STATUS_LINE[1] = (bg | red, 'A');
        STATUS_LINE[2] = (bg | red, 'N');
        STATUS_LINE[3] = (bg | red, 'I');
        STATUS_LINE[4] = (bg | red, 'C');
        update_status_line(); // Can we use stack?
        loop {
            asm!("hlt":::: "intel", "volatile");
        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    let bg = 0x40;
    let red = 0x0f;
    unsafe {
        STATUS_LINE[0] = (bg | red, 'U');
        STATUS_LINE[1] = (bg | red, 'N');
        STATUS_LINE[2] = (bg | red, 'W');
        STATUS_LINE[3] = (bg | red, 'I');
        STATUS_LINE[4] = (bg | red, 'N');
        STATUS_LINE[5] = (bg | red, 'D');
        update_status_line(); // Can we use stack?
        loop {
            asm!("hlt":::: "intel", "volatile");
        }
    }
}
