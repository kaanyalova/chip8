use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashSet;

use crate::chip8::Emulator;

const SCREEN_SIZE_MULTIPLYER: u32 = 10;
const EMULATOR_SPEED_MULTIPLIER: usize = 25;

pub fn sdl_init(ch8: &mut Emulator) {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "chip8",
            64 * SCREEN_SIZE_MULTIPLYER,
            32 * SCREEN_SIZE_MULTIPLYER,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(60).unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // set all keys to false at the start of the frame
        for input in ch8.key_inputs.iter_mut() {
            *input = false;
        }

        let keys: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        for key in keys {
            match key {
                Keycode::Num1 => ch8.key_inputs[0] = true,
                Keycode::Num2 => ch8.key_inputs[1] = true,
                Keycode::Num3 => ch8.key_inputs[2] = true,
                Keycode::Num4 => ch8.key_inputs[3] = true,
                Keycode::Q => ch8.key_inputs[4] = true,
                Keycode::W => ch8.key_inputs[5] = true,
                Keycode::E => ch8.key_inputs[6] = true,
                Keycode::R => ch8.key_inputs[7] = true,
                Keycode::A => ch8.key_inputs[8] = true,
                Keycode::S => ch8.key_inputs[9] = true,
                Keycode::D => ch8.key_inputs[10] = true,
                Keycode::F => ch8.key_inputs[11] = true,
                Keycode::Z => ch8.key_inputs[12] = true,
                Keycode::X => ch8.key_inputs[13] = true,
                Keycode::C => ch8.key_inputs[14] = true,
                Keycode::V => ch8.key_inputs[15] = true,

                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for _ in 0..EMULATOR_SPEED_MULTIPLIER {
            ch8.tick();
        }
        ch8.timer_tick();

        canvas.set_draw_color(Color::WHITE);

        for (x_pos, x_arr) in ch8.display_buffer.iter().enumerate() {
            for (y_pos, pixel_state) in x_arr.iter().enumerate() {
                if *pixel_state == true {
                    canvas
                        .fill_rect(Rect::new(
                            (x_pos * SCREEN_SIZE_MULTIPLYER as usize) as i32,
                            (y_pos * SCREEN_SIZE_MULTIPLYER as usize) as i32,
                            SCREEN_SIZE_MULTIPLYER,
                            SCREEN_SIZE_MULTIPLYER,
                        ))
                        .unwrap();
                }
            }
        }

        canvas.present();

        fps_manager.delay();
    }
}
