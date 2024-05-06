#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::arch::VirtualAddress;
#[rustfmt::skip]
#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::{
  gdt::init_gdt,
  interrupt::init_idt,
};
use crate::arch::x86_64::active_level_4_table;
use crate::arch::x86_64::hw::qemu;
use crate::arch::x86_64::hw::qemu::QemuExitCode;

pub mod alloc;
pub mod arch;
pub mod ipc;
pub mod mem;
pub mod proc;
pub mod support;
pub mod syscall;
pub mod vfs;
pub mod vga;

mod log;
#[cfg(test)]
mod test;

/// Panic handler must be implemented manually if using `no_std`.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  println!("[FATAL  ] {}", info);
  loop {}
}

/// Panic handler must be implemented manually if using `no_std`.
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  println!("[FATAL  ] {}", info);
  qemu::exit_qemu(QemuExitCode::Failed);
  loop {
    unsafe {
      core::arch::asm!("hlt");
    }
  }
}

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
  crate::log::init();

  println!("[INFO   ] Kernel is booting.");

  // Prepare memory page tables.
  // let page_table = unsafe { PageTableImpl::from_frame() };
  println!("[DEBUG  ] Initialize page table.");
  let page_table = unsafe { active_level_4_table(VirtualAddress::new(0x0000)) };

  // Initialize the heap allocator.
  let heap = unsafe {};

  // Initialize the GDT and IDT.
  // Replace the GDT and IDT as soon as we can, instead of UEFI.
  init_gdt();
  init_idt();

  // Parse the static ACPI tables.

  // Initialize PCI.

  // Start scheduler.

  // Run test.
  #[cfg(test)]
  test_main();

  loop {
    println!("[TRACE  ] hlt");
    unsafe {
      core::arch::asm!("hlt");
    }
  }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
  println!("Running {} tests", tests.len());
  for test in tests {
    test();
  }
  // Exit qemu in test mode.
  crate::arch::x86_64::hw::qemu::exit_qemu(QemuExitCode::Success);
}
