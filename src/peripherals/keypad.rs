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
    // TODO: Implement keyboard input
  }
}
