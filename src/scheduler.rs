
use process;


// round robin scheduling
pub fn reschedule() {
    unsafe {
        let start = process::get_process_iterator_start();
        for i in start..(process::MAX_PROCESS_COUNT * 2) {
            if process::PCBS[i % process::MAX_PROCESS_COUNT].eip != 0 {
                process::dispatch_on((i % process::MAX_PROCESS_COUNT) as u32);
            }
        }
    }

    // no process can be planned
    ..::easy_print_line(0, "No process ready...   ...waiting", 0x2f);
    loop {
    }
}


pub unsafe fn spinkacek() {
    for i in 0..1000000 {
        asm!("nop" :::: "intel", "volatile");
    }
}
