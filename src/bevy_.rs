use crate::chip8::Emulator;
use bevy::prelude::*;
use std::env;
use std::fs;

pub fn bevy_start() {
    App::new()
        .add_startup_system(init_ch8)
        .add_startup_system(test)
        //.add_startup_system(get_rom)
        //.add_startup_system(load_rom)
        //.add_system(main_loop)
        .run()
}
#[derive(Bundle)]
struct C8Bundle {
    c8_component: Emulator,
}

fn main_loop(mut c8: Query<(&mut Emulator)>) {
    let mut ch8 = c8.single_mut();
    Emulator::tick(&mut ch8);
}

fn test(mut ch8: Query<&mut Emulator>) {
    let mut chip = ch8.single_mut();
}

fn init_ch8(mut cmd: Commands) {
    cmd.spawn_bundle(C8Bundle {
        c8_component: Emulator::init(),
    });
}

fn draw(mut ch8: Query<(&mut Emulator)>) {
    println!("{:?}", ch8.single_mut().display_buffer)
}

/// Gets the path from std::env returns the rom as Vec<u8>
pub fn get_rom(mut ch8: Query<(&mut Emulator)>) {
    let mut chip8 = ch8.single_mut();
    let args: Vec<String> = env::args().collect();

    let path;
    let rom;

    if args.len() > 1 {
        path = &args[1];
        rom = fs::read(path).expect("something went wrong while reading the file");
    } else {
        panic!("Usage: chip8 <path/to/rom>")
    }

    chip8.rom = rom;
}

fn load_rom(mut ch8: Query<&mut Emulator>) {
    let mut chip8 = ch8.single_mut();
    // check the rom size
    if chip8.rom.len() > 3584 {
        panic!("not a chip8 rom")
    }
    let rom = chip8.rom.clone();
    chip8.memory[512..].clone_from_slice(&rom);

    let a: i32 = 12;
    let b = a.clone();
    println!("{}", b)
}
