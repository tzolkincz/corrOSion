
// process manager

use programs::program1;
use memory;

use core::mem;
use core::ptr;


const NO_PROCESS_RUNNING: u32 = 2 ^ 20;

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

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct PCB {
    pid: u32,
    // registers
    ebp: u64,
    esp: u64,
    eip: u64,
    last_alloc_page: u64, // page 1 is stack, page 2 is program code
    page_table_addr: u64,
}

struct KernelBlock {
    ebp: u64, // esp: u64, //not needed - stays in esp0 register
    current_process: u32,
    page_table_addr: u64,
}

// Kernel Control Block
static mut KCB: KernelBlock = KernelBlock {
    page_table_addr: 0,
    ebp: 0,
    current_process: NO_PROCESS_RUNNING,
};

static mut APT: [APTEntry; 3] = [APTEntry {
    pid: 0,
    start_addr: 0,
    entry_addr: program1::main,
    size: 0,
    name: ['0'; 10],
}; 3];

static mut PCBS: [PCB; 3] = [PCB {
    pid: 0,
    ebp: 0,
    esp: 0,
    eip: 0,
    page_table_addr: 0,
    last_alloc_page: 0,
}; 3];

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
    }

    ..::easy_print_line(2, "load_apt !", 0x2f);
}


pub fn create_prcess(id: u32) {

    unsafe {
        let super_dir = memory::init_super_dir_table(id);

        PCBS[id as usize].pid = id;
        PCBS[id as usize].ebp = memory::get_program_code_fa(id);
        PCBS[id as usize].esp = memory::get_program_code_fa(id);
        PCBS[id as usize].eip = memory::get_program_code_fa(id);
        PCBS[id as usize].page_table_addr = super_dir;
        PCBS[id as usize].last_alloc_page = 1;
    }

    // move program code to memory accessible from user space
    unsafe {
        let program_current_addr: *const u64 = mem::transmute_copy(&APT[id as usize].entry_addr);
        let program_destination_addr = memory::get_program_code_fa(id) as *mut u64;
        // @TODO set correct size dynamically
        ptr::copy(program_current_addr, program_destination_addr, 20);
    };

}


pub fn dispatch_on(pid: u32) {

    unsafe {
        if PCBS[pid as usize].eip == 0 {
            // uninicialized process
            ..::easy_print_line(1, "Cant dispatch uninicialized process", 0x4f);
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
            :: "intel", "volatile"
        );


        if PCBS[pid as usize].esp != PCBS[pid as usize].ebp {

            // ---------------------------------------------------------------------------------
            // these registers state must be handled by program iteslf (are needed for syscalls)
            // rax, rbx, rcx, rdx
            // ---------------------------------------------------------------------------------

            // process has saved registers on stack
            asm!("
                mov esp, ecx

                pop rbp
                pop rsi
                pop rdi
                pop r8
                pop r9
                pop r10
                pop r11
                pop r12
                pop r13
                pop r14
                pop r15

                "
                :
                :
                "{ecx}"(PCBS[pid as usize].esp),
                "{ebp}"(PCBS[pid as usize].ebp)
                :: "intel", "volatile"
            );
        }


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
#[inline(always)]
pub fn dispatch_off() {

    unsafe {
        if KCB.current_process == NO_PROCESS_RUNNING {
            return;
        }

        asm!("
            add r14, 2 //set program instruction pointer to next instruction

            mov rax, rsp //save kernel stack pointer
            mov rsp, r15

            push r15
            push r14
            push r13
            push r12
            push r11
            push r10
            push r9
            push r8
            push rdi
            push rsi
            push rbp

            mov r15, rsp
            mov rsp, rax //set back kernel pointer
            mov cr3, rdx //set kernel page table

            "
            :
            "={r14}"(PCBS[KCB.current_process as usize].eip),
            "={r15}"(PCBS[KCB.current_process as usize].esp)
            :
            "{rdx}"(KCB.page_table_addr)
            :
            "rax", "r14", "r15"
            :
             "intel", "volatile"
        );

        KCB.current_process = NO_PROCESS_RUNNING;
    }

}

pub fn terminate(pid: u32) {
    unsafe {
        PCBS[pid as usize].ebp = 0;
        PCBS[pid as usize].esp = 0;
        PCBS[pid as usize].eip = 0;
        PCBS[pid as usize].page_table_addr = 0;
        PCBS[pid as usize].last_alloc_page = 0;
    }
}
