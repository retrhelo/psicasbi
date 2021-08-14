// Some xv6-k210 specific SBI functions 

use crate::trap::sbi::{TrapFrame, SbiRet};
use crate::trap::sbi::error::*;

use riscv::register::mie;

use crate::hal::{uart, clint};

const SET_EXT: i64 = 0;
const IS_EXT: i64 = 1;
const CONSOLE_PUTS: i64 = 0x10;
const GET_TIMER: i64 = 0x20;

pub(super) fn handler(tf: &TrapFrame) ->SbiRet {
	let fid = tf.a6;

	match fid {
		SET_EXT => {
			// struct sbiret sbi_xv6_set_ext();
			unsafe {
				mie::set_mext();
				mie::set_mtimer();
			}

			SbiRet(SUCCESS, 0)
		}, 
		IS_EXT => {
			// struct sbiret sbi_xv6_is_ext();
			// as what we do, when there's an external interrupt, 
			// mie::mext is clear to avoid multiple interrupts
			if mie::read().mext() {
				SbiRet(SUCCESS, 0)
			}
			else {
				SbiRet(SUCCESS, 1)
			}
		}, 
		CONSOLE_PUTS => {
			// struct sbiret sbi_xv6_puts(char *str, int n);
			let mut str = tf.a0 as *const u8;
			let n = tf.a1 as usize;

			let mut cnt = 0;
			while cnt < n {
				let c = unsafe {*str};
				if 0 == c {break;}
				else {
					uart::putchar(c);
				}

				str = str.wrapping_add(1);
				cnt += 1;
			}

			SbiRet(SUCCESS, cnt as i64)
		}, 
		GET_TIMER => {
			// struct sbiret sbi_xv6_get_timer(void);
			// This may be implemented other way, as k210 provides some other timers, 
			// they may be a better choice than mtime
			let mtime = clint::get_mtime();
			SbiRet(SUCCESS, mtime as i64)
		}, 
		_ => {
			SbiRet(ERR_NOT_SUPPORTED, 0)
		}
	}
}