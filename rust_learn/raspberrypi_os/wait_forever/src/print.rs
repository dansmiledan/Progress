use crate::console;
use core::fmt;

pub fn _print(args: fmt::Arguments) {
	console::console().write_fmt(args).unwrap();
}


#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ({
		$crate::print::_print(format_args_nl!($($arg)*));
	})
}