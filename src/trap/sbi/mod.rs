// Handler for SBI call

use super::TrapFrame;

mod base;

#[allow(dead_code)]
mod error {
	pub const SUCCESS: i32 = 0;
	pub const ERR_FAILED: i32 = -1;
	pub const ERR_NOT_SUPPORTED: i32 = -2;
	pub const ERR_INVALID_PARAM: i32 = -3;
	pub const ERR_DENIED: i32 = -4;
	pub const ERR_INVALID_ADDRESS: i32 = -5;
	pub const ERR_ALREADY_AVAILABLE: i32 = -6;
	pub const ERR_ALREADY_STARTED: i32 = -7;
	pub const ERR_ALREADY_STOPPED: i32 = -8;
}

const EID_BASE: i32 = 0x10;

// SbiRet(error, value)
struct SbiRet(i32, i32);

pub(super) fn handler(tf: &mut TrapFrame) {
	let eid = tf.a7 as i32;

	if EID_BASE == eid {
		let ret = base::handler(tf);
		tf.a0 = ret.0 as i64;
		tf.a1 = ret.1 as i64;
	}
	else {	// unsupported extension 
		println!("unsupported sbi extension: {}", eid);
		super::tf_dump(tf);
		tf.a0 = error::ERR_NOT_SUPPORTED as i64;
	}
}