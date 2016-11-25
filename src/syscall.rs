
use process;
use memory;

#[no_mangle]
#[inline(always)]
pub fn handle_syscall(pid: u32) {


    let syscall_number: u64;
    unsafe {
        asm!("nop"
            :"={r13}"(syscall_number):
            :: "intel", "volatile"
        );
    }

    match syscall_number {
        1 => { handle_alloc(pid);}
        _ => { process::dispatch_on(pid);}
    }
}


//allocate memory for process and set return addr to r13 register
#[no_mangle]
#[inline(always)]
pub fn handle_alloc(pid: u32) {

    let addr = memory::alloc(pid);
    unsafe {
        asm!("nop"
            ::"{r13}"(addr):
            : "intel", "volatile"
        );
    }

    //return to program
    process::dispatch_on(pid);
}
