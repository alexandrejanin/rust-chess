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
    let conf = match config::Config::from_file(std::path::Path::new("res/config.ron")) {
        Ok(conf) => conf,
        Err(error) => {
            println!("ERROR: Could not load config file, exiting.\n{}", error);
            return;
        },
    };

    //Initialize SDL
    let sdl = sdl2::init().unwrap();

    //Initialize graphics
    let mut graphics_manager = graphics::GraphicsManager::new(&conf, &sdl);

    if let Err(error) = graphics_manager.init() {
        println!("ERROR: Graphics initialization failed, exiting.\n{}", error);
        return;
    };

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
