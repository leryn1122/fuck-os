use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::println;

lazy_static! {
  static ref IDT: InterruptDescriptorTable = {
    let idt = InterruptDescriptorTable::new();
    idt
  };
}

pub fn init_idt() {
  println!("[INFO   ] Initialize IDT.");
  IDT.load();
}

/// Halt instruction.
/// 
/// # Safety
/// 
#[inline(always)]
pub unsafe fn halt() {
  core::arch::asm!("hlt", options(nomem, nostack));
}

/// Pause instruction.
#[inline(always)]
pub fn pause() {
  unsafe {
    core::arch::asm!("pause", options(nomem, nostack));
  }
}
