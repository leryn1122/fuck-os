use core::marker::PhantomData;

use crate::arch::PhysicalAddress;
use crate::mem::MemoryKind;

pub struct PageMapper {
  kind:     MemoryKind,
  address:  PhysicalAddress,
  _phantom: PhantomData<()>,
}

impl PageMapper {
  pub fn new(kind: MemoryKind, address: PhysicalAddress) -> Self {
    Self {
      kind,
      address,
      _phantom: PhantomData,
    }
  }

  pub fn create(kind: MemoryKind) -> Option<Self> {
    None
  }

  pub fn current(kind: MemoryKind) -> Option<Self> {
    None
  }
}
