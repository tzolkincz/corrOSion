

// zatim jen jako fce v kernel space, bez crt0



#[no_mangle]
pub extern "C" fn run() -> u8 {
    return 4;
}
