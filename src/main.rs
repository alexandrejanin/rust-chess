extern crate cuivre;
extern crate floating_duration;
extern crate rand;
#[macro_use]
extern crate serde;

use cuivre::{
    graphics::{camera::Camera, manager::GraphicsManager, sprites},
    input,
    maths::{Point3f, Vector2i, Vector2u, Vector3f},
    resources,
    transform::Transform,
};
use floating_duration::TimeAsFloat;
use std::{path::Path, time::SystemTime};

mod config;

// Main
fn main() {
    //Initialize time
    let start_time = SystemTime::now();
    let mut last_time = start_time;

    //Initialize ResourceLoader
    let resource_loader = resources::ResourceLoader::new("res".as_ref())
        .unwrap_or_else(|error| panic!("ERROR: Could not initialize resource loader.\n{}", error));

    //Load configuration from file
    let conf_path = Path::new("config.ron");
    let conf = resource_loader
        .load_object::<config::Config>(conf_path)
        .unwrap_or_else(|error| {
            panic!(
                "ERROR: Could not load config file from {:?}.\n{}",
                conf_path, error
            )
        });

    //Initialize SDL
    let sdl = cuivre::init_sdl().expect("ERROR: Could not initialize SDL");

    //Initialize graphics
    let mut graphics_manager = GraphicsManager::new(
        &sdl,
        &resource_loader,
        "shaders/standard.vert".as_ref(),
        "shaders/standard.frag".as_ref(),
        "RustChess",
        conf.video.width,
        conf.video.height,
        conf.video.vsync,
    ).expect("ERROR: Could not initialize graphics manager");

    //Initialize events
    let mut events = sdl
        .event_pump()
        .expect("ERROR: Could not initialize Event Pump");

    //Initialize input
    let mut input_manager = input::InputManager::new();

    //Create camera
    let mut camera = Camera::from_height(
        Point3f::new(4.0, 4.0, 1.0),
        Vector3f::new(0.0, 0.0, -1.0),
        0.1,
        10.0,
        false,
        8.0,
        graphics_manager.window_size(),
    );

    //Load tiles texture
    let tiles_texture = graphics_manager
        .get_texture("sprites/tiles.png".as_ref())
        .unwrap();
    let tiles_sheet = sprites::SpriteSheet::new(tiles_texture, Vector2u::new(16, 16));
    let mut tile_sprite = sprites::Sprite::new(tiles_sheet, Vector2i::new(0, 0));

    //Load pieces texture
    let pieces_texture = graphics_manager
        .get_texture("sprites/pieces.png".as_ref())
        .unwrap();
    let pieces_sheet = sprites::SpriteSheet::new(pieces_texture, Vector2u::new(16, 16));
    let mut sprite = sprites::Sprite::new(pieces_sheet, Vector2i::new(3, 0));

    let mut transform = Transform::from_position((0.5, 0.5, 0.0).into());

    println!(
        "Startup took {:?}",
        SystemTime::now().duration_since(start_time).unwrap()
    );

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
                cuivre::Event::Quit { .. } => break 'main,
                cuivre::Event::Window { win_event, .. } => match win_event {
                    cuivre::WindowEvent::SizeChanged(width, height) => {
                        graphics_manager.resize(width, height);
                        camera.resize_keep_height(width, height);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        //Update input manager
        input_manager.update(&events);

        //Change sprite
        if input_manager.key_released(input::Keycode::Left) {
            sprite.position.x -= 1
        }
        if input_manager.key_released(input::Keycode::Right) {
            sprite.position.x += 1
        }
        if input_manager.key_released(input::Keycode::Up) {
            sprite.position.y -= 1
        }
        if input_manager.key_released(input::Keycode::Down) {
            sprite.position.y += 1
        }

        //Move piece around
        if input_manager.key_pressed(input::Keycode::A) {
            transform.position.x -= 1.0
        }
        if input_manager.key_pressed(input::Keycode::D) {
            transform.position.x += 1.0
        }
        if input_manager.key_pressed(input::Keycode::W) {
            transform.position.y += 1.0
        }
        if input_manager.key_pressed(input::Keycode::S) {
            transform.position.y -= 1.0
        }

        transform.rotation.z = 100.0 * elapsed_seconds;

        //draw board
        for x in 0..8 {
            for y in 0..8 {
                let mut tile_transform =
                    Transform::from_position(Point3f::new(x as f32 + 0.5, y as f32 + 0.5, -1.0));

                tile_transform.rotation.x = x as f32 * 20.0 * elapsed_seconds;
                tile_transform.rotation.y = y as f32 * 20.0 * elapsed_seconds;
                tile_transform.rotation.z = (x + y) as f32 * 20.0 * elapsed_seconds;

                tile_transform.scale = Vector3f::new(0.8, 0.8, 0.8);

                tile_sprite.position.y = y + x;
                graphics_manager.draw_sprite(tile_sprite, tile_transform, &camera);
            }
        }

        //draw pieces
        graphics_manager.draw_sprite(sprite, transform, &camera);

        //Render
        graphics_manager
            .render()
            .expect("ERROR: Rendering failed, exiting.");

        //Limit fps
        //std::thread::sleep(Duration::from_millis(1));
    }
}
