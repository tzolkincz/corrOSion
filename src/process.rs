
// process manager

use programs::program1;
use memory;

use core::mem;
use core::ptr;

// All program table structure
struct APTEntry {
    pid: u32,
    start_addr: usize,
    entry_addr: extern "C" fn() -> u8,
    size: usize,
    name: [char; 10],
}


struct PCB {
    pid: u32,
    // registers
    ebp: u64,
    esp: u64,
    eip: u64,
    last_alloc_page: u64, // page 0 is invalid, page 1 is stack, page 2 is program code
    page_table_addr: u64,
}

struct KernelBlock {
    ebp: u64, // esp: u64, //not needed - stays in esp0 register
}

//@TODO pořešit to líp
// have to be initialized on compile time
static APT: [APTEntry; 3] = [APTEntry {
                                 pid: 0,
                                 start_addr: 0,
                                 entry_addr: program1::main,
                                 size: 0,
                                 name: ['0'; 10],
                             },
                             APTEntry {
                                 pid: 0,
                                 start_addr: 0,
                                 entry_addr: program1::main,
                                 size: 0,
                                 name: ['0'; 10],
                             },
                             APTEntry {
                                 pid: 0,
                                 start_addr: 0,
                                 entry_addr: program1::main,
                                 size: 0,
                                 name: ['0'; 10],
                             }];

static PCBs: [PCB; 3] = [PCB {
                             pid: 0,
                             ebp: 2 * 0x200000,
                             esp: 2 * 0x200000,
                             eip: 2 * 0x200000,
                             page_table_addr: 10,
                             last_alloc_page: 2,
                         },
                         PCB {
                             pid: 0,
                             ebp: 2 * 0x200000,
                             esp: 2 * 0x200000,
                             eip: 2 * 0x200000,
                             page_table_addr: 10,
                             last_alloc_page: 2,
                         },
                         PCB {
                             pid: 0,
                             ebp: 2 * 0x200000,
                             esp: 2 * 0x200000,
                             eip: 2 * 0x200000,
                             page_table_addr: 0,
                             last_alloc_page: 2,
                         }];

pub fn load_apt() {
    ..::easy_print_line(2, "load_apt .", 0x4f);
    let pr1 = APTEntry {
        pid: 0,
        start_addr: 0,
        entry_addr: program1::main,
        size: 1 << 12,
        name: ['p', 'r', '0', 'g', 'r', 'a', 'm', '1', ' ', ' '],
    };

    unsafe {
        //    let apt_ref: &mut [APTEntry] = mem::transmute_copy(&APT);
        let apt_ref: &mut APTEntry = mem::transmute_copy(&APT[0]);
        // apt_ref[0] = pr1;
        ptr::copy(&pr1, apt_ref, 1);
    }

    ..::easy_print_line(2, "load_apt !", 0x2f);
}


pub fn create_prcess(id: u32) {
    unsafe {
        let pcbs_ref: &mut [PCB] = mem::transmute_copy(&PCBs);

        let super_dir = memory::init_super_dir_table(id);

        pcbs_ref[id as usize].pid = id;
        pcbs_ref[id as usize].ebp = 33 * 0x200000;
        pcbs_ref[id as usize].esp = 33 * 0x200000;
        pcbs_ref[id as usize].eip = 33 * 0x200000;
        pcbs_ref[id as usize].page_table_addr = super_dir;
        pcbs_ref[id as usize].last_alloc_page = 2;
    }

    // move program code to memory accessible from user space
    unsafe {
        let program_current_addr: *const u64 = mem::transmute_copy(&APT[id as usize].entry_addr);
        let program_destination_addr = memory::get_program_code_fa(id) as *mut u64;
        ptr::copy(program_current_addr, program_destination_addr, 20);
    };
}


pub fn dispatch_process(pid: u32) {
    unsafe {
        // WTF? pt shout equal pt2
        // let pt = PCBs[pid as usize].page_table_addr;
        let pt2 = memory::get_program_dir_table_addr(pid);

        asm!("
            mov cr3, rax //set page table addr   <------- TU TO PADÁ
            sysexit
            "
            :
            :
        // registers input registers, program entry point
             "{rcx}"(PCBs[pid as usize].esp),
             "{rdx}"(PCBs[pid as usize].eip),
             "{rax}"(pt2)
            :
            : "intel", "volatile"
        );
    }
}
