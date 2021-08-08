// SBI legacy extension group

// Please notice that Legacy SBI calls are "deprecated in favor of the 
// other extensions" according to SBI spec. I personally suggest avoiding
// using them in your S-mode kernel codes. 
// However, two key functions `sbi_console_getchar()` and `sbi_console_putchar()`
// "have no replacement" for **now**. As they are so important to the kernel for 
// output, it would be hard to escape from using them. 

#![allow(dead_code)]

use super::TrapFrame;
use super::error::*;

const EID_SET_TIMER: i64 = 0;
const EID_CONSOLE_PUTCHAR: i64 = 1;
const EID_CONSOLE_GETCHAR: i64 = 2;
const EID_CLEAR_IPI: i64 = 3;
const EID_SEND_IPI: i64 = 4;
const EID_REMOTE_FENCE_I: i64 = 5;
const EID_REMOTE_SFENCE_VMA: i64 = 6;
const EID_REMOTE_SFENCE_VMA_ASID: i64 = 7;
const EID_SHUTDOWN: i64 = 8;

use crate::hal::{uart, clint, };
use riscv::register::{
	mip, mstatus, mie, 
};

pub(super) fn handler(tf: &mut TrapFrame) {
	let eid = tf.a7;

	match eid {
		EID_SET_TIMER => {
			// void sbi_set_timer(uint64_t stime_value)
			let stime_value = tf.a0 as u64;
			clint::set_timer(stime_value);
			if mip::read().mtimer() {
				unsafe {
					mip::set_stimer();
					mie::clear_mtimer();
				}
			}
			else {
				unsafe {
					mip::clear_stimer();
					mie::set_mtimer();
				}
			}
		}, 
		EID_CONSOLE_PUTCHAR => {
			// void sbi_console_putchar(int ch)
			let c = tf.a0 as u8;
			uart::putchar(c);
		}, 
		EID_CONSOLE_GETCHAR => {
			// int sbi_console_getchar(void)
			tf.a0 = (uart::getchar() as i8) as i64;
		}, 
		EID_CLEAR_IPI => {
			// void sbi_clear_ipi(void)
			// This funciton is deprecated as SSIP can be cleared in Supervisor by software now
			unsafe {
				mip::clear_ssoft();
			}
		}, 
		EID_SEND_IPI => {
			// void sbi_send_ipi(const unsigned long *hart_mask)
			// It's really wierd to pass hart_mask via a pointer, maybe it's for 
			// cases where more than 32 harts exists
			// And as this function is deprecated, the new interface use 
			// (hart_mask: u32, hart_mask_base: u32) instead, which sounds more reasonable.
			let hart_mask = unsafe {
				*(tf.a0 as *const u32)
			} as usize;

			use crate::config::NCPU;
			for hartid in 0..NCPU {
				let mask = 1 << hartid;
				if 0 != (hart_mask & mask) {
					clint::send_ipi(hartid);
				}
			}
		}, 
		EID_SHUTDOWN => {
			// void sbi_shutdown(void)
			// turn off interrupts and run into wfi

			println!("SBI shutdown...");
			unsafe {
				mstatus::clear_mie();
				mstatus::clear_sie();
				mstatus::clear_uie();

				loop {
					riscv::asm::wfi();
				}
			}
		}, 
		_ => {
			// To be simple we don't implement any "remote fence" sbi functions
			// in legacy extension. They may be added later in RFENCE Extension
			println!("unsupported legacy SBI function: {:#x}", eid);
			tf.a0 = ERR_NOT_SUPPORTED;
		}
	}
}

#[inline]
pub(super) fn has_extension(ext: i64) ->bool {
	(EID_SET_TIMER == ext) |
	(EID_CONSOLE_PUTCHAR == ext) |
	(EID_CONSOLE_GETCHAR == ext) |
	(EID_CLEAR_IPI == ext) |
	(EID_SEND_IPI == ext) |
	(EID_SHUTDOWN == ext)
}