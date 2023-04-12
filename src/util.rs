extern "C" {
    fn write_memory(address: *mut u32, data: u32);
    fn read_memory(address: *mut u32) -> u32;
}

pub unsafe fn write_volatile(address: *mut u32, data: u32) {
    write_memory(address, data);
}

pub unsafe fn read_volatile(address: *mut u32) -> u32 {
    read_memory(address)
}