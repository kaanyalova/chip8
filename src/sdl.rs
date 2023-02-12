use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashSet;

use crate::chip8::Emulator;
use crate::sound::*;

pub fn sdl_init(ch8: &mut Emulator, speed: u32, screen_size: u32, framerate: u32) {
    // Sound
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = setup_beep(stream_handle);

    // Video
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("chip8", 64 * screen_size, 32 * screen_size)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(framerate).unwrap();

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

        if ch8.sound_timer != 0 {
            start_beep(&sink);
        }

        if ch8.sound_timer == 0 {
            stop_beep(&sink);
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
                Keycode::X => ch8.key_inputs[0] = true,
                Keycode::Num1 => ch8.key_inputs[1] = true,
                Keycode::Num2 => ch8.key_inputs[2] = true,
                Keycode::Num3 => ch8.key_inputs[3] = true,
                Keycode::Q => ch8.key_inputs[4] = true,
                Keycode::W => ch8.key_inputs[5] = true,
                Keycode::E => ch8.key_inputs[6] = true,
                Keycode::A => ch8.key_inputs[7] = true,
                Keycode::S => ch8.key_inputs[8] = true,
                Keycode::D => ch8.key_inputs[9] = true,
                Keycode::Z => ch8.key_inputs[10] = true,
                Keycode::C => ch8.key_inputs[11] = true,
                Keycode::Num4 => ch8.key_inputs[12] = true,
                Keycode::R => ch8.key_inputs[13] = true,
                Keycode::F => ch8.key_inputs[14] = true,
                Keycode::V => ch8.key_inputs[15] = true,

                _ => {}
            }
        }

        if ch8.draw {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
        }
        for _ in 0..speed {
            ch8.tick();
        }
        ch8.timer_tick();

        canvas.set_draw_color(Color::WHITE);

        for (x_pos, x_arr) in ch8.display_buffer.iter().enumerate() {
            for (y_pos, pixel_state) in x_arr.iter().enumerate() {
                if *pixel_state == true {
                    canvas
                        .fill_rect(Rect::new(
                            (x_pos * screen_size as usize) as i32,
                            (y_pos * screen_size as usize) as i32,
                            screen_size,
                            screen_size,
                        ))
                        .unwrap();
                }
            }
        }

        canvas.present();

        fps_manager.delay();
    }
}
