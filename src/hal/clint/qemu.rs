// The driver for QEMU clint device

const BASE: usize = 0x0200_0000;
mod offset {
	pub const MTIME: usize = 0xbff8;
	pub const MTIMECMP: usize = 0x4000;
}

use core::ptr::{
	read_volatile, write_volatile, 
};

// mtime and mtimecmp may differ from hart, so it is necessary to specify hartid

pub(super) fn mtime(_hartid: usize) ->u64 {
	unsafe {
		read_volatile((BASE + offset::MTIME) as *const u64)
	}
}

pub(super) fn set_timer(hartid: usize, timer_val: u64) {
	unsafe {
		let mtimecmp = (BASE + offset::MTIMECMP) as *mut u64;
		write_volatile(
			mtimecmp.add(hartid), 
			timer_val
		)
	}
}

pub(super) fn send_ipi(hartid: usize) {
	unsafe {
		write_volatile((BASE as *mut u32).add(hartid), 1);
	}
}

pub(super) fn clear_ipi(hartid: usize) {
	unsafe {
		write_volatile(
			(BASE as *mut u32).add(hartid), 
			0
		)
	}
}