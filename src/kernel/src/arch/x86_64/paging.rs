use bitflags::bitflags;

use crate::arch::FrameError;
use crate::arch::PhysicalAddress;
use crate::arch::PhysicalFrame;
use crate::arch::PtrWidth;
use crate::arch::VirtualAddress;

/// Count of entries in the page table.
const ENTRY_COUNT: usize = 512;

/// Page size for CPU page.
pub trait PageSize {
  const SIZE: PtrWidth;

  /// String for debug, such as `4KiB`.
  #[cfg(debug_assertions)]
  const DEBUG_STR: &'static str;
}

pub struct Size4KiB;

// noinspection RsSortImplTraitMembers
impl PageSize for Size4KiB {
  #[cfg(debug_assertions)]
  const DEBUG_STR: &'static str = "4KiB";
  const SIZE: PtrWidth = 4096;
}

impl VirtualAddress {
  #[inline]
  pub const fn page_offset(self) -> PageOffset {
    PageOffset::new_truncate(self.0 as u16)
  }
}

/// # CPU Page Table
#[derive(Clone)]
#[repr(C)]
#[repr(align(4096))]
pub struct PageTable {
  entries: [PageTableEntry; 512],
}

impl PageTable {
  /// Create a new page table.
  #[inline]
  pub const fn new() -> Self {
    const EMPTY: PageTableEntry = PageTableEntry::new();
    Self {
      entries: [EMPTY; ENTRY_COUNT],
    }
  }

  /// Return the iterator over page table entries.
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

  /// Reset the page table.
  #[inline]
  pub fn reset(&mut self) {
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
  entry: PtrWidth,
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
    PhysicalAddress::new(self.entry & 0x000F_FFFF_FFFF_F000)
  }

  #[inline]
  pub fn set_address(&mut self, address: PhysicalAddress, flags: PageTableFlags) {
    self.entry = address.as_raw() | flags.bits();
  }

  #[inline]
  pub fn set_flags(&mut self, flags: PageTableFlags) {
    self.entry = self.address().as_raw() | flags.bits()
  }

  #[inline]
  pub fn frame(&self) -> Result<PhysicalFrame, FrameError> {
    if !self.flags().contains(PageTableFlags::PRESENT) {
      Err(FrameError::FrameNotPresent)
    } else if self.flags().contains(PageTableFlags::HUGE_PAGE) {
      Err(FrameError::HugePage)
    } else {
      Ok(PhysicalFrame::containing_address(self.address()))
    }
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

/// A 12-bit offset into a 4KiB Pages.
#[derive(Debug)]
pub(crate) struct PageOffset(u16);

impl PageOffset {
  /// Create a new page offset with given offset `u16`.
  #[inline]
  pub fn new(offset: u16) -> Self {
    debug_assert!(offset < (1 << 12));
    Self(offset)
  }

  #[inline]
  pub const fn new_truncate(offset: u16) -> Self {
    Self(offset % (1 << 12))
  }
}

#[cfg(target_pointer_width = "64")]
impl From<PageOffset> for PtrWidth {
  #[inline]
  fn from(offset: PageOffset) -> Self {
    Self::from(offset.0)
  }
}

impl From<PageOffset> for u16 {
  #[inline]
  fn from(offset: PageOffset) -> Self {
    Self::from(offset.0)
  }
}

impl From<PageOffset> for usize {
  #[inline]
  fn from(offset: PageOffset) -> Self {
    Self::from(offset.0)
  }
}

/// Level for page table, totally four levels.
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
