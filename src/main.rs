// Load extern crates
extern crate gl;
extern crate cgmath;
extern crate image;
extern crate ron;
extern crate sdl2;
#[macro_use]
extern crate serde;


// Load local modules
mod config;
mod graphics;
mod input;
mod resources;


use std::path::Path;


use graphics::manager::GraphicsManager;


// Main
fn main() {
    //Initialize ResourceLoader
    let resource_loader = match resources::ResourceLoader::new() {
        Ok(resource_loader) => resource_loader,
        Err(error) => panic!("ERROR: Could not initialize ResourceLoader, exiting.\n{}", error),
    };

    //Load Configuration from file
    let conf = match config::Config::from_file(&resource_loader, Path::new("config.ron")) {
        Ok(conf) => conf,
        Err(error) => panic!("ERROR: Could not load config file, exiting.\n{}", error),
    };

    //Initialize SDL
    let sdl = sdl2::init().unwrap();

    //Initialize graphics
    let mut graphics_manager = match GraphicsManager::new(&resource_loader, &conf, &sdl) {
        Ok(graphics_manager) => graphics_manager,
        Err(error) => panic!("ERROR: Could not initialize graphics, exiting.\n{}", error),
    };

    //Initialize events
    let mut events = sdl.event_pump().unwrap();

    //Initialize input
    let mut input_manager = input::InputManager::new();

    //Create sprite
    let sprite = match graphics_manager.new_sprite(Path::new("terrain.png")) {
        Ok(sprite) => sprite,
        Err(error) => panic!("ERROR: Could not load sprite, exiting.\n{}", error),
    };

    //Main loop
    'main: loop {
        //Handle events
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window { win_event, .. } =>
                    match win_event {
                        sdl2::event::WindowEvent::SizeChanged(width, height) => graphics_manager.resize(width, height),
                        _ => {}
                    },
                _ => {}
            }
        }

        input_manager.update(&events);

        //Game Logic
        //...
        //...

        //Clear
        graphics_manager.clear();

        //Draw
        if let Err(error) = graphics_manager.draw_sprite(sprite) {
            panic!("Error: Could not draw sprite, exiting. {}", error)
        };

        //Render
        graphics_manager.render();

        //Limit fps
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
