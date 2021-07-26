/* basic configurations about PascSBI */

pub const CLK: u32 = 11_059_200;

pub const NCPU: usize = 2;

pub const STACK_SIZE: usize = 2 * 1024;		// 2 KiB per hart
pub const STACK_OFFSET: usize = 11;

pub const HEAP_SIZE: usize = 4 * 1024;		// 4 KiB for heap

pub mod uart {
	pub const BAUDRATE: u32 = 115200;
	pub const RECV_IRQ: bool = true;
	pub const TRANS_IRQ: bool = false;
}