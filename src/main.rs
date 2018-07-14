extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::time::Duration;

mod input;

pub fn main() {
    //Create context and init video
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let max_fps = 60;

    //Create window
    let window = video_subsystem
        .window("Rust/SDL2 Game", 800, 600)
        .position_centered()
        //.opengl()
        .build()
        .unwrap();

    //Create canvas for window
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    //Create event pump for window
    let mut event_pump = sdl_context.event_pump().unwrap();

    //Create input manager
    let mut input_manager = input::InputManager::new();

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,

                _ => {}
            }
        }
        
        // The rest of the game loop goes here...
        input_manager.update(&event_pump);

        if input_manager.get_key_pressed(Keycode::Space) {
            canvas.set_draw_color(Color::RGB(255, 0, 0))
        } else if input_manager.get_key_down(Keycode::Space) {
            canvas.set_draw_color(Color::RGB(0, 0, 255))
        } else {
            canvas.set_draw_color(Color::RGB(0, 0, 0))
        }

        canvas.clear();

        //Draw stuff

        canvas.present();

        std::thread::sleep(Duration::from_micros(1_000_000 / max_fps));
    }
}
