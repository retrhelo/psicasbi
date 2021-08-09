// IPI Extension 
// "sPI: s-mode IPI"

use super::{SbiRet, TrapFrame};
use super::error::*;

use crate::hal::clint;
use crate::config::NCPU;

const SEND_IPI: i64 = 0;

pub(super) fn handler(tf: &TrapFrame) ->SbiRet {
	let fid = tf.a6;

	match fid {
		SEND_IPI => {
			let hart_mask = tf.a0 as usize;
			let hart_mask_base = tf.a1 as usize;

			if hart_mask_base > NCPU {
				SbiRet(ERR_INVALID_PARAM, 0)
			}
			else {
				let max_shift = core::cmp::min(NCPU - hart_mask_base, 32);
				if 0 != (hart_mask >> max_shift) {
					SbiRet(ERR_INVALID_PARAM, 0)
				}
				else {
					for i in 0..max_shift {
						let hart = 1usize << i;
						if 0 != (hart_mask & hart) {
							clint::send_ipi(i);
						}
					}
					SbiRet(SUCCESS, 0)
				}
			}
		}, 
		_ => {
			SbiRet(ERR_NOT_SUPPORTED, 0)
		}, 
		// Different from Legacy, sPI extension does not provide 
		// `clear_ipi` calling. That's because SSIP bit can now be 
		// cleared by software in S-mode.
	}
}