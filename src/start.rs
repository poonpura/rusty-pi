use core::arch::asm;

extern "C" {
    fn notmain(); 
}

/// Enter super mode, disable interrupts, and initialize stack pointer 
#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern "C" fn _start() -> ! {
    asm!(
        "mov r0, #0x13", // 0b10011 - switch to svc      
        "orr r0, r0, #(1 << 7)", // disable IRQ
        "msr cpsr, r0",
        "mcr p15, 0, r1, c7, c5, 4", // prefetch flush
        "ldr sp, =0x80000",  
        "bl _start2",     
        options(noreturn)
    );
}

/// Zeroes out bss and calls notmain()
#[no_mangle]
pub unsafe extern "C" fn _start2() -> ! {
    let bss_start: *mut u32;
    let bss_end: *mut u32;
    asm!(
        "ldr {0}, =__symbol_bss_start__",
        "ldr {1}, =__symbol_bss_end__",
        out(reg) bss_start,
        out(reg) bss_end
    );

    let mut ptr = bss_start;
    while ptr < bss_end {
        unsafe {
            ptr.write_volatile(0);
            ptr = ptr.add(1);
        }
    }

    unsafe {
        notmain();
    }
    loop {} 
}