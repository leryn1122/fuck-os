//! # Memory Management

use x86_64::structures::paging::OffsetPageTable;

use crate::arch::active_level_4_table;
use crate::arch::PhysicalAddress;
use crate::arch::VirtualAddress;

pub(crate) mod mapper;

pub enum MemoryKind {
  /// Kernel memory
  Kernel,
  /// Userspace memory
  Userspace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct MemoryDescriptor {
  pub(crate) physical_address: PhysicalAddress,
  pub(crate) virtual_address:  VirtualAddress,
}

/// ## Safety
pub unsafe fn init_memory(physical_address_offset: VirtualAddress)
// -> ()<'static>
{
  let level_4_table = unsafe { active_level_4_table(physical_address_offset) };

  // OffsetPageTable::new(level_4_table, physical_address_offset)
}
