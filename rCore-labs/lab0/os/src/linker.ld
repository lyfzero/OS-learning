/* target architecture */
OUTPUT_ARCH(riscv)

/* execution entry */
ENTRY(_start)

/* data storage start address */
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* . indicates the current address (location counter) */
    . = BASE_ADDRESS;

    /* start symbol indicates all starting address */
    kernel_start = .;

    text_start = .;

    /* .text field */
    .text : {
        /* */
        *(.text.entry)
        /* */
        *(.text .text.*)
    }

    rodata_start = .;

    /* .rodata filed */
    .rodata : {
        *(.rodata .rodata.*)
    }

    data_start = .;

    /* .data field */
    .data : {
        /* */
        *(.data .data.*)
    }

    bss_start = .;

    /* .bss field */
    .bss : {
        /* */
        *(.sbss .bss .bss.*)
    }

    /* */
    kernel_end = .;
}