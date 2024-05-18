/// Halt instruction
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
