/// simple mini-UART driver

use crate::bits::*;
use crate::io::*;
use crate::gpio::*;
use crate::time::*;

const AUX_BASE: u32 = 0x20215000;
const AUX_ENABLES: u32 = AUX_BASE + 0x04;
const AUX_MU_IO: u32 = AUX_BASE + 0x40;
const AUX_MU_IER: u32 = AUX_BASE + 0x44;
const AUX_MU_IIR: u32 = AUX_BASE + 0x48;
const AUX_MU_LCR: u32 = AUX_BASE + 0x4C;
const AUX_MU_CNTL: u32 = AUX_BASE + 0x60;
const AUX_MU_STAT: u32 = AUX_BASE + 0x64;
const AUX_MU_BAUD: u32 = AUX_BASE + 0x68;

/// called first to setup UART to 8n1 115200 baud, no interrupts
pub unsafe fn uart_init() {
    dsb();
    gpio_set_fn(14, 0b010); // set to TXD1
    gpio_set_fn(15, 0b010); // set to RXD1
    dsb();
    put32(AUX_ENABLES, bit_set(get32(AUX_ENABLES), 0)); // enable mini-UART
    dsb();
    put32(AUX_MU_CNTL, 0); // disable TX/RX, control flow
    put32(AUX_MU_LCR, 0b11); // disable DLAB access, set to 8-bit mode
    put32(AUX_MU_IER, 0); // disable interrupts
    put32(AUX_MU_IIR, 0b110); // clear FIFO queues
    put32(AUX_MU_BAUD, 270); // 115200 baud (assuming 250 MHz clock)
    put32(AUX_MU_CNTL, 0b11); // re-enable TX/RX
    dsb();
}

/// disable the UART
pub unsafe fn uart_disable() {
    uart_flush_tx();
    put32(AUX_MU_CNTL, 0); // disable TX/RX
    put32(AUX_MU_IIR, 0b110); // clear FIFO queues
    put32(AUX_ENABLES, bit_clr(get32(AUX_ENABLES), 0)); // disable mini-UART
}

/// Returns one byte from the RX FIFO. If FIFO is empty, blocks until there is 
/// at least one byte.
pub unsafe fn uart_get8() -> u8 {
    while !uart_has_data() { 
        wait();
    }
    get8(AUX_MU_IO)
}

/// Returns one byte from the RX FIFO. If FIFO is empty, returns -1.
pub unsafe fn uart_get8_async() -> Option<u8> {
    if uart_has_data() { 
        Some(get8(AUX_MU_IO)) 
    } else { 
        None 
    }
}

/// Puts one byte on the TX FIFO. If necessary, waits until FIFO has space.
pub unsafe fn uart_put8(x: u8) {
    while !uart_can_put8() {
        wait();
    }
    put8(AUX_MU_IO, x);
}

/// Writes a string to the UART, sending it character by character.
pub unsafe fn uart_write(msg: &str) {
    for byte in msg.bytes() {
        uart_put8(byte);
    }
}

/// Returns whether TX FIFO has room for at least one byte.
pub unsafe fn uart_can_put8() -> bool {
    bit_is_set(get32(AUX_MU_STAT), 1) // space available
}

/// Returns whether there is at least one byte on the RX FIFO
pub unsafe fn uart_has_data() -> bool {
    bit_is_set(get32(AUX_MU_STAT), 0) // symbol available
}

/// Returns whether TX FIFO is empty and idle
pub unsafe fn uart_tx_is_empty() -> bool {
    bit_is_set(get32(AUX_MU_STAT), 9) // transmitter done
}

/// Returns only when TX FIFO is empty and idle.
pub unsafe fn uart_flush_tx() {
    while !uart_tx_is_empty() {
        wait();
    }
}

