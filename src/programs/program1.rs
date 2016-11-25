

#[no_mangle]
pub extern "C" fn main() -> u8 {
    unsafe {
        //*((0xb8000 + 160 * 6) as *mut _) = ['P' as u8, 0x4f as u8];

        asm!("
            mov r10, 0x12345 //test register preservation
            //mov rsi, rsp  //registerum rsi a rdi se nejak nechtelo, tak jsem pouzil rsi, rdi (docasne :D)
            //lea rdi, [rip]

            mov r15, rsp    //pass stack pointer to OS
            lea r14, [rip]  //pass instruction pointer to OS

            sysenter
            nop

            lea r14, [rip] //pass instruction pointer to OS
            sysenter
            "::::"intel", "volatile");

        //*((0xb8000 + 160 * 6) as *mut _) = ['P' as u8, 0x2f as u8];
    }

    loop {
        //after two sysenters
    }

    return 4;
}
