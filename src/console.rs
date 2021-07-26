// Some useful codes that can be used everywhere

use crate::hal::uart::{
	putchar, 
};

use core::fmt::{self, Write};

struct Stdout;
impl Write for Stdout {
	fn write_str(&mut self, s: &str) ->fmt::Result {
		for c in s.chars() {
			putchar(c as u8);
		}

		Ok(())
	}
}

pub fn _print(args: fmt::Arguments) {
	Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::console::_print(format_args!($fmt $(, $($arg)+)?));
	}
}

#[macro_export]
macro_rules! println {
	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::console::_print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
	}
}