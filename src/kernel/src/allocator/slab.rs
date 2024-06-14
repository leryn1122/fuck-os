use core::alloc::AllocError;
use core::alloc::Allocator;
use core::alloc::Layout;
use core::ptr::NonNull;

pub struct SlabAllocator;

impl SlabAllocator {}

unsafe impl<'a> Allocator for &'a SlabAllocator {
  fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    todo!()
  }

  unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
    todo!()
  }
}
