// Handler for SBI call

use crate::trap::sbi::error::ERR_NOT_SUPPORTED;

use super::TrapFrame;

mod base;
mod legacy;

#[allow(dead_code)]
mod error {
	pub const SUCCESS: i64 = 0;
	pub const ERR_FAILED: i64 = -1;
	pub const ERR_NOT_SUPPORTED: i64 = -2;
	pub const ERR_INVALID_PARAM: i64 = -3;
	pub const ERR_DENIED: i64 = -4;
	pub const ERR_INVALID_ADDRESS: i64 = -5;
	pub const ERR_ALREADY_AVAILABLE: i64 = -6;
	pub const ERR_ALREADY_STARTED: i64 = -7;
	pub const ERR_ALREADY_STOPPED: i64 = -8;
}

const EID_BASE: i64 = 0x10;
const EID_LEGACY_MIN: i64 = 0x00;
const EID_LEGACY_MAX: i64 = 0x0f;

// SbiRet(error, value)
struct SbiRet(i64, i64);

pub(super) fn handler(tf: &mut TrapFrame) {
	let eid = tf.a7;

	if eid >= EID_LEGACY_MIN && eid <= EID_LEGACY_MAX {
		// a legacy sbi function, which returns int or void, different from other SBI functions
		// According to sbi spec, legacy calls are "deprecated in favor of the other
		// extensions". So I suggest avoid using them in the S-mode kernel
		legacy::handler(tf);
	}
	else {
		let ret = if EID_BASE == eid {
			base::handler(tf)
		}
		else {
			println!("unsupported SBI extension: {:#x}", eid);
			super::tf_dump(tf);
			SbiRet(ERR_NOT_SUPPORTED, 0)
		};

		tf.a0 = ret.0;
		tf.a1 = ret.1;
	}
}