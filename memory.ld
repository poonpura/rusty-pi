/*
Acknowledgement: Thank you Max for providing me starter code. 

Defined symbols:

__symbol_exec_start__

__symbol_code_start__
__symbol_code_end__
__symbol_rodata_start__
__symbol_rodata_end__
__symbol_data_start__
__symbol_data_end__
__symbol_bss_start__
__symbol_bss_end__

__symbol_exec_end__
 */

SECTIONS {
    ENTRY(_start)
    .text 0x8000 : {
        __symbol_exec_start__ = .;
        __symbol_code_start__ = .;
        KEEP(*(.text.boot))
        *(.text*) 
        __symbol_code_end__ = .;
        . = ALIGN(8);
    }
    .rodata : {
        . = ALIGN(0x1000);
        __symbol_rodata_start__ = .;
        *(.rodata*)
        __symbol_rodata_end__ = .;
    }
    .data : {
        . = ALIGN(0x1000);
        __symbol_data_start__ = .;
        *(.data*)
        . = ALIGN(4);
        __symbol_data_end__ = .;
    }
    /* start.S will zero out every byte between __symbol_bss_start__ and __symbol_bss_end__!!!
     */
    .bss : {
        . = ALIGN(0x1000);
        . = ALIGN(4);
        __symbol_bss_start__ = .;
        *(.bss*)
        /* anything that wasn't explicitly assigned a section */
        *(COMMON)
        . = ALIGN(8);
        __symbol_bss_end__ = .;
    }
    .stack : {
        . = ALIGN(8);
        __symbol_stack_begin__ = .;
        . = . + 0x20000;
        __symbol_stack_end__ = .;
        . = ALIGN(8);
        __symbol_exec_end__ = .;
    }
    .tstack 0xC0000 : {
        . = ALIGN(8);
        __symbol_tstack_start__ = .;
        . = . + 0x20000;
        __symbol_tstack_end__ = .;
    }
    .interrupt_table 0x100000 : {
        . = ALIGN(8);
        __symbol_interrupt_table_start__ = .;
        KEEP(*(.interrupt_table))
        . = ALIGN(8);
        __symbol_interrupt_table_end__ = .;
    }
}