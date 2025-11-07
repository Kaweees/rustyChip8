use crate::constants::{
  DISPLAY_HEIGHT, DISPLAY_WIDTH, FRAME_DURATION, INSTRUCTIONS_PER_FRAME,
  KEYPAD_SIZE, PROGRAM_START, RAM_SIZE,
};
use crate::mapper::mapper::Mapper;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::{Duration, Instant};

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
  pub i: u16,      // Index Register (I, 16-bit), - stores memory address
  pub pc: u16, // Program Counter (PC, 16-bit), - stores address of next instruction to execute
  pub sp: usize, // Stack Pointer (S, 8-bit), - points to location on memory bus
  pub delay_timer: u8,
  pub sound_timer: u8,
  pub mapper: Mapper,
  pub frame_duration: Duration,
  pub last_cycle_time: Instant,
  pub opcode: u16, // The current opcode being executed
  pub rng: ThreadRng,
}

impl Chip8 {
  pub fn new() -> Self {
    let mut chip8 = Chip8 {
      v: [0; 16],
      i: 0,
      pc: PROGRAM_START as u16,
      sp: 0,
      delay_timer: 0,
      sound_timer: 0,
      mapper: Mapper::new(),
      frame_duration: FRAME_DURATION,
      last_cycle_time: Instant::now(),
      opcode: 0x0000,
      rng: rand::rng(),
    };
    // Load fontset into memory at address 0x000
    chip8.mapper.ram[0..FONTSET.len()].copy_from_slice(FONTSET);
    chip8
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
    // Start executing at the CHIP-8 program start
    self.pc = PROGRAM_START as u16;
    self.sp = 0x00;
  }

  // Method to load a ROM into memory
  pub fn load_rom(&mut self, filename: PathBuf) {
    let mut file = File::open(filename).unwrap();
    file
      .read(
        &mut self.mapper.ram
          [PROGRAM_START as usize..RAM_SIZE - PROGRAM_START as usize],
      )
      .unwrap();
  }

  // Executes one cycle of the CPU
  pub fn cycle(&mut self) {
    let current_time = Instant::now();
    let elapsed_time = current_time.duration_since(self.last_cycle_time);

    if elapsed_time < self.frame_duration {
      std::thread::sleep(self.frame_duration - elapsed_time);
    }

    // Update the keypad
    self.mapper.keypad.update();

    // Execute multiple instructions per frame to maintain game speed
    for _ in 0..INSTRUCTIONS_PER_FRAME {
      self.fetch();
      self.execute();
    }

    // Update timers
    if self.delay_timer > 0 {
      self.delay_timer -= 1;
    }
    if self.sound_timer > 0 {
      self.sound_timer -= 1;
    }
    self.last_cycle_time = Instant::now();
  }

  // Fetches the next opcode from memory
  pub fn fetch(&mut self) {
    self.opcode =
      (self.read(self.pc) as u16) << 8 | self.read(self.pc + 1) as u16;
    // println!("Opcode: {:04X}", self.opcode);
  }

