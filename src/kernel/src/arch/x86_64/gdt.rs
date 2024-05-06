use core::ptr::addr_of;

use lazy_static::lazy_static;
use x86_64::structures::gdt::Descriptor;
use x86_64::structures::gdt::GlobalDescriptorTable;
use x86_64::structures::tss::TaskStateSegment;

use crate::println;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
  /// # TSS - Task State Segment
  static ref TSS: TaskStateSegment = {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
      const STACK_SIZE: usize = 4096 * 5;
      static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

      let stack_start = x86_64::VirtAddr::from_ptr(unsafe { addr_of!(STACK) });
      let stack_end = stack_start + STACK_SIZE as u64;
      stack_end
    };
    tss
  };
}

lazy_static! {
  /// # GPT - Global Descriptor Table
  static ref GDT: GlobalDescriptorTable = {
    let mut gdt = GlobalDescriptorTable::new();
    gdt.append(Descriptor::kernel_code_segment());
    gdt.append(Descriptor::tss_segment(&TSS));
    gdt
  };
}

pub fn init_gdt() {
  println!("[INFO   ] Initialize GDT.");
  GDT.load();
}
