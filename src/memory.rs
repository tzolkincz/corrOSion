use core::mem;
use core::ptr;
use process;

const PAGE_SIZE: u64 = 0x200000; //2MB
const KERNEL_END_ADDR: u64 = 0x4000000; //64MB
const KERNEL_PAGES_COUNT: u64 = 32;
static mut CURRENT_ALLOC_PAGE: u64 = 0;

const MPL4_MASK: u64 = 0b00000111; //(level 4 cant be huge), present, CPL3 accessible
const MPL3_MASK: u64 = 0b00000111; //Intel dev manual 3A Table 4-17
const KERNEL_MASK: u64 = 0b10000011; //huge, present, NOT CPL3 accessible
const ALLOCATED_MASK: u64 = 0b10000111; //huge, present, CPL3 accessible
const NOT_PRESENT_MASK: u64 = 0b1000001; //huge, present but NOT CPL3 accessible


fn allocate_page() -> u64 {
    unsafe {
        let addr = CURRENT_ALLOC_PAGE * PAGE_SIZE + KERNEL_END_ADDR;
        CURRENT_ALLOC_PAGE += 1;
        return addr;
    }
}

pub fn alloc(pid: u32) -> u64 {
    let addr = allocate_page();
    unsafe {
        let super_table = get_program_dir_table_addr(pid);

        process::PCBS[pid as usize].last_alloc_page += 1;
        let allocated_no = process::PCBS[pid as usize].last_alloc_page;

        let offset = (KERNEL_PAGES_COUNT + allocated_no) * mem::size_of::<u64>() as u64;
        ptr::write((2 * 4096 + super_table + offset) as *mut u64,
                   allocate_page() | ALLOCATED_MASK);

        return KERNEL_END_ADDR + PAGE_SIZE * allocated_no;
    }
}

pub fn get_program_code_va() -> u64 {
    // program starts on 66MB linear address
    return (KERNEL_PAGES_COUNT + 1) * PAGE_SIZE;
}


pub unsafe fn get_current_page_table_addr() -> u64 {
    let addr: u64;
    asm!("
        mov r15, cr3
        ":"={r15}" (addr):::"intel", "volatile");
    return addr;
}


pub unsafe fn get_program_dir_table_addr(pid: u32) -> u64 {
    // 3 * 4096 is kernel identity page mapping
    let a = get_current_page_table_addr();
    return a + (3 + 3 * pid as u64) * 4096;
}

pub fn init_super_dir_table(pid: u32) -> u64 {
    unsafe {
        let start_addr = get_program_dir_table_addr(pid);

        // set first entry of mpl4 to point to page table
        ptr::write(start_addr as *mut u64, (start_addr + 4096) | MPL4_MASK);
        ptr::write((start_addr + 4096) as *mut u64,
                   (start_addr + 2 * 4096) | MPL3_MASK);


        // map first 64MB indentically (kernel space)
        for i in 0..32 {
            let offset = (i * mem::size_of::<u64>()) as u64;
            ptr::write((2 * 4096 + start_addr + offset) as *mut u64,
                       (0x200000 * i) as u64 | KERNEL_MASK);
        }


        let mut code_addr: u64 = 0;
        // alloc two pages (stack and program init code)
        for i in 32..34 {
            code_addr = allocate_page();
            let offset = (i * mem::size_of::<u64>()) as u64;
            ptr::write((2 * 4096 + start_addr + offset) as *mut u64,
                       code_addr | ALLOCATED_MASK);
        }

        // set program start Physical address
        process::PCBS[pid as usize].code_physical_addr = code_addr;

        // map rest pages as not present
        for i in 34..512 {
            let offset = (i * mem::size_of::<u64>()) as u64;
            ptr::write((2 * 4096 + start_addr + offset) as *mut u64,
                       NOT_PRESENT_MASK);
        }

        return start_addr;
    }
}
