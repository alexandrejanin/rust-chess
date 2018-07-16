#[macro_use] extern crate serde_derive;
extern crate sdl2;
extern crate time;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::path::Path;

mod input;
mod graphics;
mod config;

pub fn main() {
    //Load config from toml file
    let conf = config::Config::from_file(Path::new("./res/config.toml"));

    //Initialize SDL context and DisplayManager
    let sdl_context = sdl2::init().unwrap();
    let display_manager = graphics::DisplayManager::new("RustGame", &sdl_context, &conf.display);

    //Create event pump for window
    let mut event_pump = sdl_context.event_pump().unwrap();

    //Create input manager
    let mut input_manager = input::InputManager::new();

    let sprite = graphics::Sprite::new("./res/img.png");

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

        display_manager.clear();

        //Draw stuff

        display_manager.draw(sprite, sdl::rect::Point{x:0, y:0});

        display_manager.render();

        std::thread::sleep(Duration::from_micros(1_000_000 / conf.display.max_fps));
    }
}

