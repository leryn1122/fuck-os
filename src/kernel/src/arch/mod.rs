use core::marker::PhantomData;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;
use core::ops::SubAssign;

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

const ADDRESS_SPACE_SIZE: u64 = 0x0001_0000_0000_0000;

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
  address & !align
}

/// # Physical address
///
/// A raw physical memory address.
///
/// This is a wrapper on `u32` for 32-bit or `u64` for 64-bit architecture.
/// To avoid frequent convention between `u64` / `u32` and `usize`, it's used `PtrWidth`.
///
/// On `x86_64`, only the 52 lower bits of a physical address can be used.
/// The top 12 bits need to be zero.
///
/// Kernel MUST NOT operate the physical address directly after the initialization of page table.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress(PtrWidth);

impl PhysicalAddress {
  #[inline]
  pub const fn new(address: PtrWidth) -> Self {
    debug_assert!(address < (1 << 52));
    Self(address)
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

  #[inline]
  fn add(self, rhs: PtrWidth) -> Self::Output {
    Self::new(self.0 + rhs)
  }
}

impl AddAssign<PtrWidth> for PhysicalAddress {
  #[inline]
  fn add_assign(&mut self, rhs: PtrWidth) {
    *self = *self + rhs;
  }
}

impl Sub<PtrWidth> for PhysicalAddress {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: PtrWidth) -> Self::Output {
    PhysicalAddress::new(self.0.checked_sub(rhs).unwrap())
  }
}

impl Sub<PhysicalAddress> for PhysicalAddress {
  type Output = u64;

  #[inline]
  fn sub(self, rhs: PhysicalAddress) -> Self::Output {
    self.as_raw().checked_sub(rhs.as_raw()).unwrap()
  }
}

/// # Virtual address
///
/// This is a wrapper over an `u64`,
///
/// A canonical virtual memory address.
///
/// On `x86_64` architecture, only the 48 lower bits of a virtual address can be used.
/// The top 16 bits need to be copies of bit 47, i.e. the most significant bit.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualAddress(pub(crate) u64);

impl VirtualAddress {
  #[inline]
  pub const fn new(address: u64) -> Self {
    // TODO: Check the 16 higher bits.
    Self(address)
  }

  /// Create a virtual address that points to zero.
  #[inline]
  pub const fn zero() -> Self {
    Self(0)
  }

  /// True if the virtual address is null.
  #[inline]
  pub const fn is_null(self) -> bool {
    self.0 == 0
  }

  #[cfg(target_pointer_width = "64")]
  #[inline]
  pub const fn as_raw(&self) -> u64 {
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
    Self::new(ptr as *const () as u64)
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

impl core::fmt::Debug for VirtualAddress {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_tuple("VirtualAddress").field(&format_args!("{:#x}", self.0)).finish()
  }
}

impl Add<u64> for VirtualAddress {
  type Output = Self;

  #[inline]
  fn add(self, rhs: u64) -> Self::Output {
    Self::new(self.0 + rhs)
  }
}

impl AddAssign<u64> for VirtualAddress {
  #[inline]
  fn add_assign(&mut self, rhs: u64) {
    *self = *self + rhs;
  }
}

impl Sub<u64> for VirtualAddress {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: u64) -> Self::Output {
    Self::new(self.0.checked_sub(rhs).unwrap())
  }
}

impl SubAssign<u64> for VirtualAddress {
  #[inline]
  fn sub_assign(&mut self, rhs: u64) {
    *self = *self - rhs;
  }
}

impl Sub<VirtualAddress> for VirtualAddress {
  type Output = u64;

  #[inline]
  fn sub(self, rhs: VirtualAddress) -> Self::Output {
    self.as_raw().checked_sub(rhs.as_raw()).unwrap()
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
    PhysicalAddress::new(self.address.as_raw())
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

/// Error occurred when get a page table entry frame.
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
