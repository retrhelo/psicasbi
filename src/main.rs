#![no_std]
#![no_main]

#![feature(naked_functions)]
#![feature(asm)]
#![feature(alloc_error_handler)]
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

#[naked]
#[no_mangle]
#[link_section = ".text.init"]
unsafe extern "C" fn _entry() ->! {
	asm!(r"
		li sp, {stack_top}
		csrr a0, mhartid
		slli a0, a0, {offset}
		sub sp, sp, a0
		csrw mscratch, sp
		
		csrr a0, mhartid
		call rust_main

		1:
			j 1b
	", 
		stack_top = const KERNEL_ENTRY, 
		offset = const STACK_OFFSET, 
		options(noreturn), 
	)
}

// use riscv::asm;
use riscv::register::mstatus::MPP;
use riscv::register::{
	mepc, mstatus, misa, 
};

#[no_mangle]
#[link_section = ".text.init"]
extern "C" fn rust_main(hartid: usize) {
	if 0 == hartid {
		// init bss section 
		extern "C" {
			static mut _sbss: u32;
			static mut _ebss: u32;
		}
		unsafe {
			r0::zero_bss(&mut _sbss, &mut _ebss);
		}
		heap::init();

		#[cfg(feature = "k210")]
		{
			hal::sysctl::init();
			hal::sysctl::set_freq();
			hal::fpioa::init();
		}
		hal::uart::init();

		extern "C" {
			fn ekernel();
		}
		let ekernel = ekernel as usize;
		println!("ekernel: {:#x}", ekernel);
		println!("heap_start: {:#x}", config::HEAP_START);
		println!("heap_size: {:#x}", config::HEAP_SIZE);
		assert!(config::HEAP_START > ekernel);

		hal::clint::init();		// init CLINT
		println!("clint init");

		trap::init();			// install trap handler, this should run per-hart
		println!("trap init");

		println!("{}", LOGO);
		println!(
			"[\x1b[32;1mPsicaSBI\x1b[0m]: Version {}.{}", 
			*SBI_IMPL_VER_MAJOR, 
			*SBI_IMPL_VER_MINOR
		);

		let mideleg: usize;
		let medeleg: usize;
		unsafe {
			asm!(
				"csrr {0}, mideleg", 
				out(reg) mideleg
			);
			asm!(
				"csrr {0}, medeleg", 
				out(reg) medeleg
			);
		}
		println!("mideleg: {:#x}, medeleg: {:#x}", mideleg, medeleg);

		let misa = misa::read().unwrap();
		match misa.mxl() {
			misa::MXL::XLEN32 => {
				panic!("RV32 not supported");
			}, 
			misa::MXL::XLEN64 => {
				print!("Extension: RV64");
			}, 
			misa::MXL::XLEN128 => {
				panic!("RV128 tql, not supported yet");
			}
		}
		for ext in 'A'..='Z' {
			if misa.has_extension(ext) {
				print!("{}", ext);
			}
		}
		println!("");
	}
	else {
		loop {
			unsafe {riscv::asm::wfi();}
		}
	}
	println!("[\x1b[32;1mPsicaSBI\x1b[0m]: hartid {}", hartid);

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