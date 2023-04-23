SECTIONS {
    . = 0x1000;

    .text :  {
        *(.text*)
    }

    .data : {
        *(.data*)
    }

    .rodata : {
        *(.rodata*)
    }

    .bss : {
        *(.bss*)
    }
}