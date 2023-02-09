//mod bevy_;
mod chip8;
mod sdl;

fn main() {
    let rom = std::fs::read("roms/demos/Stars [Sergey Naydenov, 2010].ch8").unwrap();
    let mut ch8 = chip8::Emulator::init(rom);
    sdl::sdl_init(&mut ch8);
}
