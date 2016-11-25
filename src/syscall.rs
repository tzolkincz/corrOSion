
use process;
use memory;
use scheduler;

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
        1 => {
            handle_alloc(pid);
        }
        2 => {
            handle_terminate(pid);
        }
        3 => {
            handle_pause(pid);
        }
        _ => {
            // process::dispatch_on(pid);
            ..::easy_print_line(1, "Unknown syscall", 0x4f);
            loop {}
        }
    }
}


// allocate memory for process and set return addr to r13 register
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

    // return to program
    process::dispatch_on(pid);
}


#[no_mangle]
#[inline(always)]
pub fn handle_terminate(pid: u32) {
    unsafe {
        ..::easy_print_line(14, "  handle terminate", 0xf3);
        *((0xb8000 + 160 * 14) as *mut _) = [pid as u8 + '0' as u8, 0x1f as u8];
        scheduler::spinkacek();
    }

    process::terminate(pid);
    scheduler::reschedule();
}

#[no_mangle]
#[inline(always)]
pub fn handle_pause(pid: u32) {
    unsafe {
        ..::easy_print_line(15, "  handle pause", 0xf3);
        *((0xb8000 + 160 * 15) as *mut _) = [pid as u8 + '0' as u8, 0x1f as u8];
        scheduler::spinkacek();
    }

    scheduler::reschedule();
}
