
use process;
use memory;
use scheduler;

const MUTEX_COUNT: usize = 10;

#[derive(Copy, Clone, Debug)]
struct MutexEntry {
    acquired: bool,
    by: u32,
    waiting_count: usize,
    waiting_queue: [u32; process::MAX_PROCESS_COUNT],
}

static mut MUTEX_TABLE: [MutexEntry; MUTEX_COUNT] = [MutexEntry {
    acquired: false,
    by: 0,
    waiting_count: 0,
    waiting_queue: [0; process::MAX_PROCESS_COUNT],
}; MUTEX_COUNT];


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
        4 => {
            handle_acquire(pid);
        }
        5 => {
            handle_release(pid);
        }
        _ => {
            // process::dispatch_on(pid);
            ..::easy_print_line(1, "Unknown syscall", 0x4f);
            loop {}
        }
    }
}


// allocate memory for process and set return addr to r13 register
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


pub fn handle_terminate(pid: u32) {
    unsafe {
        ..::easy_print_line(14, "  handle terminate", 0xf3);
        *((0xb8000 + 160 * 14) as *mut _) = [pid as u8 + '0' as u8, 0x1f as u8];
        scheduler::spinkacek();
    }

    // release all mutexes
    for i in 0..MUTEX_COUNT {
        unsafe {
            if MUTEX_TABLE[i].acquired == true && MUTEX_TABLE[i].by == pid {
                if MUTEX_TABLE[i].waiting_count == 0 {
                    MUTEX_TABLE[i].acquired = false;
                } else {
                    MUTEX_TABLE[i].waiting_count -= 1;
                    let pid_w = MUTEX_TABLE[i].waiting_queue[MUTEX_TABLE[i].waiting_count];
                    MUTEX_TABLE[i].by = pid_w;
                    process::PCBS[pid_w as usize].state = process::ProcessState::Ready;
                }
            }
        }
    }

    process::terminate(pid);
    scheduler::reschedule();
}


pub fn handle_pause(pid: u32) {
    unsafe {
        ..::easy_print_line(15, "  handle pause", 0xf3);
        *((0xb8000 + 160 * 15) as *mut _) = [pid as u8 + '0' as u8, 0x1f as u8];
        scheduler::spinkacek();
    }

    scheduler::reschedule();
}


unsafe fn get_mutex_id() -> usize {
    let mut mutex_id: usize;
    // get mutex id
    asm!("nop"
        :"={r12}"(mutex_id)
        ::: "intel", "volatile"
    );
    return mutex_id;
}


pub fn handle_acquire(pid: u32) {
    unsafe {
        ..::easy_print_line(16, "  handle acquire", 0xf3);
        *((0xb8000 + 160 * 16) as *mut _) = [pid as u8 + '0' as u8, 0x1f as u8];
        scheduler::spinkacek();
    }

    unsafe {
        let mutex_id = get_mutex_id();

        if MUTEX_TABLE[mutex_id].acquired == true && MUTEX_TABLE[mutex_id].by == pid {
            process::dispatch_on(pid);
        } else if MUTEX_TABLE[mutex_id].acquired == false {
            MUTEX_TABLE[mutex_id].acquired = true;
            MUTEX_TABLE[mutex_id].by = pid;
            process::dispatch_on(pid);
        } else {
            MUTEX_TABLE[mutex_id].waiting_queue[MUTEX_TABLE[mutex_id].waiting_count] = pid;
            MUTEX_TABLE[mutex_id].waiting_count += 1;
            process::PCBS[pid as usize].state = process::ProcessState::Blocked;
            scheduler::reschedule();
        }
    }
}

pub fn handle_release(pid: u32) {
    unsafe {
        ..::easy_print_line(16, "  handle release", 0xf3);
        *((0xb8000 + 160 * 16) as *mut _) = [pid as u8 + '0' as u8, 0x1f as u8];
        scheduler::spinkacek();
    }

    unsafe {
        let mutex_id = get_mutex_id();

        if MUTEX_TABLE[mutex_id].waiting_count == 0 {
            MUTEX_TABLE[mutex_id].acquired = false;
            process::dispatch_on(pid);
        } else {
            MUTEX_TABLE[mutex_id].waiting_count -= 1;
            let pid = MUTEX_TABLE[mutex_id].waiting_queue[MUTEX_TABLE[mutex_id].waiting_count];
            MUTEX_TABLE[mutex_id].by = pid;
            process::PCBS[pid as usize].state = process::ProcessState::Ready;
            process::dispatch_on(pid);
        }
    }
}
