use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::println;

pub fn init_idt() {
  println!("[INFO ] Initialize IDT.");
  // IDT.load();
}

/// Enable interrupts.
///
/// x86_64 assemble instruction `STI` to enable the interrupts.
///
/// ```asm
/// # Set interrupt flag
/// sti
/// ```
///
/// ## Safety
#[inline(always)]
pub unsafe fn enable() {
  unsafe {
    core::arch::asm!("sti", options(preserves_flags, nostack));
  }
}

/// Disable interrupts.
///
/// x86_64 assemble instruction `CLI` to disable the interrupts.
///
/// ```asm
/// # Clear the interrupt flag
/// cli
/// ```
///
/// ## Safety
#[inline(always)]
pub unsafe fn disable() {
  unsafe {
    core::arch::asm!("cli", options(preserves_flags, nostack));
  }
}

/// Halt instruction.
///
/// ## Safety
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
