/* QEMU use NS16550a UART hardware as its Serial Communication hardware
	For a detailed description of NS16550, see http://byterunner.com/16550.html
*/

#[allow(dead_code)]
const BASE: usize = 0x1000_0000;
#[allow(dead_code)]
mod offset {
	pub const RHR: usize = 0x0;
	pub const THR: usize = 0x0;
	pub const DLL: usize = 0x0;
	pub const IER: usize = 0x1;
	pub const DLH: usize = 0x1;
	pub const FCR: usize = 0x2;
	pub const ISR: usize = 0x2;
	pub const LCR: usize = 0x3;
	// pub const MCR: usize = 0x4;
	pub const LSR: usize = 0x5;
	// pub const MSR: usize = 0x6;
	pub const SPR: usize = 0x7;
}
#[allow(dead_code)]
mod mask {
	pub const IER_RECV: u8 = 1 << 0;		// rhr interrupt 
	pub const IER_TRANS: u8 = 1 << 1;	// thr interrupt

	pub const FCR_FIFO_EN: u8 = 1 << 0;		// FIFO enable
	pub const FCR_RECV_RST: u8 = 1 << 1;		// Recv FIFO reset
	pub const FCR_TRANS_RST: u8 = 1 << 2;	// Trans FIFO reset
	
	pub const LCR_LEN0: u8 = 1 << 0;
	pub const LCR_LEN1: u8 = 1 << 1;
	pub const LCR_STOPBIT: u8 = 1 << 2;
	pub const LCR_PARITY: u8 = 1 << 3;
	pub const LCR_LATCH: u8 = 1 << 7;	// internal baudrate latch enable

	pub const LSR_RECV_READY: u8 = 1 << 0;
	pub const LSR_TRANS_EMPTY: u8 = 1 << 6;
}

use core::ptr::{
	read_volatile, write_volatile, 
};

pub(super) struct NS16550a;

impl NS16550a {
	pub fn init() ->Self {
		use crate::config::*;
		use crate::config::uart::*;

		unsafe {
			// enable divisor latch
			write_volatile(
				(BASE + offset::LCR) as *mut u8, 
				mask::LCR_LATCH
			);

			// set baud rate 
			let latch = CLK / (16 * BAUDRATE);
			write_volatile(
				(BASE + offset::DLL) as *mut u8, 
				(latch & 0xff) as u8
			);
			write_volatile(
				(BASE + offset::DLH) as *mut u8, 
				(latch >> 8) as u8
			);

			// disable divisor latch and set serial format as 8N1
			write_volatile(
				(BASE + offset::LCR) as *mut u8, 
				mask::LCR_LEN0 | mask::LCR_LEN1
			);

			// enable interrupt for recv
			let int_mask = 
			if RECV_IRQ { mask::IER_RECV } else {0}
			| if TRANS_IRQ { mask::IER_TRANS } else {0};
			write_volatile(
				(BASE + offset::IER) as *mut u8, 
				int_mask
			);
			
			// enable FIFO 
			write_volatile(
				(BASE + offset::FCR) as *mut u8, 
				mask::FCR_FIFO_EN | 
				mask::FCR_RECV_RST | mask::FCR_TRANS_RST
			);
		}

		// finish initialization 
		Self {}
	}

	fn _getchar(&mut self) ->u8 {
		unsafe {
			// block until the data is ready 
			while 0 == read_volatile((BASE + offset::LSR) as *const u8) & mask::LSR_RECV_READY {}

			read_volatile((BASE + offset::RHR) as *const u8)
		}
	}

	fn _putchar(&mut self, c: u8) {
		unsafe {
			while 0 == read_volatile((BASE + offset::LSR) as *const u8) & mask::LSR_TRANS_EMPTY {}

			write_volatile((BASE + offset::THR) as *mut u8, c);
		}
	}
}

impl core::fmt::Write for NS16550a {
	fn write_str(&mut self, fmt: &str) ->core::fmt::Result {
		let mut buffer = [0u8; 4];
		for c in fmt.chars() {
			for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
				self._putchar(*code_point as u8);
			}
		}

		Ok(())
	}
}

impl super::UartHandler for NS16550a {
	fn getchar(&mut self) ->u8 {
		self._getchar()
	}

	fn putchar(&mut self, c: u8) {
		self._putchar(c);
	}
}