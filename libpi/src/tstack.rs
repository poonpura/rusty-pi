/// Mini stack memory allocator for 8-thread kernel.

pub const MAX_THREADS: usize = 8;
pub const STACK_SIZE: usize = 16 * 1024;  // 16 KB per thread

/// Preallocated stack pool
#[link_section = ".tstack"]
static mut STACKS: [[u8; STACK_SIZE]; MAX_THREADS] = [[0; STACK_SIZE]; MAX_THREADS];

/// Stack availability tracker (`true` = free, `false` = in use)
static mut STACK_FREE: [bool; MAX_THREADS] = [true; MAX_THREADS];

/// Returns a pointer to the lowest available stack and its index in `STACKS`, 
/// or `None` if full.
pub unsafe fn stack_alloc() -> Option<(&'static mut [u8], usize)> {
    for i in 0..MAX_THREADS {
        if STACK_FREE[i] {
            STACK_FREE[i] = false;
            return Some((&mut STACKS[i], i));
        }
    }
    None
}

/// Frees a stack, making it available again
/// Precondition: `i` is a valid index in `STACKS` and points to a not-free stack
pub unsafe fn stack_free(i: usize) {
    assert!(i < MAX_THREADS, "Invalid index!");
    assert!(!STACK_FREE[i], "Double-free!");
    STACK_FREE[i] = true;
}
