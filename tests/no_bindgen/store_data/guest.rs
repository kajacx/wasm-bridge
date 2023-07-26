#[no_mangle]
pub fn add_three_i32(number: i32) -> i32 {
    unsafe { add_one_i32(number.wrapping_add(1)).wrapping_add(1) }
}

#[no_mangle]
pub fn allocate_bytes(amount: u32) -> u32 {
    let bytes = vec![0u8; amount as _];
    let bytes = bytes.into_boxed_slice();
    let addr = Box::leak(bytes);
    addr as *mut [u8] as *mut u8 as usize as u32
}

#[no_mangle]
pub fn increment_bytes_at(offset: u32, len: u32) {
    let mut offset = offset as usize as *mut u8;
    for _ in 0..len {
        // SAFETY: we trust the host to give us "safe" memory space created be "allocate_bytes"
        unsafe {
            let value = std::ptr::read(offset);
            let value = value + 1;
            std::ptr::write(offset, value);
            offset = offset.offset(1);
        }
    }
}

#[link(wasm_import_module = "imported_fns")]
extern "C" {
    fn add_one_i32(number: i32) -> i32;
}
