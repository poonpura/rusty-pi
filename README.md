## rusty-pi kernel

This is a Rust implementation for a minimal kernel for Raspberry Pi Zero. It has
support for GPIO, UART, cooperative round-robin scheduling, and GPIO interrupt
handling. In particular, the scheduler has support for fork(), yield() and exit().
Furthermore, we have a built-in stack memory allocation/deallocation system that
supports up to 8 threads at once. 

This codebase is based on the C implementation found in [CS 140E Winter 2025](https://github.com/dddrrreee/cs140e-25win/tree/main).
