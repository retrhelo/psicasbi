// M-mode trap handling

mod sbi;
mod sbt;

/// This vector is used when trap from S/U mode
#[naked]
#[no_mangle]
#[repr(align(4))]
unsafe extern "C" fn trap_vec() {
	// asm codes below are from xv6-k210's kerneltrap, 
	// which stores contexts on stack
	// mscratch stores the stack top of current hart
	asm!("
		csrrw sp, mscratch, sp
		addi sp, sp, -256

		sd ra, 0(sp)
		sd gp, 16(sp)
		sd tp, 24(sp)
		sd t0, 32(sp)
		sd t1, 40(sp)
		sd t2, 48(sp)
		sd s0, 56(sp)
		sd s1, 64(sp)
		sd a0, 72(sp)
		sd a1, 80(sp)
		sd a2, 88(sp)
		sd a3, 96(sp)
		sd a4, 104(sp)
		sd a5, 112(sp)
		sd a6, 120(sp)
		sd a7, 128(sp)
		sd s2, 136(sp)
		sd s3, 144(sp)
		sd s4, 152(sp)
		sd s5, 160(sp)
		sd s6, 168(sp)
		sd s7, 176(sp)
		sd s8, 184(sp)
		sd s9, 192(sp)
		sd s10, 200(sp)
		sd s11, 208(sp)
		sd t3, 216(sp)
		sd t4, 224(sp)
		sd t5, 232(sp)
		sd t6, 240(sp)

		csrr a0, mscratch
		sd a0, 8(sp)

		mv a0, sp
		call trap_handler

		ld ra, 0(sp)
		ld gp, 16(sp)
		ld t0, 32(sp)
		ld t1, 40(sp)
		ld t2, 48(sp)
		ld s0, 56(sp)
		ld s1, 64(sp)
		ld a0, 72(sp)
		ld a1, 80(sp)
		ld a2, 88(sp)
		ld a3, 96(sp)
		ld a4, 104(sp)
		ld a5, 112(sp)
		ld a6, 120(sp)
		ld a7, 128(sp)
		ld s2, 136(sp)
		ld s3, 144(sp)
		ld s4, 152(sp)
		ld s5, 160(sp)
		ld s6, 168(sp)
		ld s7, 176(sp)
		ld s8, 184(sp)
		ld s9, 192(sp)
		ld s10, 200(sp)
		ld s11, 208(sp)
		ld t3, 216(sp)
		ld t4, 224(sp)
		ld t5, 232(sp)
		ld t6, 240(sp)

		addi sp, sp, 256
		csrrw sp, mscratch, sp

		mret
	", options(noreturn));
}

#[naked]
#[no_mangle]
#[repr(align(4))]
unsafe extern "C" fn sbi_trap_vec() {
	asm!("
		addi sp, sp, -256

		sd ra, 0(sp)
		sd gp, 16(sp)
		sd tp, 24(sp)
		sd t0, 32(sp)
		sd t1, 40(sp)
		sd t2, 48(sp)
		sd s0, 56(sp)
		sd s1, 64(sp)
		sd a0, 72(sp)
		sd a1, 80(sp)
		sd a2, 88(sp)
		sd a3, 96(sp)
		sd a4, 104(sp)
		sd a5, 112(sp)
		sd a6, 120(sp)
		sd a7, 128(sp)
		sd s2, 136(sp)
		sd s3, 144(sp)
		sd s4, 152(sp)
		sd s5, 160(sp)
		sd s6, 168(sp)
		sd s7, 176(sp)
		sd s8, 184(sp)
		sd s9, 192(sp)
		sd s10, 200(sp)
		sd s11, 208(sp)
		sd t3, 216(sp)
		sd t4, 224(sp)
		sd t5, 232(sp)
		sd t6, 240(sp)

		add a0, sp, 256
		sd a0, 8(sp)

		mv a0, sp
		call trap_handler

		ld ra, 0(sp)
		ld gp, 16(sp)
		ld t0, 32(sp)
		ld t1, 40(sp)
		ld t2, 48(sp)
		ld s0, 56(sp)
		ld s1, 64(sp)
		ld a0, 72(sp)
		ld a1, 80(sp)
		ld a2, 88(sp)
		ld a3, 96(sp)
		ld a4, 104(sp)
		ld a5, 112(sp)
		ld a6, 120(sp)
		ld a7, 128(sp)
		ld s2, 136(sp)
		ld s3, 144(sp)
		ld s4, 152(sp)
		ld s5, 160(sp)
		ld s6, 168(sp)
		ld s7, 176(sp)
		ld s8, 184(sp)
		ld s9, 192(sp)
		ld s10, 200(sp)
		ld s11, 208(sp)
		ld t3, 216(sp)
		ld t4, 224(sp)
		ld t5, 232(sp)
		ld t6, 240(sp)

		addi sp, sp, 256

		mret
	", options(noreturn));
}

#[repr(C)]
#[derive(Debug)]
struct TrapFrame {
	ra: i64, 
	sp: i64, 
	gp: i64, 
	tp: i64, 
	t0: i64, 
	t1: i64, 
	t2: i64, 
	s0: i64, 
	s1: i64, 
	a0: i64, 
	a1: i64, 
	a2: i64, 
	a3: i64, 
	a4: i64, 
	a5: i64, 
	a6: i64, 
	a7: i64, 
	s2: i64, 
	s3: i64, 
	s4: i64, 
	s5: i64, 
	s6: i64, 
	s7: i64, 
	s8: i64, 
	s9: i64, 
	s10: i64, 
	s11: i64, 
	t3: i64, 
	t4: i64, 
	t5: i64, 
	t6: i64, 
}

use riscv::register::{
	mcause, 
	mcause::{Trap, Interrupt, Exception}, 
	mepc, mtval, 
};

#[no_mangle]
extern "C" fn trap_handler(tf: &mut TrapFrame) {
	let cause = mcause::read().cause();

	match cause {
		Trap::Exception(Exception::SupervisorEnvCall) => {
			// let mepc = mepc::read().wrapping_add(4);
			// unsafe {
			// 	// install trap vec for SBI, to support nested trap
			// 	mtvec::write(sbi_trap_vec as usize, mtvec::TrapMode::Direct);
			// 	mstatus::set_mie();
			// }
			sbi::handler(tf);
			mepc::write(mepc::read().wrapping_add(4));
			// unsafe {
			// 	// restore trap vec 
			// 	mstatus::clear_mie();
			// 	mepc::write(mepc);
			// 	mtvec::write(trap_vec as usize, mtvec::TrapMode::Direct);
			// }
		}, 
		Trap::Interrupt(Interrupt::MachineTimer) => {
			// println!("hart {} MachineTimer", mhartid::read());
			// delegate to supervisor 
			unsafe {
				mip::set_stimer();
				mie::clear_mtimer();
			}
		}, 
		Trap::Interrupt(Interrupt::MachineSoft) => {
			//println!("hart {} MachineSoft", mhartid::read());
			unsafe {
				mip::set_ssoft();
			}
			// Software Interrupt at Machine Mode can only be cleared by 
			// setting CLINT, but in Supervisor Mode, it can be done by clearing 
			// SSIP in sip
			let hartid = riscv::register::mhartid::read();
			crate::hal::clint::clear_ipi(hartid);
		}, 
		Trap::Interrupt(Interrupt::MachineExternal) => {
			//println!("hart {} MachineExternal", mhartid::read());
			match () {
				#[cfg(feature = "soft-extern")]
				() => {
					// This setting is for those which don't implement S-mode 
					// external interrupts, like k210. If your hardware does have 
					// Supervisor External Interrupt, then delegate it to S-mode 
					// may be a better solution. 
					// For the sake of k210, we deliver a software interrupt to S-mode 
					// kernel, so that the kernel can deal the it in S-mode. And after 
					// the interrupt is "complete", the kernel should re-trap into 
					// SBI to enable MEIP, which can be done by SBI function sbi_xv6_set_ext()
					unsafe {
						mie::clear_mext();
						// disable mtimer because we don't want to be interrupted
						// this may not be necessary
						mie::clear_mtimer();

						// set S-mode software interrupt 
						mip::set_ssoft();
					}
				}, 
				#[cfg(not(feature = "soft-extern"))]
				() => {
					// do nothing here
				}, 
			}
		},
		#[cfg(feature = "old-spec")]
		Trap::Exception(Exception::IllegalInstruction) => {
			let vaddr = mepc::read();

			if sbt::translation(vaddr, tf) {}
			else {
				panic!(
					"Illegal Instruction, mepc: {:016x?}\ntrap frame: {:x?}", 
					vaddr, 
					tf
				);
			}
		}, 
		_ => {
			panic!(
				"Unhandled exception! mcause: {:?}, mepc: {:016x?}, mtval: {:016x?}\ntrap frame: {:p}\n{:x?}",
				cause,
				mepc::read(),
				mtval::read(),
				&tf as *const _,
				tf
			);
		}, 
	}
}

use riscv::register::{
	mstatus, mtvec, mie, mip, 
	mhartid, 
};

// should run on every hart
pub fn init() {
	// install trap vector 
	unsafe {
		mtvec::write(
			sbi_trap_vec as usize, 
			mtvec::TrapMode::Direct
		);
	}

	// delegate traps
	unsafe {
		// somehow we can't set medeleg via riscv crate
		asm!("
			li {0}, 0x222
			csrw mideleg, {0}
			li {0}, 0xb1ab
			csrw medeleg, {0}
		", out(reg) _);
	}

	// enable interrupts
	unsafe {
		crate::hal::clint::clear_ipi(
			mhartid::read()
		);
		mie::set_mext();
		mie::set_msoft();
		// we don't enable mtimer because it would be enabled 
		// through SBI call set_timer()
		// And enable mtimer may introduce some problem in kernel, 
		// like a mtimer interrupt before kernel's trapvec is installed.
	}
}

use mstatus::MPP;
use crate::config::KERNEL_ENTRY;

pub fn enter_supervisor(hartid: usize) {
	unsafe {
		// disable trap to switch trap vector 
		mstatus::clear_mie();

		mtvec::write(trap_vec as usize, mtvec::TrapMode::Direct);

		// set previous mode as supervisor 
		mstatus::set_mpp(MPP::Supervisor);
		mepc::write(KERNEL_ENTRY);

		asm!(
			"csrw mscratch, sp", 
			"mret", 
			in("a0") hartid, 
			options(noreturn)
		);
	}
}