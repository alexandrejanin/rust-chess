// Load extern crates
extern crate floating_duration;
extern crate cgmath;
extern crate gl;
extern crate image;
extern crate rand;
extern crate ron;
extern crate sdl2;
#[macro_use]
extern crate serde;

use floating_duration::TimeAsFloat;

use graphics::{
    camera::Camera,
    manager::GraphicsManager,
    sprites
};
use maths::{Point3f, Vector2i, Vector2u, Vector3f};
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
    let mut last_time = start_time;

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
    let mut sprite2 = sprites::Sprite::new(terrain_sheet, Vector2i::new(0, 0));

    //Create transforms
    let mut transform = Transform::new();
    let mut transform2 = Transform::new();

    //Create camera
    let mut camera = Camera::from_width(
        Point3f::new(-5.0, 0.0, 10.0), Vector3f::new(0.5, 0.0, -1.0),
        true,
        0.1, 100.0,
        60.0, graphics_manager.window_size()
    );
    let mut cam_angle = 0.0;

    println!("Startup took {} ms.", (SystemTime::now().duration_since(start_time)).unwrap().subsec_millis());

    //Main loop
    'main: loop {
        //Current time
        let now = SystemTime::now();

        //Total elapsed time
        let elapsed_seconds = now.duration_since(start_time).unwrap().as_fractional_secs();

        //Delta time
        let delta_time = now.duration_since(last_time).unwrap().as_fractional_secs();
        last_time = now;

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

        //Update input manager
        input_manager.update(&events);

        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Left) { sprite.position.x -= 1 }
        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Right) { sprite.position.x += 1 }
        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Up) { sprite.position.y -= 1 }
        if input_manager.get_key_pressed(sdl2::keyboard::Keycode::Down) { sprite.position.y += 1 }

        if input_manager.get_key_down(sdl2::keyboard::Keycode::A) { cam_angle -= delta_time }
        if input_manager.get_key_down(sdl2::keyboard::Keycode::D) { cam_angle += delta_time }


        //make sprite orbit vertically around origin
        transform.position.y = elapsed_seconds.cos() as f32;
        transform.position.x = elapsed_seconds.sin() as f32;

        //make camera orbit around origin
        camera.position.z = 10.0 * cam_angle.cos() as f32;
        camera.position.x = 10.0 * cam_angle.sin() as f32;

        camera.look_at(Point3f::new(0.0, 1.0, 0.0));

        //Clear
        graphics_manager.clear();

        //Draw
        graphics_manager.draw_sprite(sprite, transform, Some(&camera));
        graphics_manager.draw_sprite(sprite2, transform2, Some(&camera));

        //Render
        graphics_manager.render().expect("ERROR: Rendering failed, exiting.");

        //Limit fps
        std::thread::sleep(Duration::from_millis(1));
    }
}
