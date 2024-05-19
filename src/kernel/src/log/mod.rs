#[rustfmt::skip]
#[allow(unused_imports)]
pub use log::{
  trace,
  debug,
  info,
  warn,
  error,
};
use crate::arch::hw::vga::VgaWriter;
use crate::arch::hw::vga::VGA_WRITER;

/// Initialize console log.
pub fn init() {
  log::set_max_level(log::LevelFilter::Trace);
}

impl log::Log for VgaWriter {
  /// True if log is enabled.
  fn enabled(&self, _: &log::Metadata<'_>) -> bool {
    // It is intended to be left blank.
    false
  }

  fn log(&self, record: &log::Record<'_>) {
    use core::fmt::Write;

    VGA_WRITER
      .lock()
      .write_fmt(format_args!("[{:>5}] {}\n", record.level(), record.args()))
      .unwrap();
  }

  fn flush(&self) {
    // It is intended to be left blank.
  }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::log::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
  use core::fmt::Write;

  VGA_WRITER.lock().write_fmt(args).unwrap();
}
