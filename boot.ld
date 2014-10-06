MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x00040000
    SRAM (rwx) : ORIGIN = 0x20000000, LENGTH = 0x00008000
}

SECTIONS
{
    .text :
    {
        _text = .;
        KEEP(*(.isr_vector))
	/* Skip past the vector in case not all functions are implemented.
	 * It's got 155 entries total. */
	. = 155 * 4;
        *(.text*)
        *(.rodata*)
        _etext = .;
    } > FLASH

    /* data is stored in flash and will be copied into sram by the
     * reset handler */
    .data : AT(_etext)
    {
        _data = .;
        *(.data*)
        _edata = .;
    } > SRAM

    .bss :
    {
        _bss = .;
        *(.bss*)
        *(COMMON)
        _ebss = .;
    } > SRAM

    PROVIDE(_stack_top = ORIGIN(SRAM) + LENGTH(SRAM) - 4);
}
