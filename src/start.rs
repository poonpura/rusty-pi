use core::arch::asm;

extern "C" {
    fn notmain(); 
}

#[no_mangle]
#[link_section = ".text.boot"]
/// Initialize stack pointer 
pub unsafe extern "C" fn _start() -> ! {
    asm!(
        "ldr sp, =0x80000",  
        "bl _start2",     
        options(noreturn)
    );
}

#[no_mangle]
/// Zeroes out bss and calls notmain()
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