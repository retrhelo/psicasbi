// FPIOA: Field Programmable Input and Output Array 

// I only find it on k210, not sure if it's a Kendryte-Only design

use k210_pac as pac;

const FUNC_UARTHS_RX: u8 = 18;
const FUNC_UARTHS_TX: u8 = 19;
const IO4: usize = 4;
const IO5: usize = 5;

// initialization of fpioa is performed only at the beginning of the SBI, 
// thus we don't create an instance of it 
pub fn init() {
	unsafe {
		let fpioa = pac::FPIOA::ptr();

		// map UARTHS_rx and UARTHS_tx to io4 and io5
		(*fpioa).io[IO4].write(|w| {
			w.ch_sel().bits(FUNC_UARTHS_RX)
				.ie_en().set_bit()
				.st().set_bit()
		});
		(*fpioa).io[IO5].write(|w| {
			w.ch_sel().bits(FUNC_UARTHS_TX)
				.ds().bits(0xf)
				.oe_en().set_bit()
		});
	}
}