#![allow(dead_code)]
/// Code for handling of GPIO (and hopefully UART RX) interrupts.

use core::arch::{asm, global_asm};
use crate::io::*;
use crate::bits::*;
use crate::uart::*;
use crate::gpio::*;
use crate::user::*;

const IRQ_BASE: u32 = 0x2000B200;
const IRQ_BASIC_PENDING: u32 = IRQ_BASE;
const IRQ_PENDING_1: u32 = IRQ_BASE + 0x04;
const IRQ_PENDING_2: u32 = IRQ_BASE + 0x08;
const IRQ_FIQ_CONTROL: u32 = IRQ_BASE + 0x0C;
const IRQ_ENABLE_1: u32 = IRQ_BASE + 0x10;
const IRQ_ENABLE_2: u32 = IRQ_BASE + 0x14;
const IRQ_ENABLE_BASIC: u32 = IRQ_BASE + 0x18;
const IRQ_DISABLE_1: u32 = IRQ_BASE + 0x1C;
const IRQ_DISABLE_2: u32 = IRQ_BASE + 0x20;
const IRQ_DISABLE_BASIC: u32 = IRQ_BASE + 0x24;

pub const GPIO_BASE: u32 = 0x20200000;
pub const GPREN0: u32 = GPIO_BASE + 0x4C;
pub const GPREN1: u32 = GPIO_BASE + 0x50;
pub const GPFEN0: u32 = GPIO_BASE + 0x58;
pub const GPFEN1: u32 = GPIO_BASE + 0x5C;
pub const GPEDS0: u32 = GPIO_BASE + 0x40;

// UART code that is not working yet 
const AUX_BASE: u32 = 0x20215000;
const AUX_MU_IER: u32 = AUX_BASE + 0x44;
const AUX_MU_IIR: u32 = AUX_BASE + 0x48;

const AUX_IRQ: u8 = 29;

extern "C" {
    static _interrupt_table: u32;  // Declare the external symbol
}

global_asm!(
    r#"
    .section .interrupt_table, "ax"
    .global _interrupt_table
    _interrupt_table:
        .align 8
        ldr pc, =default_handler
        ldr pc, =default_handler
        ldr pc, =default_handler
        ldr pc, =default_handler
        ldr pc, =default_handler
        ldr pc, =default_handler
        ldr pc, =interrupt_handler
        ldr pc, =default_handler
    "#
);

/// Enables IRQ
/// Precondition: caller is in privileged mode 
#[no_mangle]
pub unsafe extern "C" fn enable_interrupts() {
    asm!(
        "mrs r0, cpsr",                   
        "bic r0, r0, #(1 << 7)", 
        "msr cpsr_c, r0",         
        options(nostack, preserves_flags)
    );
}

/// Disables IRQ
/// Precondition: caller is in privileged mode 
#[no_mangle]
pub unsafe extern "C" fn disable_interrupts() {
    asm!(
        "mrs r0, cpsr",                   
        "orr r0, r0, #(1 << 7)", 
        "msr cpsr_c, r0",         
        options(nostack, preserves_flags)
    );
}

/// Interrupt handler code for IRQ interrupts 
#[no_mangle]
pub unsafe extern "C" fn interrupt_handler() {
    asm!(
        "ldr sp, =0x90000", 
        "sub lr, lr, #4",
        "push {{r0-r12, lr}}",
        "mov r0, lr",
        "bl interrupt_vector",
        "pop {{r0-r12, lr}}",
        "movs pc, lr",         
        options(nostack, preserves_flags)
    );
}

/// Default handler for other interrupt types.
#[no_mangle]
pub unsafe extern "C" fn default_handler() {
    panic!("Unhandled exception!");
}

/// Initializes (but does not enable) interrupts.
/// Precondition: caller is in privileged mode 
pub unsafe fn interrupt_init() {
    disable_interrupts();
    put32(IRQ_DISABLE_1, 0xFFFFFFFF);
    put32(IRQ_DISABLE_2, 0xFFFFFFFF);

    let vector_base = &_interrupt_table as *const u32 as u32;
    dsb(); 
    vector_base_set(vector_base);
    dsb(); 
}

/// Enables rising edge detection interrupt on `pin`.
/// Precondition: `pin < 32`
pub unsafe fn gpio_rising_edge_init(pin: u8) {
    if pin >= 32 {
        return;
    }
    dsb();
    put32(GPREN0, bit_set(get32(GPREN0), pin));
    dsb();
    put32(IRQ_ENABLE_2, bit_set(0, 17));
    dsb();
}

/// Returns whether GPIO event is detected at `pin`.
/// Precondition: `pin < 32`
pub unsafe fn gpio_event_detected(pin: u8) -> bool {
    if pin >= 32 {
        false
    } else {
        dsb(); 
        let b = bit_is_set(get32(GPEDS0), pin);
        dsb();
        b
    }
}

/// Clears the GPIO interrupt pending bit at `pin`.
/// Precondition: `pin < 32`
pub unsafe fn gpio_event_clear(pin: u8) {
    if pin >= 32 {
        return;
    }
    dsb();
    put32(GPEDS0, bit_set(0, pin));
    dsb();
}

/// Interrupt vector for IRQ interrupts. 
#[no_mangle]
pub unsafe extern "C" fn interrupt_vector() {
    gpio_set_on(5);
    dsb();

    if !gpio_event_detected(21) {
        return;
    }

    // Custom code that controls mini-UART and GPIO output using keystrokes.
    while gpio_read(21) == 1 {
        while uart_has_data() {
            let c = uart_get8() as char;
            uart_put8(c as u8);
            match c {
                'g' => gpio_toggle(20),
                'r' => gpio_toggle(25),
                i if i.is_numeric() => spawn(i),
                _ => continue
            };
        }
        dsb();
    }

    // Exit code 
    gpio_event_clear(21);
    gpio_set_off(5);
}

/// returns the current value vector base is set to.
unsafe fn vector_base_get() -> u32 {
    let v;
    asm!(
        "mrc p15, 0, {0}, c12, c0, 0",  // Read Vector Base Address Register
        out(reg) v,                  // Directly return as an expression
        options(nostack, preserves_flags)
    );
    v
}

/// returns whether `v` is a valid vector base (not null and alignment is good)
unsafe fn vector_base_check(v: u32) -> bool {
    (v != 0) && (v % (1 << 4) == 0)
}

/// set vector base
/// Precondition: must not have been set already.
unsafe fn vector_base_set(v: u32) {
    if !vector_base_check(v) {
        panic!("Illegal vector base!");
    }

    let v_ = vector_base_get();
    if v == v_ {
        return;
    }
    if v_ != 0 {
        panic!("vector base register already set!");
    }

    asm!(
        "mcr p15, 0, {0}, c12, c0, 0", 
        "mcr p15, 0, r3, c7, c10, 4", // dsb
        "mcr p15, 0, r3, c7, c5, 4", // prefetch flush
        in(reg) v, 
        out("r3") _,                 
        options(nostack, preserves_flags)
    );
    
    if v != vector_base_get() {
        panic!("vector base set failed!");
    }
}

// NOT WORKING! TODO 
/// Initializes mini-UART RX IRQ and clears mini-UART RX FIFO.
/// Precondition: `uart_init()` has been called.
pub unsafe fn rx_irq_init() {
    dsb();
    put32(IRQ_ENABLE_1, bit_set(0, AUX_IRQ));
    dsb();
    put32(AUX_MU_IIR, 0b10);
    put32(AUX_MU_IER, 0b10);
}

