use buddy_system_allocator::LockedHeap;
// use linked_list_allocator::LockedHeap;

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) ->! {
	loop {}
}

#[global_allocator]
static mut HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

use crate::config::{HEAP_START, HEAP_SIZE};

pub fn init() {
	unsafe {
		HEAP_ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
	}
}