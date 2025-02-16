ENTRY(_start)

SECTIONS
{
    . = 0x80000000;  /* Dirección de carga para RISC-V */

    .text : {
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    }
}
