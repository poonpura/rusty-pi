## rusty-pi kernel

This is a Rust implementation for a minimal kernel for Raspberry Pi Zero. It has
support for GPIO, UART, cooperative round-robin scheduling, and GPIO interrupt
handling. In particular, the scheduler has support for fork(), yield() and exit().
Furthermore, we have a built-in stack memory allocation/deallocation system that
supports up to 8 threads at once. 

As a simple proof-of-concept, this codebase contains the following built-in threads:
1. Toggles GPIO 25 on and off every second.
2. Turns GPIO 20 on, and prints the NieR: Automata loading screen message on the UART at a rate of 1B/s, then turns GPIO 20 off when done.
3. An interrupt handler triggered by GPIO 21 input that freezes all current threads, turns on GPIO 5, and echoes back all keystrokes via UART. Keys 'g' and 'r' also toggle GPIO 20 and 25 respectively. Removing GPIO 21 input yields control back to the user threads.
4. While the interrupt handler is active, if a numeric key is pressed, a thread which prints a smiley face i times every second before exiting (where i is the numeric key pressed) is spawned and queued.

This codebase is based on the C implementation found in [CS 140E Winter 2025](https://github.com/dddrrreee/cs140e-25win/tree/main).
