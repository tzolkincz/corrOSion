

// zatim jen jako fce v kernel space, bez crt0



#[no_mangle]
pub extern "C" fn main() -> u8 {


    unsafe {

        let a = 5 + 48; //48 is offset of numbers in ascii table

        // print on hardcoded VGA location
        let line_colored = [a, 0x4a as u8];
        let buffer_ptr = (0xb8000 + 160 * 7) as *mut _;
        *buffer_ptr = line_colored;


        asm!("
        sysenter
        nop"::::"intel", "volatile");
    }


    return 4;
}
