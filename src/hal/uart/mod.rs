// the abstraction of UART

mod qemu;
mod k210;

use core::option::Option;
use alloc::boxed::Box;

use core::fmt;

trait UartHandler: fmt::Write {
	fn getchar(&mut self) ->u8;
	fn putchar(&mut self, c: u8);
}

static mut UART_INST: spin::Mutex<Option<Box<dyn UartHandler>>> = 
		spin::Mutex::new(None);

pub fn init() {
	unsafe {
		*UART_INST.lock() = Some(Box::new(qemu::NS16550a::init()));
	}
}

#[allow(dead_code)]
pub fn putchar(c: u8) {
	unsafe {
		UART_INST.lock().as_mut().unwrap().putchar(c);
	}
}

#[allow(dead_code)]
pub fn getchar() ->u8 {
	unsafe {
		UART_INST.lock().as_mut().unwrap().getchar()
	}
}

pub fn _print(args: fmt::Arguments) {
	// UART_INST.lock().write_fmt(args).unwrap();
	unsafe {
		UART_INST.lock().as_mut().unwrap().write_fmt(args).unwrap();
	}
}

#[macro_export]
macro_rules! print {
	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::hal::uart::_print(format_args!($fmt $(, $($arg)+)?));
	}
}

#[macro_export]
macro_rules! println {
	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::hal::uart::_print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
	}
}