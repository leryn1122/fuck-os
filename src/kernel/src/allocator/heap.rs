use core::alloc::AllocError;
use core::alloc::Allocator;
use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::ptr;
use core::ptr::NonNull;

use linked_list_allocator::Heap;
use spin::Mutex;

use crate::allocator::KERNEL_HEAP_SIZE;
use crate::arch::VirtualAddress;

/// Global heap allocator.
#[global_allocator]
static HEAP_ALLOCATOR: MyAllocator = MyAllocator;

static HEAP: Mutex<Option<Heap>> = Mutex::new(None);

struct MyAllocator;

impl MyAllocator {
  /// ## Safety
  pub unsafe fn init(start: VirtualAddress, end: VirtualAddress) {
    *HEAP.lock() = Some(Heap::new(start.as_mut_ptr(), (end - start) as usize));
  }
}

unsafe impl<'a> Allocator for &'a MyAllocator {
  fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    todo!()
  }

  unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
    todo!()
  }
}

unsafe impl GlobalAlloc for MyAllocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    while let Some(ref mut heap) = *HEAP.lock() {
      match heap.allocate_first_fit(layout) {
        res => return res.ok().map_or(ptr::null_mut(), |alloc| alloc.as_ptr()),
        Err(()) => {
          let size = heap.size();
          // TODO
          heap.extend(KERNEL_HEAP_SIZE)
        }
      }
    }
    panic!("__rust_allocate: heap not initialized");
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    if let Some(ref mut heap) = *HEAP.lock() {
      heap.deallocate(NonNull::new_unchecked(ptr), layout)
    } else {
      panic!("__rust_deallocate: heap not initialized");
    }
  }
}
