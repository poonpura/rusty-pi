/// Memory-mapped I/O utility functions. Replace staff PUT/GET functions.

use core::arch::asm;

/// Reads a 32-bit value from a memory address
#[inline(always)]
pub unsafe fn get32(addr: u32) -> u32 {
    let ptr = addr as *const u32;
    ptr.read_volatile()
}

/// Writes a 32-bit value to a memory address
#[inline(always)]
pub unsafe fn put32(addr: u32, value: u32) {
    let ptr = addr as *mut u32;
    ptr.write_volatile(value);
}

/// Reads an 8-bit value from a memory address
#[inline(always)]
pub unsafe fn get8(addr: u32) -> u8 {
    let ptr = addr as *const u8;
    ptr.read_volatile()
}

/// Writes an 8-bit value to a memory address
#[inline(always)]
pub unsafe fn put8(addr: u32, value: u8) {
    let ptr = addr as *mut u8;
    ptr.write_volatile(value);
}

/// Data synchronization barrier for MMIO 
#[inline(always)]
pub unsafe fn dsb() {
    asm!(
        "mcr p15, 0, r0, c7, c10, 4", 
        out("r0") _, options(nostack, preserves_flags)
    );
}
