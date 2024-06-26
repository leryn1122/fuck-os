//! # VGA Text Buffer
//! To print a character to the screen in VGA text mode, one has to write it to the text buffer of
//! the VGA hardware. The VGA text buffer is a two-dimensional array with typically 25 rows and 80
//! columns, which is directly rendered to the screen. Each array entry describes a single screen
//! character.
//!
//! The VGA text buffer is accessible via memory-mapped I/O to the address `0xb8000`. This means
//! that reads and writes to that address don’t access the RAM but directly access the text buffer
//! on the VGA hardware.
//!
//! The code references on:
//! - [vga-text-mode](https://os.phil-opp.com/vga-text-mode/).

use core::ops::Deref;
use core::ops::DerefMut;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

/// Height of VGA text buffer.
const BUFFER_HEIGHT: usize = 25;
/// Width of VGA text buffer.
const BUFFER_WIDTH: usize = 80;

lazy_static! {
  /// VGA writer instance.
  pub static ref VGA_WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter::new());
}

/// VGA color enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
  Black = 0,
  Blue = 1,
  Green = 2,
  Cyan = 3,
  Red = 4,
  Magenta = 5,
  Brown = 6,
  LightGray = 7,
  DarkGray = 8,
  LightBlue = 9,
  LightGreen = 10,
  LightCyan = 11,
  LightRed = 12,
  Pink = 13,
  Yellow = 14,
  White = 15,
}

/// Internal struct for color code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
  pub(crate) const fn new(foreground: Color, background: Color) -> Self {
    Self((background as u8) << 4 | (foreground as u8))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenCharacter {
  ascii_character: u8,
  color_code:      ColorCode,
}

impl Deref for ScreenCharacter {
  type Target = ScreenCharacter;

  fn deref(&self) -> &Self::Target {
    self
  }
}

impl DerefMut for ScreenCharacter {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self
  }
}

/// VGA buffer.
#[repr(transparent)]
struct VgaBuffer {
  chars: [[Volatile<ScreenCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VgaWriter {
  column_position: usize,
  color_code:      ColorCode,
  buffer:          &'static mut VgaBuffer,
}

impl VgaWriter {
  pub(crate) fn new() -> Self {
    Self {
      column_position: 0,
      color_code:      ColorCode::new(Color::White, Color::Black),
      buffer:          unsafe { &mut *(0xb8000 as *mut VgaBuffer) },
    }
  }

  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      byte => {
        if self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        let color_code = self.color_code;
        self.buffer.chars[row][col].write(ScreenCharacter {
          ascii_character: byte,
          color_code,
        });
        self.column_position += 1;
      }
    }
  }

  pub fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
      match byte {
        // ASCII or '\n'
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        // Other
        _ => self.write_byte(0xFE),
      }
    }
  }

  fn new_line(&mut self) {
    for row in 1..BUFFER_HEIGHT {
      for col in 0..BUFFER_WIDTH {
        let character = self.buffer.chars[row][col].read();
        self.buffer.chars[row - 1][col].write(character);
      }
    }
    self.clear_row(BUFFER_HEIGHT - 1);
    self.column_position = 0;
  }

  fn clear_row(&mut self, row: usize) {
    let blank = ScreenCharacter {
      ascii_character: b' ',
      color_code:      self.color_code,
    };
    for col in 0..BUFFER_WIDTH {
      self.buffer.chars[row][col].write(blank);
    }
  }

  pub fn clear(&mut self) {
    let blank = ScreenCharacter {
      ascii_character: b' ',
      color_code:      self.color_code,
    };
    for row in 0..BUFFER_HEIGHT {
      for col in 0..BUFFER_WIDTH {
        self.buffer.chars[row][col].write(blank);
      }
    }
  }
}

impl core::fmt::Write for VgaWriter {
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    self.write_string(s);
    Ok(())
  }
}
