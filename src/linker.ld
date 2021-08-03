/* a linker script for riscv64 */

OUTPUT_ARCH(riscv)
ENTRY(_entry)
BASE_ADDRESS = 0x80000000;

MEMORY {
	sbi_space : ORIGIN = 0x80000000, LENGTH = 128K	/* 0x8000_0000 ~ 0x8001_ffff */
}

PROVIDE(_heap_size = 8K);

SECTIONS
{
	. = BASE_ADDRESS;
	skernel = .;

	stext = .;
	.text : ALIGN(4K) {
		*(.text.init)
		*(.text .text.*)
	} > sbi_space

	etext = .;
	srodata = .;
	.rodata : {
		*(.rodata .rodata.*)
		*(.srodata .srodata.*)
	} > sbi_space

	erodata = .;
	sdata = .;
	.data : {
		*(.data .data.*)
		*(.sdata .sdata.*)
	} > sbi_space

	. = ALIGN(4K);
	edata = .;
	_sbss = .;
	.bss (NOLOAD) : ALIGN(4K) {
		*(.bss .bss.*)
		*(.sbss .sbss.*)
	} > sbi_space

	ekernel = .;

	/* memory for heap */
	. = ALIGN(4K);
	_ebss = .;
	_sheap = .;
	. += _heap_size;
	_eheap = .;

	. = ALIGN(4K);
	_sstack = .;

	/DISCARD/ : {
		*(.eh_frame)
	}
}