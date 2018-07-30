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

use graphics::{
    camera::{OrthographicCamera, PerspectiveCamera},
    manager::GraphicsManager,
    sprites
};
use maths::{Vector2i, Vector2u, Vector3f};
use std::path::Path;
use std::time::{Duration, SystemTime};
use transform::Transform;


mod config;
mod input;
mod resources;

mod maths;
mod graphics;
mod transform;

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
    let mut transform2 = Transform::new();

    //Create camera
    //let mut ortho_camera = OrthographicCamera::from_width((0.0, 1.0, 10.0).into(), 5.0, 100.0, graphics_manager.window_size());
    let mut camera = PerspectiveCamera::new(Vector3f::new(0.0, 0.0, 10.0), 60.0, 0.1, 100.0);


    println!("Startup took {} ms.", (SystemTime::now().duration_since(start_time)).unwrap().subsec_millis());


    //Main loop
    'main: loop {
        //Handle events
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window { win_event, .. } =>
                    match win_event {
                        sdl2::event::WindowEvent::SizeChanged(width, height) => {
                            graphics_manager.resize(width, height);
                            //camera.resize_keep_width((width as u32, height as u32).into())
                        },
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

        let elapsed_seconds = SystemTime::now().duration_since(start_time).unwrap().as_fractional_secs();

        transform.position.y = (elapsed_seconds / 2.0).cos() as f32;
        transform.position.x = (elapsed_seconds / 2.0).sin() as f32;

        transform2.position.z = elapsed_seconds.sin() as f32;

        //Clear
        graphics_manager.clear();

        //Draw
        graphics_manager.draw_sprite(sprite, transform, Some(&camera));
        graphics_manager.draw_sprite(sprite, transform2, Some(&camera));

        //Render
        graphics_manager.render().expect("ERROR: Rendering failed, exiting.");

        //Limit fps
        std::thread::sleep(Duration::from_millis(1));
    }
}
