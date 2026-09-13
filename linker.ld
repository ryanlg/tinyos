ENTRY(start)

SECTIONS
{
    . = 1M;

    .multiboot : ALIGN(4) { *(.mboot) }

    .text   : ALIGN(16) { *(.text) }
    .rodata : ALIGN(16) { *(.rodata) }
    .data   : ALIGN(16) { *(.data) }
    .bss    : ALIGN(16) { *(.bss) }
}
