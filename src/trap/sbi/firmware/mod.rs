// Firmware Specific SBI Extension Space 

use super::{SbiRet, TrapFrame};
use super::error::*;

mod xv6_k210;

const XV6_K210: i64 = 0x0a00_0210;

pub(super) fn handler(tf: &TrapFrame) ->SbiRet {
	let eid = tf.a7;

	match eid {
		XV6_K210 => {
			xv6_k210::handler(tf)
		}, 
		_ => {
			SbiRet(ERR_NOT_SUPPORTED, 0)
		}, 
	}
}

#[inline]
pub(super) fn has_extension(ext: i64) ->bool {
	XV6_K210 == ext
}