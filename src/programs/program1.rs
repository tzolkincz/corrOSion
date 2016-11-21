

// zatim jen jako fce v kernel space, bez crt0



#[no_mangle]
pub extern "C" fn main() -> u8 {

    // to know we are here
    loop {
        unsafe {
            asm!("nop");
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
