#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(default_alloc_error_handler)]
#![feature(fn_align)]

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
fn panic(info: &PanicInfo) ->! {
	println!("\x1b[31;1m[panic]\x1b[0m: {}", info);
	println!("\x1b[31;1m[panic]\x1b[0m: PsicaSBI stops");
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
use riscv::register::mstatus::MPP;
use riscv::register::{
	mepc, mstatus, misa, 
};

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
		println!("{}", LOGO);
		println!(
			"[\x1b[32;1mPsicaSBI\x1b[0m]: Version {}.{}", 
			*SBI_IMPL_VER_MAJOR, 
			*SBI_IMPL_VER_MINOR
		);

		let mideleg: usize;
		let medeleg: usize;
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

		// print extension informations
		let misa = misa::read().unwrap();
		match misa.mxl() {
			misa::MXL::XLEN32 => {
				panic!("RV32 not supported");
			}, 
			misa::MXL::XLEN64 => {
				print!("Extension: RV64");
			}, 
			misa::MXL::XLEN128 => {
				panic!("RV128 tql, not supported");
			}
		}
		for ext in 'A'..'Z' {
			if misa.has_extension(ext) {
				print!("{}", ext);
			}
		}
		println!("");
	}
	else {
		trap::init();
		println!("trap init");

		unsafe {
			// hang up for M-mode ipi
			asm::wfi();
		}
	}

	// jump to S-mode kernel
	unsafe {
		mepc::write(KERNEL_ENTRY);
		mstatus::set_mpp(MPP::Supervisor);
		asm!(
			"mret", 
			in("a0") hartid, 
			options(noreturn)
		);
	}
}