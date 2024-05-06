use spin::Mutex;

pub mod time;

pub type NanoSecond = u128;

/// Kernel start time.
pub static START: Mutex<NanoSecond> = Mutex::new(0);
/// Kernel offset time.
pub static OFFSET: Mutex<NanoSecond> = Mutex::new(0);

pub fn monotonic() -> NanoSecond {
  *OFFSET.lock() + crate::arch::shared::time::counter()
}

pub fn realtime() -> NanoSecond {
  *START.lock() + monotonic()
}
