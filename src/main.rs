#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(default_alloc_error_handler)]

mod config;		// basic configurations 
mod heap;		// SBI heap
mod hal;		// hardware abstraction layer 
mod console;	// for output

extern crate alloc;

use core::panic::PanicInfo;
#[panic_handler]
#[allow(dead_code)]
fn panic(_info: &PanicInfo) ->! {
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

		// print logo
		println!("{}", config::LOGO);
	}
	else {
		unsafe {
			// hang up for M-mode ipi
			asm::wfi();
		}
	}
}