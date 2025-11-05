use crate::constants::{PROGRAM_END, PROGRAM_START, RAM_SIZE, STACK_SIZE};
use crate::peripherals::display::Display;
use crate::peripherals::keypad::Keypad;

pub struct Mapper {
  pub ram: [u8; RAM_SIZE],       // 4 kB of memory
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

  // Print memory contents
  pub fn print_memory(&self, address: u32, count: u32) {
    let mut start_address = address;
    let end_address = address + count;
    let mut num = 1;

    // Padding to align the memory address
    if address % 16 != 0 {
      let aligned_start = address.wrapping_sub(address % 16);
      let padding_end = aligned_start.wrapping_add(16).min(end_address);

      print!("0x{:08x}: ", aligned_start);
      for i in aligned_start..padding_end {
        let byte = self.ram[i as usize];
        print!("{:02x} ", byte);
        if num % 8 == 0 {
          print!(" ");
          num = 0;
        }
        num += 1;
      }
      println!();
      start_address = aligned_start;
      num = 1; // Reset for aligned section
      //   for i in address..padding_end {
      //     let byte = self.ram[i as usize];
      //     print!("{:02x} ", byte);
      //     if num % 8 == 0 {
      //       print!(" ");
      //       num = 0;
      //     }
      //     num += 1;
      //   }
      //   println!();
      //   start_address = aligned_start;
      //   num = 1; // Reset for aligned section
    }

    // for i in start_address..end_address {
    //   if i % 16 == 0 && i != start_address {
    //     println!();
    //   }
    //   if i % 16 == 0 {
    //     print!("0x{:08x}: ", i);
    //   }
    //   let byte = self.ram[i as usize];
    //   print!("{:02x} ", byte);
    //   if num % 8 == 0 && i != start_address {
    //     print!(" ");
    //     num = 0;
    //   }
    //   num += 1;
    // }
  }
}
