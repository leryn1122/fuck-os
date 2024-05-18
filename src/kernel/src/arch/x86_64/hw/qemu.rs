use super::port::Port;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum QemuExitCode {
  Success = 0x10,
  Failed = 0x11,
}

pub(crate) struct ExitPort(Port<u32>);

impl ExitPort {
  pub unsafe fn new() -> Self {
    Self(unsafe { Port::new(0xF4) })
  }

  pub fn exit(&mut self, code: QemuExitCode) -> ! {
    unsafe {
      self.0.write(code as u32);
    }
    unreachable!()
  }
}

#[cfg(test)]
pub fn exit_qemu(exit_code: QemuExitCode) {
  #[cfg(target_arch = "x86_64")]
  unsafe {
    let mut port = ExitPort::new();
    port.0.write(exit_code as u32);
  }

  #[cfg(target_arch = "aarch64")]
  unsafe {}
}
