/// Implementation for a simple round-robin scheduler and cooperative thread support.

use core::arch::asm;
use crate::tstack::*;

/// Global counter for thread IDs
static mut NEXT_TID: usize = 0;

/// Static memory for threads
static mut THREADS: [Option<Thread>; MAX_THREADS] = [const { None }; MAX_THREADS];

/// Global thread queue for scheduling.
pub static mut SCHEDULER: TQueue = TQueue::init();

/// Default exit routine
pub unsafe extern "C" fn exit() -> ! {
    loop {}  
}

/// Represents a thread in our OS
pub struct Thread {
    pub regs: [u32; 17],  // CPU registers: r0-r12, sp, lr, pc, cpsr
    pub tid: usize,        // Thread ID
    pub sid: usize,   // Index of stack in STACKS
    pub next: Option<&'static mut Thread>, // Next thread in queue
}

/// A simple linked-list-based thread queue
pub struct TQueue {
    pub current_thread: Option<Thread>,
    head: Option<&'static mut Thread>,
}

impl TQueue {
    /// Initializes an empty queue
    pub const fn init() -> Self {
        Self { 
            head: None,
            current_thread: None,
        }
    }

    /// Appends `thread` to queue
    pub unsafe fn push(&mut self, thread: &'static mut Thread) {
        fn insert(
            node: &mut Option<&'static mut Thread>, 
            thread: &'static mut Thread
        ) {
            match node {
                None => *node = Some(thread),
                Some(next) 
                    => insert(&mut next.next, thread),
            }
        }
        insert(&mut self.head, thread);
    } 

    /// Pops the mutable reference to head of the queue and unlinks it from the 
    /// queue. Also takes the thread from `THREAD_QUEUE` and returns an Option
    /// wrapping it. 
    pub unsafe fn pop(&mut self) -> Option<Thread> {
        self.head.take().map(|thread| {
            self.head = thread.next.take();  
            THREADS[thread.sid].take().unwrap()
        })
    }

    /// Creates a new thread that runs `func(args)` and adds it to the queue.
    /// Returns `Some(tid, sid)` or `None` if stack pool is full.
    /// Precondition: `func` returns ().
    pub unsafe fn fork(
        &mut self,
        func: unsafe extern "C" fn(u32),  // Thread entry function
        args: u32                  // Function argument
    ) -> Option<(usize, usize)> {
        // Allocate stack, get tid and sid.
        let (stack, sid) = stack_alloc()?;  
        let tid = NEXT_TID;
        NEXT_TID += 1;

        // Initialize thread and registers + assign to static memory
        let stack_top = stack.as_ptr() as u32 + STACK_SIZE as u32;
        THREADS[sid] = Some(Thread {
            regs: [0; 17],
            tid,
            sid,
            next: None,
        });
        let thread = THREADS[sid].as_mut().unwrap();
        thread.regs[0] = args;  // r0 = function argument
        thread.regs[13] = stack_top;  // sp (r13) = stack top
        thread.regs[14] = func as u32;  // lr (r14) = function entry
        thread.regs[15] = exit as u32;  // pc (r15) = exit code 
        thread.regs[16] = 0x10;  // cpsr = user mode (simplified)

        // Push thread to queue and return tid
        self.push(thread);
        Some((tid, sid))
    }

    /// Appends the current thread to the queue and performs context switching 
    /// into the next thread.
    /// Precondition: `self.current_thread is Some(thread)` where `thread.regs` 
    /// has been updated.
    pub unsafe fn wake(&mut self, regs: [u32; 17]) {
        let sid = self.current_thread
            .as_ref()
            .expect("No next thread!")
            .sid;
        THREADS[sid] = self.current_thread.take();
        let thread = THREADS[sid].as_mut().unwrap();
        thread.regs = regs;
        self.push(thread);
        self.cswitch();
    }

    /// Kills the current thread.
    pub unsafe fn cleanup(&mut self) {
        let sid = self.current_thread
            .as_ref()
            .expect("No next thread!")
            .sid;
        self.current_thread = None;
        stack_free(sid);
        self.cswitch();
    }

    /// Loads the next thread and context switches into it.
    /// Precondition: `self.current_thread is None` but `self.head is Some(thread).
    pub unsafe fn cswitch(&mut self) {
        self.current_thread = self.pop();
        let regs = self.current_thread
            .as_ref()
            .expect("No next thread!")
            .regs;
        load_registers(regs.as_ptr());
    }
}

/// Called by a thread to save register state and yield control to the scheduler.
#[allow(static_mut_refs)]
pub unsafe fn yield_thread() {
    let mut saved_regs = [0u32; 17];
    save_registers(saved_regs.as_mut_ptr());
    saved_regs[13] += 144; // BAD CODE: restore the stack when return to thread 
    switch_to_scheduler();
    SCHEDULER.wake(saved_regs);
}

/// Called by a thread upon termination, signalling the scheduler to destroy TCB.
#[allow(static_mut_refs)]
pub unsafe fn exit_thread() {
    switch_to_scheduler();
    SCHEDULER.cleanup();
}

/// Saves all register values to `reg`, but r0-r3 may be clobbered.
#[inline(always)]
pub unsafe extern "C" fn save_registers(regs: *mut u32) {
    asm!(
        "mrs r1, cpsr",                   
        "str r1, [r0, #64]",  
        "stm r0, {{r0-r12, sp, lr}}",                                         
        in("r0") regs,                      
        out("r1") _,                        
        options(nostack, preserves_flags)
    );
}

/// Loads all register values from `reg`, but r0-r3 may be clobbered. As pc is
/// also loaded, this function also completes the context switch.
#[inline(always)]
pub unsafe extern "C" fn load_registers(regs: *const u32) {
    asm!( 
        "ldr r1, [r0, #64]",        
        "msr cpsr, r1",     
        "mcr p15, 0, r1, c7, c5, 4", // prefetch flush       
        "ldm r0, {{r0-r12, sp, lr}}",         
        "bx lr",               
        in("r0") regs,                                           
        options(noreturn, nostack)
    );
}

/// Switches to supervisor mode and disables IRQ. 
/// Precondition: CPSR has been saved. 
#[inline(always)]
pub unsafe extern "C" fn switch_to_scheduler() {
    asm!(
        "cps #0x13", // 0b10011 - switch to svc              
        "mcr p15, 0, r3, c7, c5, 4", // prefetch flush
        "cpsid i", 
        "mcr p15, 0, r3, c7, c5, 4", // prefetch flush
        out("r3") _,                                                        
        options(nostack) 
    );
}