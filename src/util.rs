extern "C" {
    fn write_memory(address: *mut u32, data: u32);
    fn read_memory(address: *mut u32) -> u32;
}

pub type MemoryAddress = *mut u32;

pub const fn mem_addr(addr: *mut u32) -> MemoryAddress {
    addr as MemoryAddress
}

pub unsafe fn write_volatile(address: MemoryAddress, data: u32) {
    write_memory(address, data);
}

pub unsafe fn read_volatile(address: MemoryAddress) -> u32 {
    read_memory(address)
}