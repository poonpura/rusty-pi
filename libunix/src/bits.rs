#![no_std]

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

// TODO: move to separate tests file

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_set() {
        assert_eq!(bit_set(0b0000, 1), 0b0010);
        assert_eq!(bit_set(0b1010, 2), 0b1110);
    }

    #[test]
    fn test_bit_clr() {
        assert_eq!(bit_clr(0b1111, 1), 0b1101);
        assert_eq!(bit_clr(0b1010, 2), 0b1010); // Already cleared
    }

    #[test]
    fn test_bit_flip() {
        assert_eq!(bit_flip(0b1001, 3), 0b0001);
        assert_eq!(bit_flip(0b0000, 0), 0b0001);
    }

    #[test]
    fn test_bit_is_set() {
        assert!(bit_is_set(0b1010, 3)); // bit 3 is set
        assert!(!bit_is_set(0b1010, 2)); // bit 2 is not set
    }

    #[test]
    fn test_bit_is_clr() {
        assert!(bit_is_clr(0b1010, 2)); // bit 2 is cleared
        assert!(!bit_is_clr(0b1010, 3)); // bit 3 is set
    }

    #[test]
    fn test_bits_modify() {
        assert_eq!(bits_modify(0b00000000, 4, 2, 0b101), 0b00010100);
        assert_eq!(bits_modify(0b11111111, 3, 1, 0b001), 0b11110011);
    }

    #[test]
    fn test_bits_eq() {
        assert!(bits_eq(0b10110011, 4, 2, 0b100));
        assert!(!bits_eq(0b10110011, 4, 2, 0b111));
    }
}