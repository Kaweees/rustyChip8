// #[rustfmt::skip]
// /*
// Keypad is a 4x4 matrix of keys that can be pressed
// Input Mapping:
// Keypad       Keyboard
// +-+-+-+-+    +-+-+-+-+
// |1|2|3|C|    |1|2|3|4|
// +-+-+-+-+    +-+-+-+-+
// |4|5|6|D|    |Q|W|E|R|
// +-+-+-+-+ => +-+-+-+-+
// |7|8|9|E|    |A|S|D|F|
// +-+-+-+-+    +-+-+-+-+
// |A|0|B|F|    |Z|X|C|V|
// +-+-+-+-+    +-+-+-+-+
// */
use macroquad::input::{KeyCode, is_key_pressed};
const KEYPAD_SIZE: usize = 16;

pub struct Keypad {
  buffer: [bool; KEYPAD_SIZE], // 16 keys
}

impl Keypad {
  pub fn new() -> Self {
    Keypad {
      buffer: [false; KEYPAD_SIZE],
    }
  }

  pub fn get_key(&self, key: usize) -> bool {
    self.buffer[key]
  }

  pub fn set_key(&mut self, key: usize, value: bool) {
    self.buffer[key] = value;
  }

  pub fn clear(&mut self) {
    self.buffer.fill(false);
  }

  pub fn update(&mut self) {
    // Update the keypad
    self.buffer[0x1] = is_key_pressed(KeyCode::Key1);
    self.buffer[0x2] = is_key_pressed(KeyCode::Key2);
    self.buffer[0x3] = is_key_pressed(KeyCode::Key3);
    self.buffer[0xC] = is_key_pressed(KeyCode::Key4);
    self.buffer[0x4] = is_key_pressed(KeyCode::Q);
    self.buffer[0x5] = is_key_pressed(KeyCode::W);
    self.buffer[0x6] = is_key_pressed(KeyCode::E);
    self.buffer[0xD] = is_key_pressed(KeyCode::R);
    self.buffer[0x7] = is_key_pressed(KeyCode::A);
    self.buffer[0x8] = is_key_pressed(KeyCode::S);
    self.buffer[0x9] = is_key_pressed(KeyCode::D);
    self.buffer[0xE] = is_key_pressed(KeyCode::F);
    self.buffer[0xA] = is_key_pressed(KeyCode::Z);
    self.buffer[0x0] = is_key_pressed(KeyCode::X);
    self.buffer[0xB] = is_key_pressed(KeyCode::C);
    self.buffer[0xF] = is_key_pressed(KeyCode::V);
  }
}