  // Executes the next instruction (minimal, valid implementation)
  pub fn execute(&mut self) {
    let x = ((self.opcode & 0x0F00) >> 8) as usize;
    let y = ((self.opcode & 0x00F0) >> 4) as usize;
    let n = (self.opcode & 0x000F) as u8;
    let nn = (self.opcode & 0x00FF) as u8;
    let nnn = self.opcode & 0x0FFF;

    match self.opcode & 0xF000 {
      0x0000 => match self.opcode {
        0x00E0 => {
          // 0x00E0: clear
          self.mapper.display.clear();
          self.pc += 2;
        }
        0x00EE => {
          // 0x00EE: return
          if self.sp > 0 {
            self.sp -= 1;
            self.pc = self.mapper.stack[self.sp as usize];
            self.pc += 2;
          } else {
            panic!("Stack underflow");
          }
        }
        _ => {
          // 0x0NNN: call NNN
          self.mapper.stack[self.sp as usize] = self.pc;
          self.sp += 1;
          self.pc = nnn;
        }
      },
      0x1000 => {
        // 1NNN: Jump NNN
        self.pc = nnn;
      }

      0x2000 => {
        // 0x2NNN: NNN
        self.mapper.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
      }
      0x3000 => {
        // 0x3XNN: if vx != NN then
        self.pc += if self.v[x] == nn { 4 } else { 2 };
      }
      0x4000 => {
        // 0x4XNN: Skips the next instruction if Vx != NN
        self.pc += if self.v[x] != nn { 4 } else { 2 };
      }
      0x5000 => {
        // 0x5XY0: Skips the next instruction if Vx == Vy
        self.pc += if self.v[x] == self.v[y] { 4 } else { 2 };
      }
      0x6000 => {
        // 0x6XNN: Sets Vx to NN
        self.v[x] = nn;
        self.pc += 2;
      }
      0x7000 => {
        // 0x7XNN: Adds NN to Vx
        self.v[x] = self.v[x].wrapping_add(nn);
        self.pc += 2;
      }
      0x8000 => {
        match n {
          0x0 => {
            // 0x8XY0: Sets Vx to Vy
            self.v[x] = self.v[y];
            self.pc += 2;
          }
          0x1 => {
            // 0x8XY1: Sets Vx to Vx OR Vy
            self.v[x] = self.v[x] | self.v[y];
            self.pc += 2;
          }
          0x2 => {
            // 0x8XY2: Sets Vx to Vx AND Vy
            self.v[x] = self.v[x] & self.v[y];
            self.pc += 2;
          }
          0x3 => {
            // 0x8XY3: Sets Vx to Vx XOR Vy
            self.v[x] = self.v[x] ^ self.v[y];
            self.pc += 2;
          }
          0x4 => {
            // 0x8XY4: Adds Vy to Vx. VF is set to 1 if there's a carry,
            // otherwise 0
            let sum = self.v[x] as u16 + self.v[y] as u16;
            self.v[0xF] = if sum > 0xFF { 1 } else { 0 };
            self.v[x] = sum as u8;
            self.pc += 2;
          }
          0x5 => {
            // 0x8XY5: Vx = Vx - Vy, VF is set to 0 if there's a borrow,
            // otherwise 1
            self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
            self.v[x] = self.v[x].wrapping_sub(self.v[y]);
            self.pc += 2;
          }
          0x6 => {
            // 0x8XY6: Shifts Vx right by one. VF is set to the least
            // significant bit of Vx before the shift
            self.v[0xF] = self.v[x] & 0x1;
            self.v[x] >>= 1;
            self.pc += 2;
          }
          0x7 => {
            // 0x8XY7: Sets Vx to Vy - Vx. VF is set to 0 if there's a
            // borrow, otherwise 1
            self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
            self.v[x] = self.v[y].wrapping_sub(self.v[x]);
            self.pc += 2;
          }
          0xE => {
            // 0x8XYE: Shifts Vx left by one. VF is set to the most
            // significant bit of Vx before the shift
            self.v[0xF] = self.v[x] >> 7;
            self.v[x] <<= 1;
            self.pc += 2;
          }
          _ => todo!("Unknown opcode: {:04X}", self.opcode),
        }
      }
      0x9000 => {
        // 0x9XY0: Skips the next instruction if Vx != Vy
        self.pc += if self.v[x] != self.v[y] { 4 } else { 2 };
      }
      0xA000 => {
        // 0xA000: Sets I to the address NNN
        self.i = nnn;
        self.pc += 2;
      }
      0xB000 => {
        // 0xBNNN: Jumps to the address NNN + V0
        self.pc = nnn + self.v[0] as u16;
      }
      0xC000 => {
        // 0xCXNN: Sets Vx to a random number AND NN
        self.v[x] = self.rng.random_range(0..=255) & nn;
        self.pc += 2;
      }
      0xD000 => {
        // 0xDXYN: Draws a sprite at (VX, VY) with a height of N
        self.v[0xF] = 0;
        let pixel_x = self.v[x] as usize % DISPLAY_WIDTH;
        let pixel_y = self.v[y] as usize % DISPLAY_HEIGHT;
        for row in 0..n as usize {
          let sprite_byte = self.mapper.fetch_byte(self.i + row as u16);
          for col in 0..8 {
            let sprite_pixel = ((sprite_byte >> (7 - col)) & 0x1) != 0;
            let display_pixel =
              self.mapper.display.get_pixel(pixel_x + col, pixel_y + row);
            if sprite_pixel && display_pixel {
              self.v[0xF] = 1;
            }
            self.mapper.display.set_pixel(
              pixel_x + col,
              pixel_y + row,
              sprite_pixel ^ display_pixel,
            );
          }
        }
        self.pc += 2;
      }
      0xE000 => {
        match nn {
          0x9E => {
            // 0xEX9E: Skips the next instruction if the key stored in
            // Vx is pressed
            self.pc += if self.mapper.keypad.get_key(self.v[x] as usize) {
              4
            } else {
              2
            };
          }
          0xA1 => {
            // 0xEXA1: Skips the next instruction if the key stored in
            // Vx is not pressed
            self.pc += if self.mapper.keypad.get_key(self.v[x] as usize) {
              2
            } else {
              4
            };
          }
          _ => todo!("Unknown opcode: {:04X}", self.opcode),
        }
      }
      0xF000 => {
        match nn {
          0x07 => {
            // 0xF007: Sets Vx to the value of the delay timer
            self.v[x] = self.delay_timer;
            self.pc += 2;
          }
          0x0A => {
            // 0xF00A: A key press is awaited, and then stored in Vx
            let mut key_pressed = false;
            for i in 0..KEYPAD_SIZE {
              if self.mapper.keypad.get_key(i) {
                self.v[x] = i as u8;
                self.pc += 2;
                key_pressed = true;
                break;
              }
            }
            if !key_pressed {
              self.pc = self.pc.wrapping_sub(2);
            }
          }
          0x15 => {
            // 0xF015: Sets the delay timer to Vx
            self.delay_timer = self.v[x];
            self.pc += 2;
          }
          0x18 => {
            // 0xF018: Sets the sound timer to Vx
            self.sound_timer = self.v[x];
            self.pc += 2;
          }
          0x1E => {
            // 0xF01E: Adds Vx to I
            self.i = self.i.wrapping_add(self.v[x] as u16);
            self.pc += 2;
          }
          0x29 => {
            // 0xF029: Sets I to the location of the sprite for the
            // character in Vx
            self.i = self.v[x] as u16 * 5;
            self.pc += 2;
          }
          0x33 => {
            // 0xF033: Stores the binary-coded decimal representation of
            // Vx, with the most significant of three digits at the
            // address in I, the middle digit at I plus 1, and the least
            // significant digit at I plus 2
            self.write(self.i, self.v[x] / 100);
            self.write(self.i + 1, (self.v[x] / 10) % 10);
            self.write(self.i + 2, self.v[x] % 10);
            self.pc += 2;
          }
          0x55 => {
            // 0xF055: Stores V0 to VX (including VX) in memory starting
            // at address I
            for j in 0..=x {
              self.write(self.i + j as u16, self.v[j as usize]);
            }
            self.pc += 2;
          }
          0x65 => {
            // 0xFX65: Fills V0 to VX (including VX) with values from
            // memory starting at address I
            for j in 0..=x {
              self.v[j as usize] = self.read(self.i + j as u16);
            }
            self.pc += 2;
          }
          _ => todo!("Unknown opcode: {:04X}", self.opcode),
        }
      }
      _ => todo!("Unimplemented opcode: {:04X}", self.opcode),
    }
  }
}
