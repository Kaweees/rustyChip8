use crate::constants::{PROGRAM_START, RAM_SIZE};
use crate::mapper::mapper::Mapper;
use std::fs::File;
use std::io::Read;

pub const FONTSET: &[u8] = &[
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
  pub v: [u8; 16], // 16 registers
  pub pc: u16, // Program Counter (PC, 16-bit), - stores address of next instruction to execute
  pub sp: u8,  // Stack Pointer (S, 8-bit), - points to location on memory bus
  pub mapper: Mapper,
}

impl Chip8 {
  pub fn new() -> Self {
    Chip8 {
      v: [0; 16],
      pc: 0x0000,
      sp: 0x00,
      mapper: Mapper::new(),
    }
  }

  // Reads a byte from the memory at the specified address
  pub fn read(&self, address: u16) -> u8 {
    self.mapper.fetch_byte(address)
  }

  // Writes a byte to the memory at the specified address
  pub fn write(&mut self, address: u16, value: u8) {
    self.mapper.store_byte(address, value);
  }

  // Resets the CPU to a known state
  pub fn reset(&mut self) {
    self.pc = 0x0000;
    self.sp = 0x00;
  }

  // Method to load a ROM into memory
  pub fn load_rom(&mut self, filename: &str) {
    let mut file = File::open(filename).unwrap();
    file
      .read(
        &mut self.mapper.ram
          [PROGRAM_START as usize..RAM_SIZE - PROGRAM_START as usize],
      )
      .unwrap();
  }
}
