//! # CPU ID
//!
//! `cpuid` instruction on `x86_64` architecture is designed to get CPU processor info.
use core::fmt::write;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::todo;

use crate::arch::shared::cpuid::__impl::NativeCpuIdReader;

const EAX_VENDOR_INFO: u32 = 0x0000_0000;
const EAX_FEATURE_INFO: u32 = 0x0000_0001;

const EAX_EXTENDED_FUNCTION_INFO: u32 = 0x8000_0000;

pub struct CpuId<R: CpuIdReader> {
  supported_leaves: u32,
  supported_extended_leaves: u32,
  vendor: Vendor,
  read: R,
}

impl Default for CpuId<NativeCpuIdReader> {
  fn default() -> Self {
    todo!()
  }
}

impl<R: CpuIdReader> CpuId<R> {
  pub fn with_cpuid_reader(read: R) -> Self {
    let vendor_leaf = read.cpuid1(EAX_VENDOR_INFO);
    let extended_leaf = read.cpuid1(EAX_EXTENDED_FUNCTION_INFO);
    Self {
      supported_leaves: vendor_leaf.eax,
      supported_extended_leaves: extended_leaf.eax,
      vendor: Vendor::from_vendor_leaf(vendor_leaf),
      read,
    }
  }

  fn leaf_is_supported(&self, flag: u32) -> bool {
    if Vendor::Amd == self.vendor && ((0x0000_0002..=0x0000_0004).contains(&flag)) {
      return false;
    }

    if flag < EAX_EXTENDED_FUNCTION_INFO {
      flag <= self.supported_leaves
    } else {
      flag <= self.supported_extended_leaves
    }
  }
}

impl<R: CpuIdReader> CpuId<R> {
  /// Get processor vendor info.
  pub fn get_vendor_info(&self) -> Option<VendorInfo> {
    if self.leaf_is_supported(EAX_VENDOR_INFO) {
      let res = self.read.cpuid1(EAX_VENDOR_INFO);
      Some(VendorInfo {
        ebx: res.ebx,
        ecx: res.ecx,
        edx: res.edx,
      })
    } else {
      None
    }
  }
}

impl CpuId<NativeCpuIdReader> {
  pub fn new() -> Self {
    CpuId::default()
  }
}

/// Result of `cpuid` instrument.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct CpuIdResult {
  /// Value of EAX register.
  pub(crate) eax: u32,
  /// Value of EBX register.
  pub(crate) ebx: u32,
  /// Value of ECX register.
  pub(crate) ecx: u32,
  /// Value of EDX register.
  pub(crate) edx: u32,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl Debug for CpuIdResult {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("CpuIdResult")
      .field("eax", &(self.eax as *const u32))
      .field("ebx", &(self.ebx as *const u32))
      .field("ecx", &(self.ecx as *const u32))
      .field("edx", &(self.edx as *const u32))
      .finish()
  }
}

#[cfg(target_arch = "aarch64")]
impl Debug for CpuIdResult {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    todo!()
  }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl CpuIdResult {
  pub fn all_zero(&self) -> bool {
    self.eax == 0 && self.ebx == 0 && self.ecx == 0 && self.edx == 0
  }
}

#[cfg(target_arch = "aarch64")]
impl CpuIdResult {
  pub fn all_zero(&self) -> bool {
    todo!()
  }
}

#[cfg(target_arch = "aarch64")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct CpuIdResult {}

pub trait CpuIdReader {
  fn cpuid1(&self, eax: u32) -> CpuIdResult {
    self.cpuid2(eax, 0)
  }

  fn cpuid2(&self, eax: u32, ecx: u32) -> CpuIdResult;
}

impl<R> CpuIdReader for R
where
  R: Fn(u32, u32) -> CpuIdResult + Clone,
{
  fn cpuid2(&self, eax: u32, ecx: u32) -> CpuIdResult {
    self(eax, ecx)
  }
}

mod __impl {
  use super::*;

  #[derive(Copy, Clone)]
  pub(super) struct NativeCpuIdReader;

  impl CpuIdReader for NativeCpuIdReader {
    fn cpuid2(&self, eax: u32, ecx: u32) -> CpuIdResult {
      todo!()
    }
  }
}

/// Processor vendor.
#[derive(PartialEq, Eq)]
pub enum Vendor {
  #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
  Intel,
  #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
  Amd,
  Unknown(u32, u32, u32),
}

impl Vendor {
  fn from_vendor_leaf(res: CpuIdResult) -> Self {
    let info = VendorInfo {
      ebx: res.ebx,
      ecx: res.ecx,
      edx: res.edx,
    };

    match info.as_str() {
      "GenuineIntel" => Vendor::Intel,
      "AuthenticAMD" => Vendor::Amd,
      _ => Vendor::Unknown(res.edx, res.ecx, res.edx),
    }
  }
}

/// Vendor info string
///
/// A string that can be for example "AuthenticAMD" or "GenuineIntel".
///
/// # Technical Background
///
/// The vendor info is a 12-byte (96 bit) long string stored in `ebx`, `edx` and
/// `ecx` by the corresponding `cpuid` instruction.
#[derive(PartialEq, Eq, Debug)]
#[repr(C)]
pub struct VendorInfo {
  /// Value of EBX register.
  ebx: u32,
  /// Value of EBX register.
  edx: u32,
  /// Value of EBX register.
  ecx: u32,
}

impl VendorInfo {
  pub fn as_str(&self) -> &str {
    let brand = self as *const Self as *const u8;
    let slice = unsafe { core::slice::from_raw_parts(brand, core::mem::size_of::<VendorInfo>()) };
    core::str::from_utf8(slice).expect("Invalid vendor string")
  }
}

impl core::fmt::Display for VendorInfo {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cpu_id() {
    let cpu_id = CpuId::new();
    if let Some(v) = cpu_id.get_vendor_info() {
      assert!(v.as)
    }
  }
}
