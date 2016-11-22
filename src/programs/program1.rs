

// zatim jen jako fce v kernel space, bez crt0



#[no_mangle]
pub extern "C" fn main() -> u8 {

//    ..::easy_print_line(4, "program1", 0x1f);

    // to know we are here
    loop {
        unsafe {
            let a = 10 + 48; //48 is offset of numbers in ascii table

            // print on hardcoded VGA location
            let line_colored = [a, 0x4a as u8];
            let buffer_ptr = (0xb8000 + 160 * 8) as *mut _;
            *buffer_ptr = line_colored;


            asm!("
            nop"::::"intel", "volatile");
        }
    }


    unsafe {
        asm!("
            mov ecx, esp
            mov edx, 0x1000000
            mov eax, 8;
            sysenter"
            ::::"intel");
    }
    return 4;
}
