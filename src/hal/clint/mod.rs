#[cfg(feature = "qemu")]
mod qemu;
#[cfg(feature = "k210")]
mod k210;

use alloc::boxed::Box;
use core::option::Option;

trait ClintHandler {
	// for mtime
	fn mtime(&self, hartid: usize) ->u64;
	fn set_timer(&mut self, hartid: usize, timer_val: u64);

	// inter-processor interrupt
	fn send_ipi(&mut self, hartid: usize);
	fn clear_ipi(&mut self, hartid: usize);
}

static mut CLINT_INST: spin::Mutex<Option<Box<dyn ClintHandler>>> = 
		spin::Mutex::new(None);

pub fn init() {
	match () {
		#[cfg(feature = "qemu")]
		() => {
			unsafe {*CLINT_INST.lock() = Some(Box::new(qemu::Clint::init()));}
		}, 
		#[cfg(feature = "k210")]
		() => {}, 
	}
}

use riscv::register::mhartid;

#[allow(dead_code)]
pub fn get_mtime() ->u64 {
	let hartid = mhartid::read();
	unsafe {
		CLINT_INST.lock().as_ref().unwrap().mtime(hartid)
	}
}

#[allow(dead_code)]
pub fn set_timer(timer_val: u64) {
	let hartid = mhartid::read();
	unsafe {
		CLINT_INST.lock().as_mut().unwrap().set_timer(hartid, timer_val);
	}
}

#[allow(dead_code)]
pub fn send_ipi(hartid: usize) {
	unsafe {
		CLINT_INST.lock().as_mut().unwrap().send_ipi(hartid);
	}
}

#[allow(dead_code)]
pub fn clear_ipi(hartid: usize) {
	unsafe {
		CLINT_INST.lock().as_mut().unwrap().clear_ipi(hartid);
	}
}