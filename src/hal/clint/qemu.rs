// The driver for QEMU clint device

const BASE: usize = 0x0200_0000;
mod offset {
	pub const MTIME: usize = 0xbff8;
	pub const MTIMECMP: usize = 0x4000;
}

pub(super) struct Clint;
impl Clint {
	pub fn init() ->Clint {
		Self {}
	}
}

use core::ptr::{
	read_volatile, write_volatile, 
};

#[allow(unused_variables)]
impl super::ClintHandler for Clint {
	// mtime and mtimecmp may differ from hart, so it is necessary to specify hartid

	fn mtime(&self, hartid: usize) ->u64 {
		unsafe {
			read_volatile((BASE + offset::MTIME) as *const u64)
		}
	}

	fn set_timer(&mut self, hartid: usize, timer_val: u64) {
		unsafe {
			let mtimecmp = (BASE + offset::MTIMECMP) as *mut u64;
			write_volatile(
				mtimecmp.add(hartid), 
				timer_val
			)
		}
	}

	fn send_ipi(&mut self, hartid: usize) {
		unsafe {
			write_volatile((BASE as *mut u32).add(hartid), 1);
		}
	}

	fn clear_ipi(&mut self, hartid: usize) {
		unsafe {
			write_volatile(
				(BASE as *mut u32).add(hartid), 
				0
			)
		}
	}
}