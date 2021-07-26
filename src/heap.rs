use crate::config::HEAP_SIZE;

#[link_section = ".bss.heap"]
static mut SBI_HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

use buddy_system_allocator::LockedHeap;

#[global_allocator]
static mut HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

pub fn init() {
	unsafe {
		HEAP_ALLOCATOR.lock()
			.init(SBI_HEAP.as_ptr() as usize, HEAP_SIZE);
	}
}