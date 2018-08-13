extern crate cuivre;
extern crate floating_duration;
extern crate rand;
#[macro_use]
extern crate serde;

use cuivre::{
    graphics::{Camera, CameraScaleMode, GraphicsManager, Sprite, SpriteSheet},
    input,
    maths::{Point3f, Vector2u, Vector3f},
    resources,
    transform::Transform,
};
use floating_duration::TimeAsFloat;
use game::PiecesManager;
use std::{error, path::Path, time::SystemTime};

mod config;
mod game;

// Main
fn main() -> Result<(), Box<error::Error>> {
    //Initialize time
    let start_time = SystemTime::now();
    let mut last_time = start_time;

    //Initialize ResourceLoader
    let resource_loader = resources::ResourceLoader::new("res".as_ref())?;

    //Load configuration from file
    let conf_path = Path::new("config.ron");
    let conf = resource_loader.load_object::<config::Config>(conf_path)?;

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
    )?;

    //Initialize events
    let mut events = sdl.event_pump()?;

    //Initialize input
    let mut input_manager = input::InputManager::new();

    //Create camera
    let camera = Camera {
        position: Point3f::new(4.0, 4.0, 1.0),
        direction: Vector3f::new(0.0, 0.0, -1.0),
        near: 0.1,
        far: 10.0,
        size: 9.0,
        scale_mode: CameraScaleMode::Min,
        perspective: false
    };

    //Load tiles texture
    let tiles_texture = graphics_manager.get_texture("sprites/tiles.png".as_ref())?;
    let tiles_sheet = SpriteSheet::new(tiles_texture, Vector2u::new(16, 16));

    //Load pieces texture
    let pieces_texture = graphics_manager.get_texture("sprites/pieces.png".as_ref())?;
    let pieces_sheet = SpriteSheet::new(pieces_texture, Vector2u::new(16, 16));

    //Create pieces manager
    let pieces_manager = PiecesManager::new();

    println!(
        "Startup took {:?}",
        SystemTime::now().duration_since(start_time)?
    );

    //Main loop
    'main: loop {
        //Current time
        let now = SystemTime::now();

        //Total elapsed time
        let elapsed_seconds = now.duration_since(start_time)?.as_fractional_secs() as f32;

        //Delta time
        let delta_time = now.duration_since(last_time)?.as_fractional_secs() as f32;
        last_time = now;

        //Handle events
        for event in events.poll_iter() {
            match event {
                cuivre::Event::Quit { .. } => break 'main,
                cuivre::Event::Window { win_event, .. } => {
                    if let cuivre::WindowEvent::SizeChanged(width, height) = win_event {
                        graphics_manager.resize(width, height);
                    }
                }
                _ => {}
            }
        }

        input_manager.update(&events);

        if input_manager.button(input::MouseButton::Left)?.pressed() {
            println!("Clicked at {:?}", input_manager.mouse_position());
        }

        //draw board
        for x in 0..8 {
            for y in 0..8 {
                let mut tile_transform =
                    Transform::from_position(Point3f::new(x as f32 + 0.5, y as f32 + 0.5, -1.0));

                let tile_sprite = tiles_sheet.sprite(0, x + y);
                graphics_manager.draw_sprite(tile_sprite, tile_transform, &camera);
            }
        }

        //draw pieces
        for piece in &pieces_manager.pieces {
            let piece_sprite = piece.sprite(&pieces_sheet);
            let transform = piece.transform();
            graphics_manager.draw_sprite(piece_sprite, transform, &camera);
        }
        //Render
        graphics_manager.render()?;

        //Limit fps
        //std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
