use core::panic::PanicInfo;

use crate::println;

/// Panic handler must be implemented manually if using `no_std`.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("[FATAL  ] {}", info);
  loop {
    unsafe {
      crate::arch::interrupt::halt();
    }
  }
}

/// Panic handler must be implemented manually if using `no_std`.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  use crate::arch::x86_64::hw::qemu;

  println!("[FATAL  ] {}", info);
  qemu::exit_qemu(qemu::QemuExitCode::Failed);
  loop {
    unsafe {
      crate::arch::interrupt::halt();
    }
  }
}
