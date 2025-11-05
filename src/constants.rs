use std::time::Duration;

// Constants for game speed
pub const TARGET_FPS: u32 = 60; // Target frames per second
pub const FRAME_DURATION: Duration =
  Duration::from_nanos((1_000_000_000 / TARGET_FPS) as u64); // Duration of a frame in nanoseconds
pub const INSTRUCTIONS_PER_FRAME: u8 = 10;

// Constants for display size and scale factor in pixels
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const SCALE_FACTOR: usize = 20;
pub const CYCLE_DELAY: usize = 3;

// Constants for display screen size
pub const SCREEN_WIDTH: usize = DISPLAY_WIDTH * SCALE_FACTOR;
pub const SCREEN_HEIGHT: usize = DISPLAY_HEIGHT * SCALE_FACTOR;

// Constants for memory size
pub const KILOBYTE: usize = 1024;
pub const RAM_SIZE: usize = 4 * KILOBYTE;
pub const STACK_SIZE: usize = 16;
pub const KEYPAD_SIZE: usize = 16;

// Constants for program addresses
pub const PROGRAM_START: usize = 0x200;
pub const PROGRAM_END: usize = 0xFFF;

// Constants for fontset
pub const FONTSET_START: usize = 0x000;
pub const FONTSET_END: usize = 0x1FF;
