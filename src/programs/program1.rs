

#[no_mangle]
pub extern "C" fn main() -> u8 {
    unsafe {
        // *((0xb8000 + 160 * 6) as *mut _) = ['P' as u8, 0x4f as u8];

        asm!("
            mov r10, 0x12345 //test register preservation


            mov r13, 0x01   //syscall number (1 - alloc)
            //mov rsi, rsp  //rsi a rdi have some issues (no_mandle is not working? (@TODO))
            //lea rdi, [rip]
            mov r15, rsp    //pass stack pointer to OS
            lea r14, [rip]  //pass instruction pointer to OS
            //ACHTUNG! must be last instruction befor syscall
            //(or just r14 must point to sysenter)
            sysenter


            mov eax, 0x001
            mov [r13], eax //test memory allocation
            "::::"intel", "volatile");

        // *((0xb8000 + 160 * 6) as *mut _) = ['P' as u8, 0x2f as u8];
    }

    loop {
        // after two sysenters
    }

    return 4;
}
