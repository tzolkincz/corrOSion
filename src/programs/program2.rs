

// #[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn main2() -> u8 {

    unsafe {
        asm!("

            //acquire mutex
            mov r13, 0x04   //syscall number (4 - acquire mutex)
            mov r12, 0x00   //mutex id
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


            "::::"intel", "volatile");

    }

    loop {
        // after two sysenters
    }

    return 4;
}
