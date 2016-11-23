
use core::mem;

pub static mut IDTR: IdtDescriptor = IdtDescriptor {
    size: 0,
    offset: 0
};

pub static mut IDT: [IdtEntry; 256] = [IdtEntry {
    offsetl: 0,
    selector: 0,
    zero: 0,
    attribute: 0,
    offsetm: 0,
    offseth: 0,
    zero2: 0
}; 256];

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct IdtDescriptor {
    size: u16,
    offset: u64
}

impl IdtDescriptor {
    pub fn set_slice(&mut self, slice: &'static [IdtEntry]) {
        self.size = (slice.len() * mem::size_of::<IdtEntry>() - 1) as u16;
        self.offset = slice.as_ptr() as u64;
    }

    pub unsafe fn load(&self) {
        asm!("lidt [rax]" : : "{rax}"(self as *const _ as usize) : : "intel", "volatile");
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct IdtEntry {
    offsetl: u16,
    selector: u16,
    zero: u8,
    attribute: u8,
    offsetm: u16,
    offseth: u32,
    zero2: u32
}

impl IdtEntry {
    pub fn set_flags(&mut self, flags: u8) {
        self.attribute = flags;
    }

    pub fn set_offset(&mut self, selector: u16, base: usize) {
        self.selector = selector;
        self.offsetl = base as u16;
        self.offsetm = (base >> 16) as u16;
        self.offseth = (base >> 32) as u32;
    }

    pub fn set_func(&mut self, selector: u16, func: unsafe extern fn()) {
        self.set_offset(selector, func as usize);
    }
}
