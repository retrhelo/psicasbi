// Sysctl Implementation for K210 

pub(super) struct SysCtl;

#[inline]
pub(super) fn init() ->SysCtl {
	SysCtl {}
}

use k210_pac as pac;

use crate::config::CLK;

const CLK_IN: u32 = 26_000_000;		// k210 clock freq 26MHz

impl super::SysCtlHandler for SysCtl {
	// use PLL0 as CPU freq and most of preipherals' 
	// fn set_freq(&mut self) {
	// 	let ctl = pac::SYSCTL::ptr();

	// 	unsafe {
	// 		// switch CPU to use CLK_IN
	// 		(*ctl).clk_sel0.modify(|_, w| {
	// 			w.aclk_sel().clear_bit()
	// 		});

	// 		// disable PLL0 output and power off it 
	// 		(*ctl).pll0.modify(|_, w| {
	// 			w.out_en().clear_bit()
	// 				.pwrd().clear_bit()
	// 		});
	// 	}

	// 	// calculate new values for PLL0
	// 	let (nr, od, nf) = calculate_pll_config(CLK_IN, CLK_IN);
	// 	unsafe {
	// 		(*ctl).pll0.modify(|_, w| {
	// 			w.clkr().bits(nr - 1)
	// 				.clkf().bits(nf - 1)
	// 				.clkod().bits(od - 1)
	// 				.bwadj().bits(nf - 1)
	// 		});
	// 	}

	// 	// re-power on and reset
	// 	unsafe {
	// 		(*ctl).pll0.modify(|_, w| {
	// 			w.pwrd().set_bit()
	// 		});
	// 		// wait for a moment 
	// 		for _ in 0..1000 {}

	// 		(*ctl).pll0.modify(|_, w| {
	// 			w.reset().set_bit()
	// 		});
	// 		for _ in 0..1000 {}
	// 		(*ctl).pll0.modify(|_, w| {
	// 			w.reset().clear_bit()
	// 		})
	// 	}

	// 	// wait for PLL0 to be stable 
	// 	loop {
	// 		unsafe {
	// 			let lock = (*ctl).pll_lock.read().pll_lock0().bits();

	// 			if 3 == lock {break;}	// PLL0 locked, now stable 
	// 			else {
	// 				// write 1 to pll_lock.slip_clear0 to lock PLL0 again
	// 				(*ctl).pll_lock.modify(|_, w| {
	// 					w.pll_slip_clear0().set_bit()
	// 				});
	// 			}
	// 		}
	// 	}

	// 	// enable PLL0 output and switch CPU clock to it
	// 	unsafe {
	// 		(*ctl).pll0.modify(|_, w| {
	// 			w.out_en().set_bit()
	// 		});
	// 		(*ctl).clk_sel0.modify(|_, w| {
	// 			w.aclk_sel().set_bit()
	// 		});
	// 	}

	// 	// !TODO: set APB for peripherals
	// }
	fn set_freq(&mut self) {
		let sysctl = pac::SYSCTL::ptr();

		unsafe {
			// enable apb0 clock
			(*sysctl).clk_en_cent.modify(|_, w| {
				w.apb0_clk_en().set_bit()
			});

			// enable fpioa clock
			(*sysctl).clk_en_peri.modify(|_, w| {
				w.fpioa_clk_en().set_bit()
			});
		}
	}
}


// ref: https://github.com/riscv-rust/k210-hal/
/// Accept freq_in as the input frequency,
/// and try to find a set of parameters (nr, od, nf),
/// which results in a frequency as near as possible to freq
/// note for a PLL:
///   freq_out = freq_in / nr * nf / od
/// The reason why we don't port the complex config algorithm from the
/// official C language SDK is that doing floating number arithmetics
/// efficiently in no_std rust now is currently not very convenient
fn calculate_pll_config(freq_in: u32, freq: u32) -> (u8, u8, u8) {
    // finding a set of (nr, od, nf) which minify abs(freq * nr * od - freq_in * nf)
    // nr, od is in [0b1,    0b10_000], nr * od is in [0b1,   0b100_000_000]
    // nf     is in [0b1, 0b1_000_000]

    // for archiving a higher accuracy, we want the nr * od as large as possible
    // use binary search to find the largest nr_od which freq <= freq_in * 0b1_000_000 / nr_od
    let mut left = 1;
    let mut right = 0b10_000 * 0b10_000 + 1;
    while left + 1 < right {
        let mid = (left + right) / 2;
        let max_freq = freq_in * 0b1_000_000 / mid;
        if freq >= max_freq {
            // in [left, mid)
            right = mid;
        } else {
            // in [mid, right)
            left = mid;
        }
    }
    let nr_od = left;
    // so we got nf
    let nf = freq * nr_od / freq_in;
    let nf = nf.min(0b1_000_000) as u8;

    // decompose nr_od
    for nr in 1..=0b10_000 {
        if (nr_od / nr) * nr == nr_od {
            return (nr as u8, (nr_od / nr) as u8, nf);
        }
    }
    unreachable!()
}