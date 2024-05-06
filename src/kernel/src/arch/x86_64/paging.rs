use bitflags::bitflags;

use crate::arch::PhysicalAddress;

/// Count of entries in the page table.
const ENTRY_COUNT: usize = 512;

pub trait PageSize {
  const SIZE: usize;

  #[cfg(debug_assertions)]
  const DEBUG_STR: &'static str;
}

pub struct Size4KiB;

//noinspection RsSortImplTraitMembers
impl PageSize for Size4KiB {
  const DEBUG_STR: &'static str = "4KiB";
  const SIZE: usize = 4096;
}

/// # CPU Page Table
#[derive(Clone)]
#[repr(C)]
#[repr(align(4096))]
pub struct PageTable {
  entries: [PageTableEntry; 512],
}

impl PageTable {
  #[inline]
  pub const fn new() -> Self {
    const EMPTY: PageTableEntry = PageTableEntry::new();
    Self {
      entries: [EMPTY; ENTRY_COUNT],
    }
  }

  #[inline]
  pub fn iter(&self) -> impl Iterator<Item = &PageTableEntry> {
    (0..ENTRY_COUNT).map(move |i| &self.entries[i])
  }

  #[inline]
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PageTableEntry> {
    let ptr = self.entries.as_mut_ptr();
    (0..ENTRY_COUNT).map(move |i| unsafe { &mut *ptr.add(i) })
  }

  /// True if all the table entries is unused.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.iter().all(|entry| entry.is_unused())
  }

  #[inline]
  pub fn zero(&mut self) {
    for entry in self.iter_mut() {
      entry.set_unused();
    }
  }
}

impl Default for PageTable {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl core::ops::Index<usize> for PageTable {
  type Output = PageTableEntry;

  fn index(&self, index: usize) -> &Self::Output {
    &self.entries[index]
  }
}

impl core::ops::IndexMut<usize> for PageTable {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.entries[index]
  }
}

impl core::fmt::Debug for PageTable {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    self.entries[..].fmt(f)
  }
}

/// A 64-bit page table entry.
#[derive(Clone)]
#[repr(transparent)]
pub struct PageTableEntry {
  entry: u64,
}

impl PageTableEntry {
  #[inline]
  pub const fn new() -> Self {
    Self { entry: 0 }
  }

  /// True if the entry is unused.
  #[inline]
  pub const fn is_unused(&self) -> bool {
    self.entry == 0
  }

  #[inline]
  pub fn set_unused(&mut self) {
    self.entry = 0
  }

  #[inline]
  pub const fn flags(&self) -> PageTableFlags {
    PageTableFlags::from_bits_truncate(self.entry)
  }

  /// Returns the physical address
  #[inline]
  pub fn address(&self) -> PhysicalAddress {
    PhysicalAddress::new((self.entry & 0x000F_FFFF_FFFF_F000) as usize)
  }

  #[inline]
  pub fn set_address(&mut self, address: PhysicalAddress, flags: PageTableFlags) {
    self.entry = address.as_u64() | flags.bits();
  }

  #[inline]
  pub fn set_flags(&mut self, flags: PageTableFlags) {
    self.entry = self.address().as_u64() | flags.bits()
  }
}

impl Default for PageTableEntry {
  fn default() -> Self {
    Self::new()
  }
}

impl core::fmt::Debug for PageTableEntry {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut f = f.debug_struct("PageTableEntry");
    f.field("addr", &self.address());
    f.field("flags", &self.flags());
    f.finish()
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PageTableIndex(u16);

impl PageTableIndex {}

bitflags! {
  #[rustfmt::skip]
  #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
  struct PageTableFlags: u64 {
    const PRESENT               = 1 << 0;
    const WRITABLE              = 1 << 1;
    const USER_ACCESSIBLE       = 1 << 2;
    const WRITE_THROUGH_CACHING = 1 << 3;
    const DISABLE_CACHE         = 1 << 4;
    const ACCESSED              = 1 << 5;
    const DIRTY                 = 1 << 6;
    const HUGE_PAGE             = 1 << 7;
    const GLOBAL                = 1 << 8;
    const NO_EXECUTE            = 1 << 63;
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum PageTableLevel {
  One = 1,
  Two,
  Three,
  Four,
}

impl PageTableLevel {
  pub const fn lower(self) -> Option<Self> {
    match self {
      Self::Four => Some(Self::Three),
      Self::Three => Some(Self::Two),
      Self::Two => Some(Self::One),
      Self::One => None,
    }
  }

  pub const fn higher(self) -> Option<Self> {
    match self {
      Self::Four => None,
      Self::Three => Some(Self::Four),
      Self::Two => Some(Self::Three),
      Self::One => Some(Self::Two),
    }
  }

  pub const fn table_address_space_alignment(self) -> u64 {
    1u64 << (self as u8 * 9 + 12)
  }

  pub const fn entry_address_space_alignment(self) -> u64 {
    1u64 << (((self as u8 - 1) * 9) + 12)
  }
}
