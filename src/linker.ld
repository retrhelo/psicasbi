/* a linker script for riscv64 */

OUTPUT_ARCH(riscv)
ENTRY(_entry)
BASE_ADDRESS = 0x80000000;

MEMORY {
	sbi_space : ORIGIN = BASE_ADDRESS, LENGTH = 128K
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

	erodata = .;
	sdata = .;
	.data : {
		*(.data .data.*)
		*(.sdata .sdata.*)
	} > sbi_space

	edata = .;
	.bss : ALIGN(4K) {
		*(.bss.heap)
		*(.bss.stack)
		sbss = .;
		*(.bss .bss.*)
		*(.sbss .sbss.*)
	} > sbi_space

	ebss = .;
	ekernel = .;

	/DISCARD/ : {
		*(.eh_frame)
	}
}