

// process manager



use programs::program1;


use core::mem;

// All program table structure
struct APTEntry {
    start_addr: usize,
    entry_addr: extern "C" fn() -> u8,
    size: usize,
    name: &'static str, // kdyz to bude const, tak nebudeme moci zavadet programy za behu
}


struct PCB {
    pid: i32,
    // registers
    ebp: u64,
    esp: u64,
    last_alloc_page: u64, // jak adresovat stranky?
}

struct KernelBlock {
    ebp: u64, // esp: u64, //not needed - stays in esp0 register
}




pub fn load_apt() {
    let pr1 = APTEntry {
        entry_addr: program1::run as extern "C" fn() -> u8,
        size: 1 << 12,
        name: "program1",
    };

    // crt0 procedure

    // na tuto mi rust hlasi, ze to neni volany v unsafe :/
    // unsafe {
    // static table2: [APTEntry; 1] = mem::uninitialized();
    // }
    //

    program1::run();

    // jak sakra deklarovat pole ve statickym scopu bez inicializace
    let table: [APTEntry; 1] = [pr1];



    let run1 = table[0].entry_addr;



    ..::easy_print_line(5, "program 1: returned value:", 0x8e);
    unsafe {



        let ret_code = run1();
        let a = ret_code + 48; //48 is offset of numbers in ascii table

        // print on hardcoded VGA location
        let line_colored = [a, 0x4a as u8];
        let buffer_ptr = (0xb8000 + 160 * 6) as *mut _;
        *buffer_ptr = line_colored;
    }

}


#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn crt0() {

    unsafe {
        asm!("NOP");
    }
}
