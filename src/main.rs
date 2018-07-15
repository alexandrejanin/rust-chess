#[macro_use] extern crate serde_derive;
extern crate sdl2;
extern crate time;

use sdl2::event::Event;
use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::time::Duration;
use std::path::Path;

mod input;
mod graphics;
mod config;

pub fn main() {
    //Load config from toml file
    let conf = config::Config::from_file(Path::new("./res/config.toml"));

    //Create context and init video
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let image_context = sdl2::image::init(INIT_PNG).unwrap();

    //Create window
    let window = video_subsystem
        .window("Rust/SDL2 Game", conf.display.width, conf.display.height)
        .position_centered()
        //.opengl()
        .build()
        .unwrap();

    //Create canvas for window
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    //Create texture creator
    let texture_creator = canvas.texture_creator();

    //Create event pump for window
    let mut event_pump = sdl_context.event_pump().unwrap();

    //Create input manager
    let mut input_manager = input::InputManager::new();

    let mut sprite = graphics::Sprite::new("./res/img.png");
    let texture = texture_creator.load_texture(sprite.path).unwrap();

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
        } else if input_manager.get_key_released(Keycode::Space) {
            canvas.set_draw_color(Color::RGB(0, 255, 0))
        } else if input_manager.get_key_down(Keycode::Space) {
            canvas.set_draw_color(Color::RGB(0, 0, 255))
        } else {
            canvas.set_draw_color(Color::RGB(0, 0, 0))
        }

        canvas.clear();

        //Draw stuff
        
        let offset: i32 = (time::precise_time_s().sin() * 50.) as i32;
        
        sprite.dst_rect = sdl2::rect::Rect::new(0, 0, (200 + offset) as u32, 200);
        canvas.copy(&texture, sprite.src_rect, sprite.dst_rect);

        canvas.present();

        std::thread::sleep(Duration::from_micros(1_000_000 / conf.display.max_fps));
    }
}

