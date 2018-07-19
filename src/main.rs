extern crate gl;
extern crate ron;
extern crate sdl2;
#[macro_use]
extern crate serde;

mod config;
mod graphics;
mod input;

fn main() {
    //Load Configuration from file
    let conf = config::Config::from_file(std::path::Path::new("res/config.ron")).unwrap();

    //Initialize SDL
    let sdl = sdl2::init().unwrap();

    //Initialize graphics
    let mut graphics_manager = graphics::GraphicsManager::new(&conf, &sdl);
    graphics_manager.init();

    //Initialize events
    let mut events = sdl.event_pump().unwrap();

    let mut input_manager = input::InputManager::new();

    //Main loop
    'main: loop {
        //Handle events
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        //Game Logic

        input_manager.update(&events);

        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Space) {
            println!("Space pressed!");
        }

        //Draw
        let render_result = graphics_manager.render();
        match render_result {
            Ok(_) => {},
            Err(message) => {
                println!("{}", message);
                break 'main
            },
        }
    }
}
