use crate::println;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
  use crate::arch::x86_64::hw::qemu;

  println!("Running {} tests", tests.len());
  for test in tests {
    test();
  }
  // Exit qemu in test mode.
  qemu::exit_qemu(qemu::QemuExitCode::Success);
}
