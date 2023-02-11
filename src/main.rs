//mod bevy_;
mod chip8;
mod logger;
mod sdl;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// Path of the chip8 executable
    path: PathBuf,

    #[arg(short = 's', long, default_value_t = 20)]
    /// Speed of the emulator , how many cycles runs per frame
    speed: u32,

    /// Window dimensions, multiplied by 32*64
    #[arg(short = 'S', long, default_value_t = 10)]
    screen_size: u32,

    /// Framerate
    #[arg(short = 'f', long, default_value_t = 120)]
    framerate: u32,
}

fn main() {
    logger::init_logger();
    let args = Cli::parse();

    let rom = std::fs::read(args.path).unwrap();
    let mut ch8 = chip8::Emulator::init(rom);
    sdl::sdl_init(&mut ch8, args.speed, args.screen_size, args.framerate);
}
