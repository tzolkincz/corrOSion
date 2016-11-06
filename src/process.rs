

// process manager



use programs::program1;


use core::mem;

// All program table structure
struct APTEntry {
    start_addr: usize,
    entry_addr: extern "C" fn() -> u8,
    size: usize,
    name: &'static str, // kdyz to bude static, tak nebudeme moci zavadet programy za behu
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
        start_addr: 0,
        entry_addr: program1::iddqd,
        size: 1 << 12,
        name: "program1",
    };

    // crt0 procedure

    // na tuto mi rust hlasi, ze to neni volany v unsafe :/
    // unsafe {
    // static table2: [APTEntry; 1] = mem::uninitialized();
    // }
    //

    program1::iddqd();

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

    crt0(run1);

}


#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn crt0(run: extern "C" fn() -> u8) {

    let mut ret_code: u8;
    unsafe {
        //dispatch kernel
        asm!("//mov r1, rsp;
            //mov r2, rbp;


            mov rdi, rsp; call program entry point
            call rax;

        //kernel_load: ; set kernel back to track

            "
                : "{rax}"(run)  //output registers input registers, program entry point
                : "={rax}" (ret_code) // program return code
                : "rdi" //clobbers - llvm cant use this register
                : "intel" //other options
            );

    }

    //unsafe print ret code
    unsafe {
        let a = ret_code + 48; //48 is offset of numbers in ascii table

        // print on hardcoded VGA location
        let line_colored = [a, 0x4a as u8];
        let buffer_ptr = (0xb8000 + 160 * 8) as *mut _;
        *buffer_ptr = line_colored;
    }

}
