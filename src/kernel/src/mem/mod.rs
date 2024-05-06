//! # Memory Management

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

/// The memory area
/// Such as BSS, Data, Text segment.
#[derive(Clone, Copy, Debug, Default)]
pub struct MemoryArea {
  /// The start address of the memory area
  pub(crate) base_address: usize,
  pub(crate) length:       usize,
}
