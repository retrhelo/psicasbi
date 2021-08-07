#[cfg(feature = "qemu")]
mod qemu;
#[cfg(feature = "k210")]
mod k210;

use riscv::register::mhartid;

#[allow(dead_code)]
pub fn get_mtime() ->u64 {
	let hartid = mhartid::read();
	match () {
		#[cfg(feature = "qemu")]
		() => {
			qemu::mtime(hartid)
		}, 
		#[cfg(feature = "k210")]
		() => {
			k210::mtime(hartid)
		}, 
	}
}

#[allow(dead_code)]
pub fn set_timer(timer_val: u64) {
	let hartid = mhartid::read();
	match () {
		#[cfg(feature = "qemu")]
		() => {
			qemu::set_timer(hartid, timer_val);
		}, 
		#[cfg(feature = "k210")]
		() => {
			k210::set_timer(hartid, timer_val);
		}, 
	}
}

/// Set M-Mode IPI by writting to memory-mapped msip reg 
#[allow(dead_code)]
pub fn send_ipi(hartid: usize) {
	match () {
		#[cfg(feature = "qemu")]
		() => {
			qemu::send_ipi(hartid);
		}, 
		#[cfg(feature = "k210")]
		() => {
			k210::send_ipi(hartid);
		}, 
	}
}

/// Clear M-Mode IPI by writting to memory-mapped msip reg
#[allow(dead_code)]
pub fn clear_ipi(hartid: usize) {
	match () {
		#[cfg(feature = "qemu")]
		() => {
			qemu::clear_ipi(hartid);
		}, 
		#[cfg(feature = "k210")]
		() => {
			k210::clear_ipi(hartid);
		}, 
	}
}