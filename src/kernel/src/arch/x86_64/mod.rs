use crate::arch::paging::PageTableIndex;
use crate::arch::paging::PageTableLevel;
use crate::arch::x86_64::paging::PageTable;
use crate::arch::x86_64::reg::CR3;
use crate::arch::FrameError;
use crate::arch::PhysicalAddress;
use crate::arch::VirtualAddress;
use crate::println;

pub mod gdt;
pub mod hw;
pub mod interrupt;
pub mod paging;
pub mod reg;

/// Activate CPU page table of Level 4.
/// Offset: Physical memory offset.
pub unsafe fn active_level_4_table(offset: VirtualAddress) -> &'static mut PageTable {
  let (cpu_phys_frame, _) = CR3::read();
  let phys_addr = PhysicalAddress::new(cpu_phys_frame.start_address().as_raw());
  let virt_addr = offset + phys_addr.as_raw();
  let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();
  let page_table = &mut *page_table_ptr;
  #[cfg(debug_assertions)]
  for (i, entry) in page_table.iter().enumerate() {
    if !entry.is_unused() {
      println!("L4 Entry {}: {:?}", i, entry);
    }
  }
  page_table
}

pub(crate) fn translate_address_inner(
  address: VirtualAddress,
  physical_memory_offset: VirtualAddress,
) -> Option<PhysicalAddress> {
  let (mut frame, _) = CR3::read();

  let table_indexes = [
    address.p4_index(),
    address.p3_index(),
    address.p2_index(),
    address.p1_index(),
  ];
  for &index in &table_indexes {
    let virtual_address = physical_memory_offset + frame.start_address().as_raw();
    let table_ptr: *const PageTable = virtual_address.as_ptr();
    let table = unsafe { &*table_ptr };

    let entry = &table[index];
    frame = match entry.frame() {
      Ok(frame) => frame,
      Err(FrameError::FrameNotPresent) => return None,
      Err(FrameError::HugePage) => panic!(""),
    };
  }

  Some(frame.start_address() + u64::from(address.page_offset()))
}

impl VirtualAddress {
  #[inline]
  pub const fn p1_index(self) -> PageTableIndex {
    PageTableIndex::new_truncate((self.0 >> 12) as u16)
  }

  #[inline]
  pub const fn p2_index(self) -> PageTableIndex {
    PageTableIndex::new_truncate((self.0 >> 12 >> 9) as u16)
  }

  #[inline]
  pub const fn p3_index(self) -> PageTableIndex {
    PageTableIndex::new_truncate((self.0 >> 12 >> 9 >> 9) as u16)
  }

  #[inline]
  pub const fn p4_index(self) -> PageTableIndex {
    PageTableIndex::new_truncate((self.0 >> 12 >> 9 >> 9 >> 9) as u16)
  }

  #[inline]
  pub const fn page_table_index(self, level: PageTableLevel) -> PageTableIndex {
    PageTableIndex::new_truncate((self.0 >> 12 >> ((level as u8 - 1) * 9)) as u16)
  }
}
