#[allow(unused_mut)]
pub fn alloc() -> usize {
    let mut addr: usize;
    unsafe {
        set_syscall(1);
        sysenter();
        asm!("nop":"={r13}"(addr):::"intel", "volatile");
    }
    return addr;
}

pub extern "C" fn terminate_process() {
    unsafe {
        //set_syscall(2);
        //sysenter();
        asm!("
            mov r13, 2
            mov r15, rsp    //pass stack pointer to OS
            lea r14, [rip]  //pass instruction pointer to OS
            sysenter
            "::::"intel", "volatile");
    }
}

pub fn yield_process() {
    unsafe {
        set_syscall(3);
        sysenter();
    }
}

pub fn acquire_resource(m_id: u64) {
    unsafe {
        set_syscall(4);
        asm!("nop"::"{r12}"(m_id)::"intel", "volatile");
        sysenter();
    }
}

pub fn release_resource(m_id: u64) {
    unsafe {
        set_syscall(5);
        asm!("nop"::"{r12}"(m_id)::"intel", "volatile");
        sysenter();
    }
}

pub fn putc(ac: (u8, char)) {
    let (attr, ch) = ac;
    let val: u64 = (attr as u64) << 8 | ch as u64;
    unsafe {
        set_syscall(6);
        asm!(""::"{r12}"(val)::"intel", "volatile");
        sysenter();
    }
}

pub unsafe fn sysenter() {
    asm!("
        mov r15, rsp    //pass stack pointer to OS
        lea r14, [rip]  //pass instruction pointer to OS
        sysenter
        "::::"intel", "volatile");
}

unsafe fn set_syscall(s_id: u64) {
    asm!("nop"::"{r13}"(s_id)::"intel", "volatile");
}
