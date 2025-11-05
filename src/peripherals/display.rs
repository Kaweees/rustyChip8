// #[rustfmt::skip]
// /*
// Display is a 64x32 monochrome display. Each byte represents 8 pixels in a
// column.
// (0,0)  -> (63,0)
// (0,31) -> (63,31)
// */
use crate::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH, SCALE_FACTOR};
use macroquad::prelude::Color;

pub struct Display {
  // Array to represent display
  buffer: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT], // monochrome display
  pixel_color: Color,
  background_color: Color,
}

impl Display {
  pub fn new() -> Self {
    Display {
      buffer: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
      pixel_color: Color::new(255.0, 255.0, 255.0, 1.0),
      background_color: Color::new(0.0, 0.0, 0.0, 0.0),
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
    // Draw background
    macroquad::prelude::clear_background(self.background_color);
    // Draw screen buffer on screen
    for y in 0..DISPLAY_HEIGHT {
      for x in 0..DISPLAY_WIDTH {
        if self.get_pixel(x, y) {
          macroquad::prelude::draw_rectangle(
            x as f32 * SCALE_FACTOR as f32,
            y as f32 * SCALE_FACTOR as f32,
            SCALE_FACTOR as f32,
            SCALE_FACTOR as f32,
            self.pixel_color,
          );
        }
      }
    }
  }
  pub fn clear(&mut self) {
    self.buffer.fill(false);
  }
}
