use crate::arch::x86_64::paging::PageTable;
use crate::arch::x86_64::reg::CR3;
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
  let phys_addr = PhysicalAddress::new(cpu_phys_frame.start_address().as_u64() as usize);
  let virt_addr = offset + phys_addr.as_u64();
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
