/* basic configurations about PsicaSBI */

#![allow(dead_code)]

#[cfg(feature = "qemu")]
pub const CLK: u64 = 11_059_200;
#[cfg(feature = "k210")]
pub const CLK: u64 = 26_000_000;

/// The entry of S-mode kernel
pub const KERNEL_ENTRY: usize = 0x8002_0000;

pub const NCPU: usize = 2;

pub const STACK_SIZE: usize = 4 * 1024;		// 4 KiB per hart
pub const STACK_OFFSET: usize = STACK_SIZE.trailing_zeros() as usize;

pub const HEAP_SIZE: usize = 4 * 4 * 1024;
pub const HEAP_START: usize = KERNEL_ENTRY - STACK_SIZE * NCPU - HEAP_SIZE;

pub mod uart {
	pub const BAUDRATE: u64 = 115200;
	pub const RECV_IRQ: bool = true;
	pub const TRANS_IRQ: bool = false;
}

// SBI specification version: 0.3.0
pub const SBI_SPEC_VER_MAJOR: u32 = 0;
pub const SBI_SPEC_VER_MINOR: u32 = 3;

// SBI implemenation version, directly from cargo
use lazy_static::*;
lazy_static! {
	pub static ref SBI_IMPL_VER_MAJOR: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
	pub static ref SBI_IMPL_VER_MINOR: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
}

pub const SBI_IMPL_ID: u64 = 0x0909;
pub const LOGO: &'static str = "
________   ________   ___   ________   ________   ________   ________   ___     
|\\   __  \\ |\\   ____\\ |\\  \\ |\\   ____\\ |\\   __  \\ |\\   ____\\ |\\   __  \\ |\\  \\    
\\ \\  \\|\\  \\\\ \\  \\___|_\\ \\  \\\\ \\  \\___| \\ \\  \\|\\  \\\\ \\  \\___|_\\ \\  \\|\\ /_\\ \\  \\   
 \\ \\   ____\\\\ \\_____  \\\\ \\  \\\\ \\  \\     \\ \\   __  \\\\ \\_____  \\\\ \\   __  \\\\ \\  \\  
  \\ \\  \\___| \\|____|\\  \\\\ \\  \\\\ \\  \\____ \\ \\  \\ \\  \\\\|____|\\  \\\\ \\  \\|\\  \\\\ \\  \\ 
   \\ \\__\\      ____\\_\\  \\\\ \\__\\\\ \\_______\\\\ \\__\\ \\__\\ ____\\_\\  \\\\ \\_______\\\\ \\__\\
    \\|__|     |\\_________\\\\|__| \\|_______| \\|__|\\|__||\\_________\\\\|_______| \\|__|
              \\|_________|                           \\|_________|
";