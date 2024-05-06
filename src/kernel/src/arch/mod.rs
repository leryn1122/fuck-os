#[cfg(target_arch = "x86")]
pub mod x86;

use core::marker::PhantomData;
use core::ops::Add;

#[cfg(target_arch = "x86")]
use self::x86::*;
use crate::arch::x86_64::paging::PageSize;
use crate::arch::x86_64::paging::Size4KiB;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "aarch64")]
use self::aarch64::*;

#[macro_use]
pub mod macros;
pub mod shared;

pub struct AddressNotAligned;

#[inline]
const fn align_down(address: usize, align: usize) -> usize {
  debug_assert!(align.is_power_of_two(), "`align` must be a power of two");
  address & !(align - 1)
}

#[inline]
const fn align_up(address: usize, align: usize) -> usize {
  debug_assert!(align.is_power_of_two(), "`align` must be a power of two");
  address & !(align)
}

/// Physical address.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
  #[inline]
  pub const fn new(address: usize) -> Self {
    Self(address)
  }

  #[inline]
  pub const fn data(&self) -> usize {
    self.0
  }

  #[cfg(target_pointer_width = "64")]
  #[inline]
  pub const fn as_u64(&self) -> u64 {
    self.0 as u64
  }

  #[cfg(target_pointer_width = "32")]
  #[inline]
  pub const fn as_u32(&self) -> u32 {
    self.0 as u32
  }

  #[inline]
  fn align_down<S>(self, align: S) -> Self
  where
    S: Into<usize>,
  {
    PhysicalAddress::new(align_down(self.0, align.into()))
  }

  #[inline]
  fn align_up<S>(self, align: S) -> Self
  where
    S: Into<usize>,
  {
    PhysicalAddress::new(align_up(self.0, align.into()))
  }

  fn is_aligned<S>(self, align: S) -> bool
  where
    S: Into<usize>,
  {
    self.align_down(align) == self
  }
}

impl core::fmt::Debug for PhysicalAddress {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_tuple("PhysicalAddress").field(&format_args!("{:#x}", self.0)).finish()
  }
}

/// Virtual address.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualAddress(pub(crate) usize);

impl VirtualAddress {
  #[inline]
  pub const fn new(address: usize) -> Self {
    Self(address)
  }

  #[cfg(target_pointer_width = "64")]
  #[inline]
  pub const fn as_u64(&self) -> u64 {
    self.0 as u64
  }

  #[cfg(target_pointer_width = "32")]
  #[inline]
  pub const fn as_u32(&self) -> u32 {
    self.0 as u32
  }

  #[cfg(target_pointer_width = "64")]
  #[inline]
  pub fn from_ptr<T: ?Sized>(ptr: *const T) -> Self {
    Self::new(ptr as *const () as usize)
  }

  #[inline]
  pub const fn as_ptr<T>(self) -> *const T {
    self.as_u64() as *const T
  }

  #[inline]
  pub const fn as_mut_ptr<T>(self) -> *mut T {
    self.as_ptr::<T>() as *mut T
  }

  // #[inline(always)]
  // pub fn kind(&self) -> MemoryKind {
  //   if (self.0 as isize) < 0 {
  //     MemoryKind::Kernel
  //   } else {
  //     MemoryKind::Userspace
  //   }
  // }
}

impl Add<u64> for VirtualAddress {
  type Output = Self;

  #[inline]
  fn add(self, rhs: u64) -> Self::Output {
    Self::new(self.0 + rhs as usize)
  }
}

impl core::fmt::Debug for VirtualAddress {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_tuple("VirtualAddress").field(&format_args!("{:#x}", self.0)).finish()
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysicalFrame<S: PageSize = Size4KiB> {
  address: PhysicalAddress,
  size:    PhantomData<S>,
}

impl<S: PageSize> PhysicalFrame<S> {
  pub fn start_with(address: PhysicalAddress) -> Result<Self, AddressNotAligned> {
    if !address.is_aligned(S::SIZE) {
      return Err(AddressNotAligned);
    }
    Ok(Self {
      address,
      size: PhantomData,
    })
  }

  pub fn start_address(&self) -> PhysicalAddress {
    PhysicalAddress::new(self.address.data())
  }

  pub fn containing_address(address: PhysicalAddress) -> Self {
    Self {
      address: address.align_down(S::SIZE),
      size:    PhantomData,
    }
  }
}

impl<S: PageSize> core::fmt::Debug for PhysicalFrame<S> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(
      f,
      "Frame[{}]({:#x})",
      S::DEBUG_STR,
      self.start_address().as_u64()
    )
  }
}
