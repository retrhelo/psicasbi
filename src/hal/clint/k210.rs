// CLINT driver for k210

use k210_pac as pac;

// mtime and mtimecmp may differ from hart, so it's necessary to specify hartid 

pub(super) fn mtime(_hartid: usize) ->u64 {
	unsafe {
		(*pac::CLINT::ptr()).mtime.read().bits()
	}
}

pub(super) fn set_timer(hartid: usize, timer_val: u64) {
	unsafe {
		(*pac::CLINT::ptr()).mtimecmp[hartid].write(|w| {
			w.bits(timer_val)
		})
	}
}

pub(super) fn send_ipi(hartid: usize) {
	unsafe {
		(*pac::CLINT::ptr()).msip[hartid].write(|w| {
			w.bits(1)
		})
	}
}

pub(super) fn clear_ipi(hartid: usize) {
	unsafe {
		(*pac::CLINT::ptr()).msip[hartid].write(|w| {
			w.bits(0)
		})
	}
}