__rpi_phys_dram_start_addr = 0x0;

__rpi_phys_binary_load_addr = 0x80000;

/**ENTRY(__rpi_phys_binary_load_addr) */
ENTRY(__rpi_phys_dram_start_addr)

/**
4 . R
5   RX
6 . RW
*/
PHDRS
{
	segment_boot_core_stack PT_LOAD FLAGS(6);
	segment_code            PT_LOAD FLAGS(5);
	segment_data            PT_LOAD FLAGS(6);
}

SECTIONS
{
	. = __rpi_phys_dram_start_addr;

	.boot_core_stack (NOLOAD) :
	{
		. += __rpi_phys_binary_load_addr;

		__boot_core_stack_end_exclusive = .;
	} :segment_boot_core_stack


	.text :
	{
		KEEP(*(.text._start))
		*(.text._start_arguments)
		*(.text._start_rust)
		*(.text*)
	} :segment_code

	.rodata : ALIGN(8) { *(.rodata*) } :segment_code

	.data : { *(.data*) } :segment_data

	.bss (NOLOAD) : ALIGN(16) {
		__bss_start = .;
		*(.bss*);
		. = ALIGN(16);
		__bss_end_exclusive = .;
	} :segment_data

	.got : { *(.got*) }

	ASSERT(SIZEOF(.got) == 0, "Relocation support not expected")

	/DISCARD/ : { *(.comment*) }
}

