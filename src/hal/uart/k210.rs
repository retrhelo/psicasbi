// k210 UARTHS interface 

use k210_pac as pac;
use crate::config::CLK;
use crate::config::uart;

pub(super) struct Uart;

impl Uart {
	pub fn init() ->Self {
		// let baud = 26_000_000 / uart::BAUDRATE - 1;

		// let base = 26_000_000;
		let base = 390_000_000;
		let baud = base / (uart::BAUDRATE as u32) - 1;
		let uarths = pac::UARTHS::ptr();
		unsafe {
			// first set div 
			(*uarths).div.write(|w| {
				w.bits(baud)
			});

			// enable interrupts 
			(*uarths).ie.write(|w| {
				if uart::RECV_IRQ {
					w.rxwm().set_bit();
				}
				if uart::TRANS_IRQ {
					w.txwm().set_bit();
				}

				w
			});

			// finally, enable RX/TX
			(*uarths).rxctrl.write(|w| {
				w.rxcnt().bits(0)
					.rxen().set_bit()
			});
			(*uarths).txctrl.write(|w| {
				w.txcnt().bits(0)
					.txen().set_bit()
			});
		}

		Self {}
	}

	fn _getchar(&mut self) ->u8 {
		let rxdata = unsafe {(*pac::UARTHS::ptr()).rxdata.read()};

		if rxdata.empty().bit_is_set() {0xff}
		else {rxdata.data().bits()}
	}

	fn _putchar(&mut self, c: u8) {
		let uarths = pac::UARTHS::ptr();

		// loop {
		// 	unsafe {
		// 		if (*uarths).txdata.read().full().bit_is_clear() {
		// 			// txdata ok to transmit 
		// 			(*uarths).txdata.modify(|_, w| {
		// 				w.data().bits(c)
		// 			});
		// 			break;
		// 		}
		// 	}
		// }
		// unsafe {
		// 	(*uarths).txdata.modify(|_, w| {
		// 		w.data().bits(c)
		// 	});
		// }
		loop {
			unsafe {
				if (*uarths).txdata.read().full().bit_is_clear() {
					(*uarths).txdata.modify(|_, w| {
						w.data().bits(c)
					});
					break;
				}
			}
		}
	}
}

impl core::fmt::Write for Uart {
	fn write_str(&mut self, fmt: &str) ->core::fmt::Result {
		// let mut buffer = [0u8; 4];
		// for c in fmt.chars() {
		// 	for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
		// 		self._putchar(*code_point as u8);
		// 	}
		// }
		for c in fmt.bytes() {
			self._putchar(c);
		}

		Ok(())
	}
}

impl super::UartHandler for Uart {
	#[inline]
	fn getchar(&mut self) ->u8 {
		self._getchar()
	}

	#[inline]
	fn putchar(&mut self, c: u8) {
		self._putchar(c);
	}
}