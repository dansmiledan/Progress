use crate::bsp;

pub mod interface {
	use core::fmt;
	pub trait Statistics {
		fn chars_written(&self) -> usize {
			0
		}
	}

	pub trait Write {
		fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
	}

	pub trait All: Write + Statistics {

	}
}

pub fn console() -> &'static dyn interface::All {
	bsp::console::console()
}

