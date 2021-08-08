// Timer Extension "Time"

use super::{SbiRet, TrapFrame};
use super::error::*;

use riscv::register::{mie, mip};

use crate::hal::clint;

const SET_TIMER: i64 = 0;

pub(super) fn handler(tf: &TrapFrame) ->SbiRet {
	let fid = tf.a6;

	match fid {
		SET_TIMER => {
			let stime_value = tf.a0 as u64;

			// set mtimecmp
			// writting to mtimecmp will clear MTIP in mip, which can only be 
			// cleared in this way
			clint::set_timer(stime_value);
			// clear MTIP pending bit and enable M-mode time interrupt 
			unsafe {
				mip::clear_stimer();
				mie::set_mtimer();
			}
			SbiRet(SUCCESS, 0)
		}, 
		_ => {
			SbiRet(ERR_NOT_SUPPORTED, 0)
		}
	}
}