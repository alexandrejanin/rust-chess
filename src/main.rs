// Load extern crates
extern crate gl;
extern crate image;
extern crate ron;
extern crate sdl2;
#[macro_use]
extern crate serde;

use graphics::manager::GraphicsManager;
use graphics::sprites;
use maths::{
    transform::Transform, Vector2i,
    Vector2u,
};
use std::path::Path;
use std::time::{Duration, SystemTime};

// Load local modules
mod config;
mod input;
mod resources;

mod maths;
mod graphics;


// Main
fn main() {
    //Initialize time
    let start_time = SystemTime::now();

    //Initialize ResourceLoader
    let resource_loader = resources::ResourceLoader::new()
        .expect("ERROR: Could not initialize resource loader");

    //Load Configuration from file
    let conf_path: &Path = Path::new("config.ron");
    let conf = config::Config::from_file(&resource_loader, conf_path)
        .expect(&format!("ERROR: Could not load config file from '{:?}'", conf_path));

    //Initialize SDL
    let sdl = sdl2::init()
        .expect("ERROR: Could not initialize SDL");

    //Initialize graphics
    let mut graphics_manager = GraphicsManager::new(&resource_loader, &conf, &sdl)
        .expect("ERROR: Could not initialize graphics manager");

    //Initialize events
    let mut events = sdl.event_pump()
                        .expect("ERROR: Could not initialize Event Pump");

    //Initialize input
    let mut input_manager = input::InputManager::new();


    //Create texture
    let terrain_path: &Path = Path::new("terrain.png");
    let terrain_texture = graphics_manager.get_texture(terrain_path)
                                          .expect(&format!("ERROR: Could not load texture from '{:?}'", terrain_path));

    //Create sprite sheet
    let terrain_sheet = sprites::SpriteSheet::new(terrain_texture, Vector2u::new(16, 16));

    //Create sprites
    let mut sprite = sprites::Sprite::new(terrain_sheet, Vector2i::new(3, 0));

    //Create transform
    let mut transform = Transform::new();

    println!("Startup took {} ms.", (SystemTime::now().duration_since(start_time)).unwrap().subsec_millis());


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

        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Left) { sprite.position.x -= 1 }
        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Right) { sprite.position.x += 1 }
        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Up) { sprite.position.y -= 1 }
        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Down) { sprite.position.y += 1 }

        //Clear
        graphics_manager.clear();

        let time = SystemTime::now().duration_since(start_time).unwrap();

        transform.position.x = (time.as_secs() as f32 + time.subsec_nanos() as f32 / 1_000_000_000 as f32).sin() / 2.0;

        //Draw
        graphics_manager.draw_sprite(sprite, transform)
                        .expect(&format!("ERROR: Could not draw sprite ({:?})", sprite));

        //Render
        graphics_manager.render();

        //Limit fps
        std::thread::sleep(Duration::from_millis(1));
    }
}
