use core::arch::asm;

use crate::arch::PhysicalAddress;
use crate::arch::PhysicalFrame;

pub struct CR3;

impl CR3 {
  pub fn read() -> (PhysicalFrame, u16) {
    let value: u64;

    unsafe {
      asm!("mov {}, cr3", out(reg) value, options(nomem, nostack, preserves_flags));
    }

    let address = PhysicalAddress::new((value & 0x_000F_FFFF_FFFF_F000) as usize);
    let frame = PhysicalFrame::containing_address(address);
    (frame, (value & 0xFFF) as u16)
  }
}
