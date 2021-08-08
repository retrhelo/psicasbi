// Vendor Specific SBI Extension Space 

use super::{SbiRet, TrapFrame};
use super::error::*;

pub(super) fn handler(tf: &TrapFrame) ->SbiRet {
	let eid = tf.a7;

	match eid {
		_ => {
			SbiRet(ERR_NOT_SUPPORTED, 0)
		}
	}
}

#[inline]
pub(super) fn has_extension(_ext: i64) ->bool {
	false
}