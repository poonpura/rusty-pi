use crate::bits::*;
use crate::io::*;

const GPIO_BASE: u32 = 0x20200000;
const GPIO_FSEL0: u32 = GPIO_BASE;
const GPIO_SET0: u32 = GPIO_BASE + 0x1C;
const GPIO_SET1: u32 = GPIO_BASE + 0x20; 
const GPIO_CLR0: u32 = GPIO_BASE + 0x28;
const GPIO_CLR1: u32 = GPIO_BASE + 0x2C;
const GPIO_LEV0: u32 = GPIO_BASE + 0x34;
const GPIO_LEV1: u32 = GPIO_BASE + 0x38;

/// Sets `pin` to function via GPIO_FSEL to mode `y`
/// Precondition: `y` is a 3-bit number
pub unsafe fn gpio_set_fn(pin: u8, y: u8) {
    if pin >= 32 && pin != 47 { return; }
    let addr = GPIO_FSEL0 + 4 * ((pin as u32) / 10);
    let shift = 3 * (pin % 10);
    let val = bits_modify(get32(addr), shift + 2, shift, y as u32);
    put32(addr, val);
}

/// Sets `pin` to be an output pin.
pub unsafe fn gpio_set_output(pin: u8) {
    gpio_set_fn(pin, 1);
}

/// Sets `pin` to be an input pin.
pub unsafe fn gpio_set_input(pin: u8) {
    gpio_set_fn(pin, 0);
}

/// Sets `pin` on.
/// Precondition: `pin` is output
pub unsafe fn gpio_set_on(pin: u8) {
    if pin >= 32 && pin != 47 { return; }
    let (addr, val) = match pin {
        32.. => (GPIO_SET1, bit_set(0, pin - 32)),  
        _    => (GPIO_SET0, bit_set(0, pin)),   
    }; 
    put32(addr, val);   
}

/// Sets `pin` off.
/// Precondition: `pin` is output
pub unsafe fn gpio_set_off(pin: u8) {
    if pin >= 32 && pin != 47 { return; }
    let (addr, val) = match pin {
        32.. => (GPIO_CLR1, bit_set(0, pin - 32)),  
        _    => (GPIO_CLR0, bit_set(0, pin)),   
    }; 
    put32(addr, val);   
}

/// Sets `pin` to `b` 
/// Precondition: `pin` is output and `b` is 0 or 1
pub unsafe fn gpio_write(pin: u8, b: u8) {
    match b {
        1 => gpio_set_on(pin),
        0 => gpio_set_off(pin),
        _ => panic!("invalid bit!"),
    }
}

/// Returns the value of `pin`
/// Precondition: `pin` is input
pub unsafe fn gpio_read(pin: u8) -> u8 {
    if pin >= 32 && pin != 47 { panic!("Read from invalid pin!"); }
    match pin {
        32.. => bit_is_set(get32(GPIO_LEV1), pin - 32) as u8,
        _    => bit_is_set(get32(GPIO_LEV0), pin) as u8,
    }
}