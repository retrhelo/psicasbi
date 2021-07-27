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

const GET_SPECIFICATION_VER: i32 = 0;
const GET_IMPL_ID: i32 = 1;
const GET_IMPL_VER: i32 = 2;
const PROBE_EXTENSION: i32 = 3;
const GET_MACHINE_VENDOR_ID: i32 = 4;
const GET_MACHINE_ARCH_ID: i32 = 5;
const GET_MACHINE_IMPL_ID: i32 = 6;

pub(super) fn handler(tf: &mut TrapFrame) ->SbiRet {
	let fid = tf.a6 as i32;

	match fid {
		GET_SPECIFICATION_VER => {
			// this function should always succeed
			SbiRet(
				SUCCESS, 
				make_ver(SBI_SPEC_VER_MAJOR, SBI_SPEC_VER_MINOR) as i32
			)
		}, 
		GET_IMPL_ID => {
			SbiRet(SUCCESS, SBI_IMPL_ID as i32)
		}, 
		GET_IMPL_VER => {
			SbiRet(
				SUCCESS, 
				make_ver(*SBI_IMPL_VER_MAJOR, *SBI_IMPL_VER_MINOR) as i32
			)
		}, 
		PROBE_EXTENSION => {
			let ext = tf.a0 as i32;

			if probe_extension(ext) {
				SbiRet(SUCCESS, 0)
			}
			else {
				SbiRet(ERR_FAILED, 0)
			}
		}, 
		GET_MACHINE_VENDOR_ID => {
			let val = mvendorid::read().unwrap().bits();

			SbiRet(SUCCESS, val as i32)
		}, 
		GET_MACHINE_ARCH_ID => {
			let val = marchid::read().unwrap().bits();

			SbiRet(SUCCESS, val as i32)
		}, 
		GET_MACHINE_IMPL_ID => {
			let val = mimpid::read().unwrap().bits();

			SbiRet(SUCCESS, val as i32)
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
fn probe_extension(ext: i32) ->bool {
	super::EID_BASE == ext
}