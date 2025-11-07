use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, VERSION_STRING};
use crate::cpu::chip8::Chip8;
use clap::Parser;
use macroquad::prelude::is_quit_requested;
use macroquad::prelude::next_frame;
use macroquad::window::Conf;
use std::path::PathBuf;

fn window_conf() -> Conf {
  Conf {
    window_title: format!("Chip-8 Emulator v{}", VERSION_STRING),
    window_width: (SCREEN_WIDTH) as i32,
    window_height: (SCREEN_HEIGHT) as i32,
    fullscreen: false,
    window_resizable: false,
    ..Default::default()
  }
}

#[derive(Parser, Debug)]
#[command(author, version = VERSION_STRING, about = "Chip-8 Emulator")]
struct Cli {
  /// Path to the ROM file to load
  #[arg(
    short,
    long,
    value_name = "FILE",
    help = "Path to the ROM file",
    required = true
  )]
  rom: PathBuf,

  /// Whether to run verbosely
  #[arg(short, long, default_value_t = false)]
  verbose: bool,
}

#[macroquad::main(window_conf)]
pub async fn main() {
  let cli = Cli::parse();

  let mut chip8 = Chip8::new();
  chip8.load_rom(cli.rom);
  chip8.mapper.print_memory(0x0025, 200);
  while !is_quit_requested() {
    chip8.cycle();
    chip8.mapper.display.update();
    next_frame().await;
  }
  std::process::exit(0);
}
