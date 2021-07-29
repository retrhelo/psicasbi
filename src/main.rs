#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(default_alloc_error_handler)]

// basic configurations 
mod config;
// SBI heap
mod heap;
// hardware abstraction layer 
#[macro_use]
mod hal;
// trap vector
mod trap;

extern crate alloc;

use core::panic::PanicInfo;
#[panic_handler]
#[allow(dead_code)]
fn panic(_info: &PanicInfo) ->! {
	println!("\033[31;1m[panic]\033[0m: PascSBI stops");
	loop {}
}

use config::*;

const STACK_TOTAL_SIZE: usize = STACK_SIZE * NCPU;
#[link_section = ".bss.stack"]
static mut SBI_STACK: [u8; STACK_TOTAL_SIZE] = [0; STACK_TOTAL_SIZE];

#[naked]
#[no_mangle]
#[link_section = ".text.init"]
unsafe extern "C" fn _entry() ->! {
	asm!(r"
		csrr tp, mhartid
		mv a0, tp
		slli tp, tp, {offset}
		la sp, {stack_base}
		add sp, sp, tp
		
		j rust_main
	", 
		offset = const STACK_OFFSET, 
		stack_base = sym SBI_STACK, 
		options(noreturn), 
	)
}

use riscv::asm;

#[no_mangle]
#[link_section = ".text.init"]
extern "C" fn rust_main(hartid: usize) {
	if 0 == hartid {
		heap::init();	// init heap
		// faster UART is working, the better
		hal::uart::init();	// init uart

		hal::clint::init();		// init CLINT
		println!("clint init");

		trap::init();			// init trap handling
		println!("trap init");

		// display PascSBI information
		println!(
			"[\033[32;1mPascSBI\033[0m]: Version {}.{}", 
			*SBI_IMPL_VER_MAJOR, 
			*SBI_IMPL_VER_MINOR
		);
		println!("{}", LOGO);
		let mideleg: u64;
		let medeleg: u64;
		unsafe {	// read mideleg and medeleg and print
			asm!("
				csrr {0}, mideleg
			", out(reg) mideleg);
			asm!(
				"csrr {0}, medeleg", 
				out(reg) medeleg
			);
		}
		println!("mideleg: {:#x}, medeleg: {:#x}", mideleg, medeleg);
	}
	else {
		trap::init();
		println!("trap init");

		unsafe {
			// hang up for M-mode ipi
			asm::wfi();
		}
	}

	// !TODO: test if it jumps into S-mode kernel
	// jump to S-mode kernel
	unsafe {
		asm!(
			"csrw mepc, {kernel_entry}", 
			"mret", 
			kernel_entry = const KERNEL_ENTRY, 
			options(noreturn)
		);
	}
}