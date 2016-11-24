use core::mem;
use core::ptr;

const PAGE_SIZE: u64 = 0x200000; //2MB
const KERNEL_END_ADDR: u64 = 0x4000000; //64MB
static mut CURRENT_ALLOC_PAGE: u64 = 0;

fn allocate_page() -> u64 {
    unsafe {
        let addr = CURRENT_ALLOC_PAGE * PAGE_SIZE + KERNEL_END_ADDR;
        CURRENT_ALLOC_PAGE += 1;
        return addr;
    }
}

pub fn get_program_code_fa(pid: u32) -> u64 {
    // program starts on 66MB linear address
    return 33 * PAGE_SIZE;
}


unsafe fn get_current_page_table_addr() -> u64 {
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
    let mpl4_mask: u64 = 0b10000111; //(level 3,4 cant be huge), present, CPL3 accessible
    let mpl3_mask: u64 = 0b10000111; //Intel dev manual 3A Table 4-17
    let kernel_mask: u64 = 0b10000001; //huge, present, NOT CPL3 accessible
    let allocated_mask: u64 = 0b10000111; //huge, present, CPL3 accessible
    //let not_present_mask: u64 = 0b1000000; //huge, NOT present, NOT CPL3 accessible
    let not_present_mask: u64 = 0b1000001; //huge, NOT present, NOT CPL3 accessible

    unsafe {
        let start_addr = get_program_dir_table_addr(pid);


        // set first entry of mpl4 to point to page table
        ptr::write(start_addr as *mut u64, (start_addr + 4096) | mpl4_mask);
        ptr::write((start_addr + 4096) as *mut u64, (start_addr + 2*4096) | 0b10000111);




        // map first 64MB indentically (kernel space)
        for i in 0..32 {
            let offset = (i * mem::size_of::<u64>()) as u64;
            ptr::write((2*4096 + start_addr + offset) as *mut u64,
                       (0x200000 * i) as u64 | 0b10000111);
        }


        // alloc two pages (stack and program init code)
        for i in 32..34 {
            let offset = (i * mem::size_of::<u64>()) as u64;
            ptr::write((2*4096 + start_addr + offset) as *mut u64,
                       allocate_page() | 0b10000111);
        }
/*
        // map rest pages as not present
        for i in (1536 + 33 + 3)..2048 {
            let offset = (i * mem::size_of::<u64>()) as u64;
            ptr::write((start_addr + offset) as *mut u64, not_present_mask);
        }
        */

        return start_addr;
    }
}
