

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
    ..::easy_print_line(2, "load_apt .", 0x4f);
    let pr1 = APTEntry {
        start_addr: 0,
        entry_addr: program1::main,
        size: 1 << 12,
        name: "program1",
    };

    // crt0 procedure

    // na tuto mi rust hlasi, ze to neni volany v unsafe :/
    // unsafe {
    // static table2: [APTEntry; 1] = mem::uninitialized();
    // }
    //

    // jak sakra deklarovat pole ve statickym scopu bez inicializace
    let table: [APTEntry; 1] = [pr1];



    let run1 = table[0].entry_addr;


    rrt0(run1);

    ..::easy_print_line(2, "load_apt !", 0x2f);
}



#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn rrt0(run: extern "C" fn() -> u8) {
    ..::easy_print_line(3, "rrt0 .", 0x4f);

    let mut ret_code: u8;
    let mut ebp_register: u64;
    unsafe {
        // dispatch kernel
        //
        // asm!("lea ecx, [kernel_entry_point]; nebo zkusit lea instrukci
        // mov [task_state_segment + 0x04], ecx ; prepare TSS for entering ring0
        //
        // ltr [0x18] ; LTR - load task register, sets reference to TSS
        //
        //
        // jump rax; //jump to program entry point
        //
        // kernel_entry_point: ; set kernel back to track
        // "
        // : "={rax}" (ret_code) // output values
        // : "{rax}"(run)  //registers input registers, program entry point
        // : "rdi" //clobbers - llvm cant use this register
        // : "intel" //other options
        // );
        //
        // asm!("
        // ;cli #clear interrupt bit - disable
        // ;lea ecx, [kernel_entry_point]
        // ;mov [task_state_segment + 0x04], ecx
        // ;ltr [0x18]
        //
        // ;sti # enable interrupts
        //
        // ;jmp [rax]
        // ;kernel_entry_point:
        // "
        // : "={rax}" (ret_code) // output values
        // : "{rax}"(run)  //registers input registers, program entry point
        // : "rdi" //clobbers - llvm cant use this register
        // : "intel" //other options
        // );
        //

        // move function code to memory accessible from user space
        use core::ptr;

        unsafe {
            // let program_current_address = &run as *mut i64;
            let program_current_address: *const i64 = mem::transmute_copy(&run);
            ptr::copy(program_current_address, 0x1000000 as *mut i64, 20);
        };
        ..::easy_print_line(4, "program copied", 0x1f);

        asm!("
            mov rcx, 0x800000
            mov rdx, 0x1000000
        //sysexit
        call rdx
        //sysenter

            "
            : "={rax}" (ret_code) // output values
            : "{rdx}"(run) //registers input registers, program entry point
            //on sysexit epi will be set on edx value, hence should be set to program entry point
            : "rdi" //clobbers - llvm cant use this register
            : "intel" //other options
        );

        ret_code = 3;
    }

    unsafe {
        *((0xb8000 + 160 * 5) as *mut _) = [ret_code as u8 + '0' as u8, 0x1f as u8];
    }

    ..::easy_print_line(3, "rrt0 !", 0x2f);
}
