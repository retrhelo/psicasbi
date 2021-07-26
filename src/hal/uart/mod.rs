// the abstraction of UART

mod qemu;
mod k210;

use core::option::Option;
use alloc::boxed::Box;

trait UartHandler {
	fn getchar(&mut self) ->u8;
	fn putchar(&mut self, c: u8);
}

static mut UART_INST: spin::Mutex<Option<Box<dyn UartHandler>>> = spin::Mutex::new(None);

pub fn init() {
	unsafe {	// I wonder why this is unsafe here
		*UART_INST.lock() = Some(Box::new(qemu::NS16550a::init()));
	}
}

pub fn putchar(c: u8) {
	unsafe {
		UART_INST.lock().as_mut().unwrap().putchar(c);
	}
}

pub fn getchar() ->u8 {
	unsafe {
		UART_INST.lock().as_mut().unwrap().getchar()
	}
}