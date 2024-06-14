/// Enable interrupts.
///
/// # Safety
#[inline(always)]
pub unsafe fn enable() {
  unsafe {
    core::arch::asm!("msr daifclr, #2");
  }
}

/// Disable interrupts.
///
/// # Safety
#[inline(always)]
pub unsafe fn disable() {
  unsafe { core::arch::asm!("msr daifset, #2") }
}

/// Halt instruction
///
/// # Safety
#[inline(always)]
pub unsafe fn halt() {
  core::arch::asm!("wfi");
}

/// Pause instruction
pub fn pause() {
  unsafe {
    core::arch::asm!("nop");
  }
}
