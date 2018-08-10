extern crate cgmath;
// Load extern crates
extern crate floating_duration;
extern crate gl;
extern crate image;
extern crate rand;
extern crate ron;
extern crate sdl2;
#[macro_use]
extern crate serde;

use floating_duration::TimeAsFloat;
use graphics::{camera::Camera, manager::GraphicsManager, sprites};
use maths::{Point3f, Vector2i, Vector2u, Vector3f};
use std::path::Path;
use std::time::SystemTime;
use transform::Transform;

mod config;
mod input;
mod resources;

mod graphics;
mod maths;
mod transform;

// Main
fn main() {
    //Initialize time
    let start_time = SystemTime::now();
    let mut last_time = start_time;

    //Initialize ResourceLoader
    let resource_loader = resources::ResourceLoader::new()
        .expect("ERROR: Could not initialize resource loader");

    //Load Configuration from file
    let conf_path: &Path = Path::new("config.ron");
    let conf = config::Config::from_file(&resource_loader, conf_path)
        .expect(&format!("ERROR: Could not load config file from '{:?}'", conf_path));

    //Initialize SDL
    let sdl = sdl2::init().expect("ERROR: Could not initialize SDL");

    //Initialize graphics
    let mut graphics_manager = GraphicsManager::new(&resource_loader, &conf, &sdl)
        .expect("ERROR: Could not initialize graphics manager");

    //Initialize events
    let mut events = sdl.event_pump().expect("ERROR: Could not initialize Event Pump");

    //Initialize input
    let mut input_manager = input::InputManager::new();


    //Create camera
    let camera = Camera::from_height(
        Point3f::new(4.0, 4.0, 1.0),
        Vector3f::new(0.0, 0.0, -1.0),
        0.1,
        10.0,
        false,
        8.0,
        graphics_manager.window_size(),
    );


    //Load tiles texture
    let tiles_texture = graphics_manager.get_texture("sprites/tiles.png".as_ref()).unwrap();
    let tiles_sheet = sprites::SpriteSheet::new(tiles_texture, Vector2u::new(16, 16), );
    let mut tile_sprite = sprites::Sprite::new(tiles_sheet, Vector2i::new(0, 0));

    //Load pieces texture
    let pieces_texture = graphics_manager.get_texture("sprites/pieces.png".as_ref()).unwrap();
    let pieces_sheet = sprites::SpriteSheet::new(pieces_texture, Vector2u::new(16, 16), );
    let mut sprite = sprites::Sprite::new(pieces_sheet, Vector2i::new(3, 0));

    let mut transform = Transform::new();


    println!("Startup took {:?}", SystemTime::now().duration_since(start_time).unwrap());


    //Main loop
    'main: loop {
        //Current time
        let now = SystemTime::now();

        //Total elapsed time
        let elapsed_seconds = now.duration_since(start_time).unwrap().as_fractional_secs() as f32;

        //Delta time
        let delta_time = now.duration_since(last_time).unwrap().as_fractional_secs() as f32;
        last_time = now;

        //Handle events
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window { win_event, .. } => match win_event {
                    sdl2::event::WindowEvent::SizeChanged(width, height) => {
                        graphics_manager.resize(width, height);
                        //camera.resize_keep_width((width as u32, height as u32).into())
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        //Update input manager
        input_manager.update(&events);

        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Left) {
            sprite.position.x -= 1
        }
        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Right) {
            sprite.position.x += 1
        }
        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Up) {
            sprite.position.y -= 1
        }
        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Down) {
            sprite.position.y += 1
        }

        //make sprite orbit vertically around origin
        transform.position.y = 0.5 + elapsed_seconds.cos() / 2.0;
        transform.position.x = 0.5 + elapsed_seconds.sin() / 2.0;

        //Clear
        graphics_manager.clear();

        //draw board
        for x in 0..8 {
            for y in 0..8 {
                let tile_transform = Transform::from_position(Point3f::new(x as f32 + 0.5, y as f32 + 0.5, -1.0));
                tile_sprite.position.y = y + x;
                graphics_manager.draw_sprite(tile_sprite, tile_transform, &camera);
            }
        }

        //draw pieces
        graphics_manager.draw_sprite(sprite, transform, &camera);

        //Render
        graphics_manager.render().expect("ERROR: Rendering failed, exiting.");

        //Limit fps
        //std::thread::sleep(Duration::from_millis(1));
    }
}
