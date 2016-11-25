

// #[no_mangle]
pub extern "C" fn main2() -> u8 {

    unsafe {
        asm!("

            //alloc more memory
            mov r13, 0x01   //syscall number (1 - alloc)
            mov r15, rsp    //pass stack pointer to OS
            lea r14, [rip]  //pass instruction pointer to OS
            sysenter
            mov [r13], eax //test memory allocation

            //pause process
            mov r13, 0x03   //syscall number (3 - pause)
            mov r15, rsp    //pass stack pointer to OS
            lea r14, [rip]  //pass instruction pointer to OS
            sysenter


            //terminate process
            mov r13, 0x02   //syscall number (2 - terminate)
            mov r15, rsp    //pass stack pointer to OS
            lea r14, [rip]  //pass instruction pointer to OS
            sysenter


            "::::"intel", "volatile");

    }

    loop {
        // after two sysenters
    }

    return 4;
}
