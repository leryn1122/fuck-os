use super::port::Port;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum QemuExitCode {
  Success = 0x10,
  Failed = 0x11,
}

#[cfg(test)]
pub fn exit_qemu(exit_code: QemuExitCode) {
  #[cfg(target_arch = "x86_64")]
  unsafe {
    let mut port = Port::new(0xF4);
    port.write(exit_code as u32);
  }

  #[cfg(target_arch = "aarch64")]
  unsafe {}
}
