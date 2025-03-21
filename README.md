## rusty-pi kernel

This is a Rust implementation for a minimal kernel for Raspberry Pi Zero. It has
support for GPIO, UART, cooperative round-robin threading, and GPIO interrupt
handling. In particular, the scheduler has support for fork(), yield() and exit().
Furthermore, we have a built-in stack memory allocation/deallocation system that
can support up to 8 threads at once. 