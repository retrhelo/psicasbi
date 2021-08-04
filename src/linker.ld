/* a linker script for riscv64 */

OUTPUT_ARCH(riscv)
ENTRY(_entry)
BASE_ADDRESS = 0x80000000;

MEMORY {
	sbi_space : ORIGIN = 0x80000000, LENGTH = 128K	/* 0x8000_0000 ~ 0x8001_ffff */
}

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

	. = ALIGN(4);
	erodata = .;
	_sidata = LOADADDR(.data);
	_sdata = .;
	.data : ALIGN(4) {
		*(.data .data.*)
		*(.sdata .sdata.*)
	} > sbi_space

	. = ALIGN(4K);
	_edata = .;
	_sbss = .;
	.bss : ALIGN(4K) {
		*(.bss .bss.*)
		*(.sbss .sbss.*)
	} > sbi_space

	. = ALIGN(4K);
	ekernel = .;
	_ebss = .;

	/DISCARD/ : {
		*(.eh_frame)
	}
}