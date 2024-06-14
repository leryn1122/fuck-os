#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(allocator_api)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused)]

extern crate alloc;

use crate::arch::VirtualAddress;
#[rustfmt::skip]
#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::{
  gdt::init_gdt,
  interrupt::init_idt,
};
use crate::arch::x86_64::active_level_4_table;

pub mod allocator;
pub mod arch;
pub mod ipc;
pub mod log;
pub mod mem;
pub mod proc;
pub mod support;
pub mod syscall;
pub mod vfs;

mod panic;
#[cfg(test)]
mod test;

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
  log::init();

  println!("[INFO ] Kernel is booting.");

  /// Prepare and set up kernel memory page tables.
  println!("[DEBUG] Initialize page table.");
  let page_table = unsafe { active_level_4_table(VirtualAddress::new(0x0000)) };

  // Prepare and set up the heap allocator.
  // Initialize the heap through the heap allocator.
  let heap = unsafe {};

  // Initialize the GDT and IDT.
  // Replace the GDT and IDT as soon as possible, instead of ones provided by UEFI.
  // Install the exception handlers, which allows the kernel to catch and report exception
  // gracefully.
  init_gdt();
  init_idt();

  // Install a TSS for this processor. This allows us to set up per-CPU data structures.
  let tss = ();
  // let tss: Box<()> = Box::new(());
  //PerCpuImpl::install(tss);

  // Parse the static ACPI tables.

  // Initialize PCI.

  // Start scheduler.

  // Run test.
  #[cfg(test)]
  test_main();

  loop {
    println!("[TRACE] hlt");
    unsafe {
      core::arch::asm!("hlt");
    }
  }
}
