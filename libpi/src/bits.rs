//! Bit manipulation routines. These functions work on 32-bit unsigned integers 
//! and return the result (no side effects).

/// Returns `x` with `x[i] = 1`
/// Precondition: `i < 32`
pub fn bit_set(x: u32, i: u8) -> u32 { 
    assert!(i < 32);
    x | (1 << i) 
}

/// Returns `x` with `x[i] = 0`
/// Precondition: `i < 32`
pub fn bit_clr(x: u32, i: u8) -> u32 { 
    assert!(i < 32);
    x & !(1 << i) 
}

/// Returns `x` with `x[i]` toggled.
/// Precondition: `i < 32`
pub fn bit_flip(x: u32, i: u8) -> u32 { 
    assert!(i < 32);
    x ^ (1 << i) 
}

/// Returns `true` if `x[i] == 1`, otherwise `false`.
/// Precondition: `i < 32`
pub fn bit_is_set(x: u32, i: u8) -> bool { 
    assert!(i < 32);
    (x >> i) & 1 == 1 
}

/// Returns `true` if `x[i] == 0`, otherwise `false`.
/// Precondition: `i < 32`
pub fn bit_is_clr(x: u32, i: u8) -> bool { 
    assert!(i < 32);
    (x >> i) & 1 == 0 
}

/// Returns `x` with `x[ub:lb] = y`
/// Preconditions: `lb <= ub < 32` and `y < (1 << (ub - lb + 1))`
pub fn bits_modify(x: u32, ub: u8, lb: u8, y: u32) -> u32 {
    assert!(lb <= ub && ub < 32);
    assert!(y < (1 << (ub - lb + 1)));
    let mask = (1 << (ub - lb + 1)) - 1;
    x & !(mask << lb) | (y << lb)
}

/// Returns `true` if `x[ub:lb] == y`, otherwise `false`.
/// Preconditions: `lb <= ub < 32` and `y < (1 << (ub - lb + 1))`
pub fn bits_eq(x: u32, ub: u8, lb: u8, y: u32) -> bool {
    assert!(lb <= ub && ub < 32);
    assert!(y < (1 << (ub - lb + 1)));
    let mask = (1 << (ub - lb + 1)) - 1;
    (x >> lb) & mask == y
}