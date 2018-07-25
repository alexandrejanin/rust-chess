extern crate cgmath;
// Load extern crates
extern crate gl;
extern crate image;
extern crate ron;
extern crate sdl2;
#[macro_use]
extern crate serde;

use graphics::manager::GraphicsManager;
use graphics::sprites;
use std::path::Path;
use std::time::{Duration, SystemTime};

// Load local modules
mod config;
mod graphics;
mod input;
mod resources;


// Main
fn main() {
    //Initialize time
    let start_time = SystemTime::now();

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

    //Create texture
    let terrain_texture = match graphics_manager.get_texture(Path::new("terrain.png")) {
        Ok(sprite_sheet) => sprite_sheet,
        Err(error) => panic!("ERROR: Could not load sprite, exiting.\n{}", error),
    };

    //Create sprite sheet
    let terrain_sheet = sprites::SpriteSheet::new(terrain_texture, (16, 16).into());

    //Create sprites
    let dirt_sprite = sprites::Sprite::new(terrain_sheet, (3, 0).into());
    let tnt = sprites::Sprite::new(terrain_sheet, (8, 0).into());


    println!("Initialization took {} ms.", (SystemTime::now().duration_since(start_time)).unwrap().subsec_millis());


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

        //Clear
        graphics_manager.clear();

        //Draw
        if let Err(error) = graphics_manager.draw_sprite(dirt_sprite) {
            panic!("ERROR: Could not draw sprite, exiting.\n{}", error)
        }

        if (SystemTime::now().duration_since(start_time)).unwrap().as_secs() % 2 == 0 {
            graphics_manager.draw_sprite(tnt);
        }


        //Render
        graphics_manager.render();

        //Limit fps
        std::thread::sleep(Duration::from_millis(1));
    }
}
