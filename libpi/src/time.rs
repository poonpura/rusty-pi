use crate::io::*;

const SYSTEM_TIMER: u32 = 0x20003004;

/// Delays execution for `ms` milliseconds using the system timer.
pub unsafe fn delay_ms(ms: u32) {
    let start = get32(SYSTEM_TIMER);
    while get32(SYSTEM_TIMER) - start < (ms * 1000) {}
}