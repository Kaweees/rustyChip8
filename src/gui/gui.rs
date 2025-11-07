use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, VERSION_STRING};
use crate::cpu::chip8::Chip8;
use clap::{Arg, ArgAction, Command};
use macroquad::prelude::is_quit_requested;
use macroquad::prelude::next_frame;
use macroquad::window::Conf;

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

#[macroquad::main(window_conf)]
pub async fn main() {
  let cmd = Command::new(env!("CARGO_CRATE_NAME"))
    .about("make a hexdump or do the reverse")
    .version(VERSION_STRING)
    .arg_required_else_help(true)
    .multicall(false)
    .arg(
      Arg::new("file")
        .short('f')
        .long("file")
        .required(true)
        .action(ArgAction::Set)
        .num_args(1..)
        .help("The file to load"),
    )
    .get_matches();

  let file = cmd.get_one::<String>("file").unwrap();

  let mut chip8 = Chip8::new();
  chip8.load_rom(&file);
  chip8.mapper.print_memory(0x0025, 200);
  while !is_quit_requested() {
    chip8.cycle();
    chip8.mapper.display.update();
    next_frame().await;
  }
  std::process::exit(0);
}
