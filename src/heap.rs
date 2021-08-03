// use buddy_system_allocator::LockedHeap;
use linked_list_allocator::LockedHeap;

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) ->! {
	loop {}
}

#[global_allocator]
static mut HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init() {
	extern "C" {
		fn _sheap();
		fn _heap_size();
	}
	let sheap = &mut _sheap as *mut _ as usize;
	let heap_size = &_heap_size as *const _ as usize;

	unsafe {
		HEAP_ALLOCATOR.lock().init(sheap, heap_size);
	}
}