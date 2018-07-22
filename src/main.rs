// Load extern crates
extern crate gl;
extern crate glm;
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
use std::time;


use graphics::manager::GraphicsManager;


// Main
fn main() {

    //Initialize ResourceLoader
    let resource_loader = match resources::ResourceLoader::new() {
        Ok(resource_loader) => resource_loader,
        Err(error) => {
            println!("ERROR: Could not initialize ResourceLoader, exiting.\n{:?}", error);
            return;
        },
    };

    //Load Configuration from file
    let conf = match config::Config::from_file(&resource_loader, Path::new("config.ron")) {
        Ok(conf) => conf,
        Err(error) => {
            println!("ERROR: Could not load config file, exiting.\n{}", error);
            return;
        },
    };

    //Initialize SDL
    let sdl = sdl2::init().unwrap();

    //Initialize graphics
    let mut graphics_manager = GraphicsManager::new(&conf, &sdl);

    if let Err(error) = graphics_manager.init(&resource_loader) {
        println!("ERROR: Graphics initialization failed, exiting.\n{:?}", error);
        return;
    };

    //Initialize events
    let mut events = sdl.event_pump().unwrap();

    //Initialize input
    let mut input_manager = input::InputManager::new();


    //Load texture
    let terrain_id = graphics_manager.get_texture(&resource_loader, Path::new("terrain.png")).unwrap();


    //Initialize timer
    let mut last_time = time::SystemTime::now();


    //Main loop
    'main: loop {
        //Handle events
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit { timestamp } => break 'main,
                sdl2::event::Event::Window { timestamp, window_id, win_event } =>
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
        graphics_manager.draw_sprite(terrain_id);

        //Render
        graphics_manager.render();

        std::thread::sleep(std::time::Duration::from_millis(1));

        //fps
        let new_time = time::SystemTime::now();
        let micro_seconds = new_time.duration_since(last_time).unwrap().subsec_micros();
        let fps = 1_000_000. / micro_seconds as f32;
        println!("{} fps", fps);
        last_time = new_time;
    }
}
