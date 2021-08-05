// Base Extension of SBI

use super::TrapFrame;
use super::error::*;
use super::SbiRet;

use riscv::register::{
	mvendorid, 
	marchid, 
	mimpid, 
};

use crate::config::{
	SBI_IMPL_ID, 
	SBI_IMPL_VER_MAJOR, 
	SBI_IMPL_VER_MINOR, 
	SBI_SPEC_VER_MAJOR, 
	SBI_SPEC_VER_MINOR, 
};

const GET_SPECIFICATION_VER: i64 = 0;
const GET_IMPL_ID: i64 = 1;
const GET_IMPL_VER: i64 = 2;
const PROBE_EXTENSION: i64 = 3;
const GET_MACHINE_VENDOR_ID: i64 = 4;
const GET_MACHINE_ARCH_ID: i64 = 5;
const GET_MACHINE_IMPL_ID: i64 = 6;

pub(super) fn handler(tf: &TrapFrame) ->SbiRet {
	let fid = tf.a6;

	match fid {
		GET_SPECIFICATION_VER => {
			// this function should always succeed
			SbiRet(
				SUCCESS, 
				make_ver(SBI_SPEC_VER_MAJOR, SBI_SPEC_VER_MINOR) as i64
			)
		}, 
		GET_IMPL_ID => {
			SbiRet(SUCCESS, SBI_IMPL_ID as i64)
		}, 
		GET_IMPL_VER => {
			SbiRet(
				SUCCESS, 
				make_ver(*SBI_IMPL_VER_MAJOR, *SBI_IMPL_VER_MINOR) as i64
			)
		}, 
		PROBE_EXTENSION => {
			let ext = tf.a0;

			if probe_extension(ext) {
				SbiRet(SUCCESS, ext)
			}
			else {
				SbiRet(ERR_FAILED, 0)
			}
		}, 
		GET_MACHINE_VENDOR_ID => {
			let val = if let Some(bits) = mvendorid::read() {
				bits.bits()
			} else {0};

			SbiRet(SUCCESS, val as i64)
		}, 
		GET_MACHINE_ARCH_ID => {
			let val = if let Some(bits) = marchid::read() {
				bits.bits()
			} else {0};

			SbiRet(SUCCESS, val as i64)
		}, 
		GET_MACHINE_IMPL_ID => {
			let val = if let Some(bits) = mimpid::read() {
				bits.bits()
			} else {0};

			SbiRet(SUCCESS, val as i64)
		}, 
		_ => {	// any unsupported sbi call for this extension
			SbiRet(ERR_NOT_SUPPORTED, 0)
		}, 
	}
}

// combine major and minor into the version that fits SBI spec
#[inline]
fn make_ver(major: u32, minor: u32) ->u32 {
	let major = major & 0x7f;
	let minor = minor & 0xff_ffff;

	(major << 24) | minor
}

#[inline]
fn probe_extension(ext: i64) ->bool {
	(super::EID_BASE == ext) | 
	(super::legacy::EID_SET_TIMER == ext) | 
	(super::legacy::EID_CONSOLE_PUTCHAR == ext) | 
	(super::legacy::EID_CONSOLE_GETCHAR == ext) |
	(super::legacy::EID_CLEAR_IPI == ext) |
	(super::legacy::EID_SEND_IPI == ext) |
	(super::legacy::EID_SHUTDOWN == ext)
}