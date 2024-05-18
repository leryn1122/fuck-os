use core::marker::PhantomData;
use core::ops::Add;

use crate::arch::x86_64::paging::PageSize;
use crate::arch::x86_64::paging::Size4KiB;

// For aarch64
#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use self::aarch64::*;

// For x86
#[cfg(target_arch = "x86")]
pub mod x86;
#[cfg(target_arch = "x86")]
pub use self::x86::*;

// For x86_64
#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

pub mod shared;

#[macro_use]
pub mod macros;

pub struct AddressNotAligned;

#[cfg(target_pointer_width = "64")]
pub type PtrWidth = u64;
#[cfg(target_pointer_width = "32")]
pub type PtrWidth = u32;

#[inline]
const fn align_down(address: PtrWidth, align: PtrWidth) -> PtrWidth {
  debug_assert!(align.is_power_of_two(), "`align` must be a power of two");
  address & !(align - 1)
}

#[inline]
const fn align_up(address: PtrWidth, align: PtrWidth) -> PtrWidth {
  debug_assert!(align.is_power_of_two(), "`align` must be a power of two");
  address & !(align)
}

/// # Physical address
///
/// Raw address.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress(PtrWidth);

impl PhysicalAddress {
  #[inline]
  pub const fn new(address: PtrWidth) -> Self {
    Self(address)
  }

  #[inline]
  pub const fn data(&self) -> PtrWidth {
    self.0
  }

  #[cfg(target_pointer_width = "64")]
  #[inline]
  pub const fn as_raw(&self) -> u64 {
    self.0
  }

  #[cfg(target_pointer_width = "32")]
  #[inline]
  pub const fn as_raw(&self) -> u32 {
    self.0
  }

  #[inline]
  fn align_down<S>(self, align: S) -> Self
  where
    S: Into<PtrWidth>,
  {
    Self::new(align_down(self.0, align.into()))
  }

  #[inline]
  fn align_up<S>(self, align: S) -> Self
  where
    S: Into<PtrWidth>,
  {
    Self::new(align_up(self.0, align.into()))
  }

  #[inline]
  fn is_aligned<S>(self, align: S) -> bool
  where
    S: Into<PtrWidth>,
  {
    self.align_down(align) == self
  }
}

impl core::fmt::Debug for PhysicalAddress {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_tuple("PhysicalAddress").field(&format_args!("{:#x}", self.0)).finish()
  }
}

impl Add<PtrWidth> for PhysicalAddress {
  type Output = Self;

  fn add(self, rhs: PtrWidth) -> Self::Output {
    Self::new(self.0 + rhs)
  }
}

/// Virtual address.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualAddress(pub(crate) PtrWidth);

impl VirtualAddress {
  #[inline]
  pub const fn new(address: PtrWidth) -> Self {
    Self(address)
  }

  #[cfg(target_pointer_width = "64")]
  #[inline]
  pub const fn as_raw(&self) -> PtrWidth {
    self.0
  }

  #[cfg(target_pointer_width = "32")]
  #[inline]
  pub const fn as_u32(&self) -> PtrWidth {
    self.0
  }

  #[cfg(target_pointer_width = "64")]
  #[inline]
  pub fn from_ptr<T: ?Sized>(ptr: *const T) -> Self {
    Self::new(ptr as *const () as PtrWidth)
  }

  #[inline]
  pub const fn as_ptr<T>(self) -> *const T {
    self.as_raw() as *const T
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

impl Add<PtrWidth> for VirtualAddress {
  type Output = Self;

  #[inline]
  fn add(self, rhs: PtrWidth) -> Self::Output {
    Self::new(self.0 + rhs)
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
      self.start_address().as_raw()
    )
  }
}

pub enum FrameError {
  FrameNotPresent,
  HugePage,
}

/// Translate the given virtual address into the physical address. If not mapped, return `None`.
pub unsafe fn translate_address(
  address: VirtualAddress,
  physical_memory_offset: VirtualAddress,
) -> Option<PhysicalAddress> {
  translate_address_inner(address, physical_memory_offset)
}
