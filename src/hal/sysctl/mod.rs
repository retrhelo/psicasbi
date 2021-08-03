// the abstraction of SYSCTL, which is common on SoC boards

// retrhelo: 
// According to my understanding, I think SYSCTL is some hardware 
// component controlling the clock freqency of CPU and peripherals. 
// Currently only k210's sysctl is in consideration of being supported, 
// thus the interface below is designed for use of k210's. As more boards 
// being supported, the interface may evovle into a more generic version.

use alloc::boxed::Box;
use spin::Mutex;

static mut SYSCTL_INST: Mutex<Option<Box<dyn SysCtlHandler>>> = 
		Mutex::new(None);

trait SysCtlHandler {
	// config clock freqency on this SoC 
	// detailed implementation differs from different dev boards
	fn set_freq(&mut self);
}

mod k210;

pub fn init() {
	unsafe {
		*SYSCTL_INST.lock() = Some(Box::new(k210::init()))
	}
}

pub fn set_freq() {
	unsafe {
		SYSCTL_INST.lock().as_mut().unwrap().set_freq();
	}
}