use programs::user;

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn main() -> u8 {
    unsafe {
        /*asm!("
            mov r10, 0x12345 //test register preservation
            "::::"intel", "volatile");

        let addr = user::alloc();
        asm!("
            mov eax, 0x001
            mov [r13], eax //test memory allocation
            "::"{r13}"(addr)::"intel", "volatile");*/

        //user::acquire_resource(0);

        //user::yield_process();

        //user::putc((0x1f, '@'));

        //user::yield_process();

        //user::release_resource(0);

        //user::yield_process();

        /*asm!("
            mov r13, 0x02   //syscall number (2 - terminate)
            "::::"intel", "volatile");
        user::sysenter();*/

        asm!("
            call test // mimicking user::terminate_process() call and here it works
            test:
            push rbp
            mov rsp, rbp

            mov r13, 2
            mov r15, rsp    //pass stack pointer to OS
            lea r14, [rip]  //pass instruction pointer to OS
            sysenter
            "::::"intel", "volatile");

        user::terminate_process();

        asm!("
            hlt //this will be never called
            "::::"intel", "volatile");

    }

    loop {
        // after two sysenters
    }

    return 4;
}
