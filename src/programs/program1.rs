

// zatim jen jako fce v kernel space, bez crt0



#[no_mangle]
pub extern "C" fn main() -> u8 {
    unsafe {
        *((0xb8000 + 160 * 6) as *mut _) = ['P' as u8, 0x4f as u8];

        asm!("
            //hlt //is privileged (cant be checked we are really at CPL3)
            sysenter
            nop"::::"intel");

        *((0xb8000 + 160 * 6) as *mut _) = ['P' as u8, 0x2f as u8];
    }

    return 4;
}
