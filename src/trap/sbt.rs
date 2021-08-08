// Software Binary Translation
// For handling some old-spec instructions, which are invalid on newer platform.
// The basic idea is that, when an Invalid Instruction Exception is raised, 
// extract it from memory, and reform the instruction into a valid one, 
// and write it back to the memory.

// retrhelo: 
// This part of codes may differ among platforms. But as I only have a k210 board 
// I'll work on it only. 
// I'd thanks to luojia65, for his brilliant work onk210 platform in RustSBI, 
// where PsicaSBI is inspired. Codes below heavily depends on his previous work. 

use super::TrapFrame;

use riscv::register::{
	satp, mepc, 
};

pub(super) fn translation(vaddr: usize, tf: &mut TrapFrame) ->bool {
	let ins = unsafe {extract_inst(vaddr)};

	if ins & 0xFE007FFF == 0x12000073 { // sfence.vma instruction
		// There is no `sfence.vma` in 1.9.1 privileged spec; however there is a `sfence.vm`.
		// For backward compability, here we emulate the first instruction using the second one.
		// sfence.vma: | 31..25 funct7=SFENCE.VMA(0001001) | 24..20 rs2/asid | 19..15 rs1/vaddr | 
		//               14..12 funct3=PRIV(000) | 11..7 rd, =0 | 6..0 opcode=SYSTEM(1110011) |
		// sfence.vm(1.9):  | 31..=20 SFENCE.VM(000100000100) | 19..15 rs1/vaddr |
		//               14..12 funct3=PRIV(000) | 11..7 rd, =0 | 6..0 opcode=SYSTEM(1110011) |
		// discard rs2 // let _rs2_asid = ((ins >> 20) & 0b1_1111) as u8;
		// let rs1_vaddr = ((ins >> 15) & 0b1_1111) as u8;
		// read paging mode from satp (sptbr)
		let satp_bits = satp::read().bits();
		// bit 63..20 is not readable and writeable on K210, so we cannot
		// decide paging type from the 'satp' register.
		// that also means that the asid function is not usable on this chip.
		// we have to fix it to be Sv39.
		let ppn = satp_bits & 0xFFF_FFFF_FFFF; // 43..0 PPN WARL
		// write to sptbr
		let sptbr_bits = ppn & 0x3F_FFFF_FFFF;
		// unsafe { llvm_asm!("csrw 0x180, $0"::"r"(sptbr_bits)) }; // write to sptbr
		unsafe {
			asm!("csrw 0x180, {0}", in(reg) sptbr_bits);
		}
		// enable paging (in v1.9.1, mstatus: | 28..24 VM[4:0] WARL | ... )
		let mut mstatus_bits: usize; 
		// unsafe { llvm_asm!("csrr $0, mstatus":"=r"(mstatus_bits)) };
		unsafe {
			asm!("csrr {0}, mstatus", out(reg) mstatus_bits);
		}
		mstatus_bits &= !0x1F00_0000;
		mstatus_bits |= 9 << 24; 
		// unsafe { llvm_asm!("csrw mstatus, $0"::"r"(mstatus_bits)) };
		unsafe {
			asm!("csrw mstatus, {0}", in(reg) mstatus_bits);
		}
		// emulate with sfence.vm (declared in privileged spec v1.9)
		// unsafe { llvm_asm!(".word 0x10400073") }; // sfence.vm x0
		unsafe {
			asm!(".word 0x10400073");	// sfence.vm x0
		}
		// ::"r"(rs1_vaddr)
		mepc::write(mepc::read().wrapping_add(4)); // skip current instruction

		true 
	}
	else if ins & 0xFFFFF07F == 0xC0102073 { // rdtime instruction
		// rdtime is actually a csrrw instruction
		let rd = ((ins >> 7) & 0b1_1111) as u8;
		let mtime = crate::hal::clint::get_mtime();
		let time_usize = mtime as usize;
		set_rd(tf, rd, time_usize);
		mepc::write(mepc::read().wrapping_add(4)); // skip current instruction 

		true
	}
	else {false}
}

/// Extract Instruction from memory. Remember, when doing this, the vaddr must 
/// be a valid address. This turns quite important after Virtual Memory Mapping 
/// is enabled. 
unsafe fn extract_inst(vaddr: usize) ->u32 {
	let low = get_vaddr_u16(vaddr) as u32;
	let high = get_vaddr_u16(vaddr + 2) as u32;

	low | (high << 16)
}

#[inline]
unsafe fn get_vaddr_u16(vaddr: usize) ->u16 {
	let mut ans: u16;
	asm!(
		"li {0}, (1 << 17)", 
		"csrrs {0}, mstatus, {0}", 
		"lhu {1}, 0({2})", 
		"csrw mstatus, {0}", 
		out(reg) _, 
		out(reg) ans, 
		in(reg) vaddr, 
	);
	ans
}

#[inline]
fn set_rd(trap_frame: &mut TrapFrame, rd: u8, value: usize) {
    match rd {
        10 => trap_frame.a0 = value as i64,
        11 => trap_frame.a1 = value as i64,
        12 => trap_frame.a2 = value as i64,
        13 => trap_frame.a3 = value as i64,
        14 => trap_frame.a4 = value as i64,
        15 => trap_frame.a5 = value as i64,
        16 => trap_frame.a6 = value as i64,
        17 => trap_frame.a7 = value as i64,
        5  => trap_frame.t0 = value as i64,
        6  => trap_frame.t1 = value as i64,
        7  => trap_frame.t2 = value as i64,
        28 => trap_frame.t3 = value as i64,
        29 => trap_frame.t4 = value as i64,
        30 => trap_frame.t5 = value as i64,
        31 => trap_frame.t6 = value as i64,
        _ => panic!("invalid target `rd`"),
    }
}