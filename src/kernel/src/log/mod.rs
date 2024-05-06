#[rustfmt::skip]
#[allow(unused_imports)]
pub use log::{
  trace,
  debug,
  info,
  warn,
  error,
};
use crate::vga::Writer;
use crate::vga::WRITER;

/// Initialize VGA log.
pub fn init() {
  log::set_max_level(log::LevelFilter::Trace);
}

impl log::Log for Writer {
  /// True if log is enabled.
  fn enabled(&self, _: &log::Metadata) -> bool {
    true
  }

  fn log(&self, record: &log::Record) {
    use core::fmt::Write;

    if !self.enabled(record.metadata()) {
      return;
    }

    WRITER.lock().write_fmt(format_args!("{}", record.args())).unwrap();
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

  WRITER.lock().write_fmt(args).unwrap();
}
