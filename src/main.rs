#![no_std]
#![no_main]
#![feature(asm)]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;


use buddy_system_allocator::LockedHeap;

#[global_allocator]
static mut HEAP: LockedHeap<32> = LockedHeap::empty();

#[panic_handler]
fn panic(_info: &PanicInfo) ->! {
	loop {}
}

fn main() {}