// #[rustfmt::skip]
// /*
// Display is a 64x32 monochrome display. Each byte represents 8 pixels in a
// column.
// (0,0)  -> (63,0)
// (0,31) -> (63,31)
// */
use crate::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

use macroquad::prelude::Color;

pub struct Display {
  // Array to represent display
  buffer: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT], // monochrome display
  pixel_color: u8,
  background_color: u8,
}

impl Display {
  pub fn new() -> Self {
    Display {
      buffer: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
      pixel_color: 0xFF,
      background_color: 0x00,
    }
  }

  pub fn get_pixel(&self, x: usize, y: usize) -> bool {
    if x < DISPLAY_WIDTH && y < DISPLAY_HEIGHT {
      return self.buffer[y * DISPLAY_WIDTH + x];
    } else {
      panic!("Invalid pixel coordinates");
    }
  }

  pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
    if x < DISPLAY_WIDTH && y < DISPLAY_HEIGHT {
      self.buffer[y * DISPLAY_WIDTH + x] = value;
    } else {
      panic!("Invalid pixel coordinates");
    }
  }

  pub fn update(&mut self) {
    // TODO: Implement display update
  }
  pub fn clear(&mut self) {
    self.buffer.fill(false);
  }
}
