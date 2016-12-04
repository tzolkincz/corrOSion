
// process manager

use programs::program1;
use programs::program2;
use memory;

use core::mem;
use core::ptr;


const NO_PROCESS_RUNNING: u32 = 10 ^ 5;
pub const MAX_PROCESS_COUNT: usize = 10;


static mut PROCESS_DEBUG_OUTPUT_LINE: i32 = 4;
const DEBUG_OUTPUT: bool = true;

// All program table structure
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct APTEntry {
    pid: u32,
    start_addr: usize,
    entry_addr: extern "C" fn() -> u8,
    size: usize,
    name: [char; 10],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProcessState {
    Created,
    Ready,
    Blocked,
    Uninitialized,
    Running,
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct PCB {
    pid: u32,
    // registers
    ebp: u64,
    esp: u64,
    pub eip: u64,
    pub last_alloc_page: u64, // page 1 is stack, page 2 is program code
    page_table_addr: u64,
    pub code_physical_addr: u64, // Physical address
    pub state: ProcessState,
}

#[allow(dead_code)]
struct KernelBlock {
    ebp: u64, // esp: u64, //not needed - stays in esp0 register
    current_process: u32,
    last_process: u32,
    page_table_addr: u64,
}

// Kernel Control Block
static mut KCB: KernelBlock = KernelBlock {
    page_table_addr: 0,
    ebp: 0,
    current_process: NO_PROCESS_RUNNING,
    last_process: NO_PROCESS_RUNNING,
};

static mut APT: [APTEntry; MAX_PROCESS_COUNT] = [APTEntry {
    pid: 0,
    start_addr: 0,
    entry_addr: program1::main,
    size: 0,
    name: ['0'; 10],
}; MAX_PROCESS_COUNT];

pub static mut PCBS: [PCB; MAX_PROCESS_COUNT] = [PCB {
    pid: 0,
    ebp: 0,
    esp: 0,
    eip: 0,
    page_table_addr: 0,
    last_alloc_page: 0,
    code_physical_addr: 0,
    state: ProcessState::Uninitialized,
}; MAX_PROCESS_COUNT];

pub fn load_apt() {
    ..::easy_print_line(2, "load_apt .", 0x4f);

    unsafe {
        // init first program
        let id: usize = 0;
        APT[id].pid = id as u32;
        APT[id].start_addr = 0;
        APT[id].entry_addr = program1::main;
        APT[id].size = 1 << 12;
        APT[id].name = ['p', 'r', '0', 'g', 'r', 'a', 'm', '1', ' ', ' '];


        // init second program
        let id2: usize = 1;
        APT[id2].pid = id as u32;
        APT[id2].start_addr = 0;
        APT[id2].entry_addr = program2::main2;
        APT[id2].size = 1 << 12;
        APT[id2].name = ['p', 'r', '0', 'g', 'r', 'a', 'm', '2', ' ', ' '];

    }

    ..::easy_print_line(2, "load_apt !", 0x2f);
}


pub fn create_prcess(id: u32) {

    unsafe {
        let super_dir = memory::init_super_dir_table(id);

        PCBS[id as usize].pid = id;
        PCBS[id as usize].ebp = memory::get_program_code_va();
        PCBS[id as usize].esp = memory::get_program_code_va();
        PCBS[id as usize].eip = memory::get_program_code_va();
        PCBS[id as usize].page_table_addr = super_dir;
        PCBS[id as usize].last_alloc_page = 1;
        PCBS[id as usize].state = ProcessState::Created;
    }

    // move program code to memory accessible from user space
    unsafe {
        let program_current_addr: *const u64 = mem::transmute_copy(&APT[id as usize].entry_addr);
        let program_destination_addr = PCBS[id as usize].code_physical_addr as *mut u64;
        // @TODO set correct size dynamically
        ptr::copy(program_current_addr, program_destination_addr, 1000);
    };

}




#[no_mangle]
#[inline(always)]
#[allow(private_no_mangle_fns)]
pub fn dispatch_on(pid: u32) {
    if DEBUG_OUTPUT {
        unsafe {
            ..::easy_print_line(PROCESS_DEBUG_OUTPUT_LINE, "  dispatching.", 0xa1);
            *((0xb8000 + 160 * PROCESS_DEBUG_OUTPUT_LINE) as *mut _) = ['0' as u8 + pid as u8,
                                                                        0xc8 as u8];
            PROCESS_DEBUG_OUTPUT_LINE += 1;
            use scheduler;
            scheduler::spinkacek();
        }
    }

    unsafe {
        if PCBS[pid as usize].state == ProcessState::Uninitialized {
            // uninicialized process
            ..::easy_print_line(1, "Cant dispatch uninicialized process", 0x4f);
            loop {}
        }

        if PCBS[pid as usize].state == ProcessState::Blocked {
            // uninicialized process
            ..::easy_print_line(1, "Cant dispatch blocked process", 0x4f);
            loop {}
        }

        KCB.current_process = pid;
        KCB.page_table_addr = memory::get_current_page_table_addr();

        asm!("
            //save current (kernel) stack
            mov ecx, 0x175 // writes kernel ESP to model specific registers
            mov edx, 0
            mov eax, esp // do not erase kernel stack on interrupt
            wrmsr

            mov cr3, rbx //set page table addr
            "
            ::
            "{rbx}"(PCBS[pid as usize].page_table_addr)
            :"ecx", "edx", "eax": "intel", "volatile"
        );

        if PCBS[pid as usize].state != ProcessState::Created {
            PCBS[pid as usize].state = ProcessState::Running;

            // ---------------------------------------------------------------------------------
            // these registers state must be handled by program iteslf (are needed for syscalls)
            // rax, rbx, rcx, rdx
            // ---------------------------------------------------------------------------------

            // process has saved registers on stack
            asm!("
                mov esp, ecx

                //pop rbp
                pop rsi
                pop rdi
                pop r8
                pop r9
                pop r10
                pop r11
                pop r12
                //pop r13 //not preserved, used for syscalls
                //pop r14
                //pop r15

                sysexit //avoid dirty registers
                " ::
                "{ecx}"(PCBS[pid as usize].esp),
                "{ebp}"(PCBS[pid as usize].ebp),
                "{rdx}"(PCBS[pid as usize].eip)
                :: "intel", "volatile"
            );
        }

        PCBS[pid as usize].state = ProcessState::Running;

        asm!("
            sysexit" ::
        // registers input registers, program entry point
             "{rcx}"(PCBS[pid as usize].esp),
             "{rdx}"(PCBS[pid as usize].eip)
            :: "intel", "volatile"
        );
    }
}

// -------------------------------------
// sysenter needs registers r14 and r15
// r15 = program stack pointer
// r14 = program instruction pointer
// -------------------------------------
#[no_mangle]
#[inline(always)]
#[allow(private_no_mangle_fns)]
pub fn dispatch_off() -> u32 {

    unsafe {
        if KCB.current_process == NO_PROCESS_RUNNING {
            ..::easy_print_line(1, "Cant dispatch off E:No process is running", 0x4f);
            loop {}
        }

        asm!("
            add r14, 2 //set program instruction pointer to next instruction

            mov rcx, rsp //save kernel stack pointer
            mov rsp, r15

            //push r15  //not preserved, used for syscalls
            //push r14
            //push r13
            push r12
            push r11
            push r10
            push r9
            push r8
            push rdi
            push rsi
            //push rbp

            mov r15, rsp
            mov rsp, rcx //set back kernel pointer
            mov cr3, rdx //set kernel page table
            "
            :
            "={r14}"(PCBS[KCB.current_process as usize].eip),
            "={r15}"(PCBS[KCB.current_process as usize].esp)
            :
            "{rdx}"(KCB.page_table_addr)
            :
            "rcx"
            :
             "intel", "volatile"
        );

        let old_pid = KCB.current_process;
        PCBS[old_pid as usize].state = ProcessState::Ready;
        KCB.last_process = old_pid;
        KCB.current_process = NO_PROCESS_RUNNING;
        return old_pid;
    }

}

pub fn terminate(pid: u32) {
    if DEBUG_OUTPUT {
        unsafe {
            ..::easy_print_line(PROCESS_DEBUG_OUTPUT_LINE, "  terminating.", 0x3f);
            *((0xb8000 + 160 * PROCESS_DEBUG_OUTPUT_LINE) as *mut _) = ['0' as u8 + pid as u8,
                                                                        0xc8 as u8];
            PROCESS_DEBUG_OUTPUT_LINE += 1;
            use scheduler;
            scheduler::spinkacek();
        }
    }


    unsafe {
        PCBS[pid as usize].ebp = 0;
        PCBS[pid as usize].esp = 0;
        PCBS[pid as usize].eip = 0;
        PCBS[pid as usize].page_table_addr = 0;
        PCBS[pid as usize].last_alloc_page = 0;

        if pid == KCB.current_process {
            KCB.current_process = NO_PROCESS_RUNNING;
        }
    }
}

pub unsafe fn get_process_iterator_start() -> usize {
    if KCB.last_process == NO_PROCESS_RUNNING {
        return 0;
    }
    return ((KCB.last_process + 1) % MAX_PROCESS_COUNT as u32) as usize;
}
