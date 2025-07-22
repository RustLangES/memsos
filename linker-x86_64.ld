OUTPUT_FORMAT(elf64-x86-64)

ENTRY(kmain)


PHDRS
{
    null    PT_NULL    FLAGS(0);
    text    PT_LOAD    FLAGS((1 << 0) | (1 << 2));
    rodata  PT_LOAD    FLAGS((1 << 2));
    data    PT_LOAD    FLAGS((1 << 1) | (1 << 2));
}

SECTIONS
{

    . = 0xffffffff80000000;

    .text : {
        *(.text .text.*)
    } :text

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .data : {
        *(.data .data.*)

        KEEP(*(.requests_start_marker))
        KEEP(*(.requests))
        KEEP(*(.requests_end_marker))
    } :data


    .bss : {
	*(COMMON)
        *(.bss .bss.*) 
    } :data

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note .note.*)
    }

    
    . += CONSTANT(MAXPAGESIZE);

    _kernel_end = .;
}
