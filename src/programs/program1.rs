

#[no_mangle]
pub extern "C" fn main() -> u8 {
    unsafe {
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

            hlt //this will be never called
            "::::"intel", "volatile");

    }

    loop {
        // after two sysenters
    }

    return 4;
}
