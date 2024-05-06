use core::arch::asm;
use core::marker::PhantomData;

pub type PortNumber = u16;

pub trait PortSize: PortRead + PortWrite {}

pub trait PortRead {
  unsafe fn read_from_port(port: PortNumber) -> Self;
}

pub trait PortWrite {
  unsafe fn write_to_port(port: PortNumber, value: Self);
}

pub struct Port<S: PortSize> {
  port: PortNumber,
  size: PhantomData<S>,
}

impl<S: PortSize> Port<S> {
  pub const unsafe fn new(port: PortNumber) -> Self {
    Self {
      port,
      size: PhantomData,
    }
  }

  pub unsafe fn read(&self) -> S {
    unsafe { S::read_from_port(self.port) }
  }

  pub unsafe fn write(&self, value: S) {
    unsafe {
      S::write_to_port(self.port, value);
    }
  }
}

mod _impl {
  use core::arch::asm;

  use super::PortNumber;
  use super::PortRead;
  use super::PortSize;
  use super::PortWrite;

  impl PortSize for u8 {}

  impl PortSize for u16 {}

  impl PortSize for u32 {}

  impl PortRead for u8 {
    #[inline]
    unsafe fn read_from_port(port: PortNumber) -> u8 {
      let result: u8;
      unsafe {
        asm!("in al, dx", out("al") result, in("dx") port, options(nomem, nostack, preserves_flags));
      }
      result
    }
  }

  impl PortRead for u16 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u16 {
      let result: u16;
      unsafe {
        asm!("in ax, dx", out("ax") result, in("dx") port, options(nomem, nostack, preserves_flags));
      }
      result
    }
  }

  impl PortRead for u32 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u32 {
      let result: u32;
      unsafe {
        asm!("in eax, dx", out("eax") result, in("dx") port, options(nomem, nostack, preserves_flags));
      }
      result
    }
  }

  impl PortWrite for u8 {
    #[inline]
    unsafe fn write_to_port(port: PortNumber, value: Self) {
      unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
      }
    }
  }

  impl PortWrite for u16 {
    #[inline]
    unsafe fn write_to_port(port: PortNumber, value: Self) {
      unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
      }
    }
  }

  impl PortWrite for u32 {
    #[inline]
    unsafe fn write_to_port(port: PortNumber, value: Self) {
      unsafe {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
      }
    }
  }
}
