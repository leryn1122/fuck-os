#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod alloc;
pub mod arch;
pub mod ipc;
pub mod mem;
pub mod proc;
pub mod syscall;
pub mod vfs;
pub mod vga;

#[cfg(test)]
mod test;

/// Panic handler must be implemented manually if using `no_std`.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

/// Panic handler must be implemented manually if using `no_std`.
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  println!("{}", info);
  exit_qemu(QemuExitCode::Failed);
  loop {}
}

static HELLO: &[u8] = b"Hello World!";

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World!");

  #[cfg(test)]
  test_main();

  loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
  println!("Running {} tests", tests.len());
  for test in tests {
    test();
  }
  // Exit qemu in test mode.
  exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
  Success = 0x10,
  Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
  use x86_64::instructions::port::Port;
  unsafe {
    let mut port = Port::new(0xf4);
    port.write(exit_code as u32);
  }
}
