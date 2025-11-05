use crate::constants::{PROGRAM_END, PROGRAM_START, RAM_SIZE, STACK_SIZE};
use crate::peripherals::display::Display;
use crate::peripherals::keypad::Keypad;

pub struct Mapper {
  pub ram: [u8; RAM_SIZE],      // 4 kB of memory
  pub stack: [u16; STACK_SIZE], // 16 levels of stack
  pub keypad: Keypad,
  pub display: Display,
}

impl Mapper {
  pub fn new() -> Self {
    Mapper {
      ram: [0; RAM_SIZE],
      stack: [0; STACK_SIZE],
      keypad: Keypad::new(),
      display: Display::new(),
    }
  }

  // Method to read a byte from memory
  pub fn fetch_byte(&self, address: u16) -> u8 {
    // Guard against invalid addresses
    if (PROGRAM_START as u16 <= address) && (address <= PROGRAM_END as u16) {
      return self.ram[address as usize];
    } else {
      panic!("Memory read out of bounds");
    }
  }

  // Method to write a byte to memory
  pub fn store_byte(&mut self, address: u16, value: u8) {
    // Guard against invalid addresses
    if PROGRAM_START as u16 <= address && address <= PROGRAM_END as u16 {
      self.ram[address as usize] = value;
    } else {
      panic!("Memory write out of bounds");
    }
  }
}
