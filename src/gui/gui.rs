use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::cpu::chip8::Chip8;
use macroquad::prelude::is_quit_requested;
use macroquad::prelude::next_frame;
use macroquad::window::Conf;

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = -1;

fn window_conf() -> Conf {
  Conf {
    window_title: "Chip-8 Emulator:".to_owned(),
    window_width: (SCREEN_WIDTH) as i32,
    window_height: (SCREEN_HEIGHT) as i32,
    fullscreen: false,
    window_resizable: false,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
pub async fn main() {
  let file: String = "./rom/space-invaders.ch8".to_string();

  let mut chip8 = Chip8::new();
  chip8.load_rom(&file);
  while !is_quit_requested() {
    chip8.cycle();
    next_frame().await;
  }
  std::process::exit(EXIT_SUCCESS);
}
