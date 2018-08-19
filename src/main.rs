extern crate cuivre;
extern crate floating_duration;
extern crate rand;
extern crate ron;
#[macro_use]
extern crate serde;

use cuivre::{
    graphics::{
        camera::{Camera, CameraScaleMode},
        sprites::{Sprite, SpriteSheet},
        text::{Font, TextSettings},
        textures::{Texture, TextureOptions},
        GraphicsManager, WindowSettings,
    },
    input::{Event, InputManager, Keycode, WindowEvent},
    maths::Vector3f,
    resources::Loadable,
    transform::Transform,
};
use floating_duration::TimeAsFloat;
use game::PiecesManager;
use std::{error, time::SystemTime};

mod config;
mod game;

// Main
fn main() -> Result<(), Box<error::Error>> {
    // Initialize time
    let start_time = SystemTime::now();
    let mut last_time = start_time;

    // Load configuration from file
    let conf = config::Config::load_from_file("res/config.ron", ())?;

    // Initialize SDL
    let sdl = cuivre::init_sdl()?;

    // Initialize input
    let mut input_manager = InputManager::new();

    // Initialize graphics
    let mut graphics_manager = GraphicsManager::new(
        &sdl,
        WindowSettings {
            title: "RustChess",
            width: conf.video.width,
            height: conf.video.height,
            vsync: conf.video.vsync,
        },
    )?;

    // Get default texture options
    let texture_options = TextureOptions::default();

    // Load tiles texture
    let tiles_texture = Texture::load_from_file("res/sprites/tiles.png", texture_options)?;
    let tiles_sheet = SpriteSheet::new(tiles_texture, 16, 16);

    // Load pieces texture
    let pieces_texture = Texture::load_from_file("res/sprites/pieces.png", texture_options)?;
    let pieces_sheet = SpriteSheet::new(pieces_texture, 16, 16);

    // Load font
    let mut roboto = Font::load_from_file("res/fonts/Roboto.ttf", ())?;
    let text_transform = Transform {
        position: Vector3f::new(0.0, 8.0, 1.0),
        scale: Vector3f::new(1.0, 1.0, 1.0),
        rotation: Vector3f::new(0.0, 0.0, 0.0),
    };
    let text_settings = TextSettings {
        scale: 100.0,
        color: (255, 0, 255),
        line_width: 800,
    };

    // Create camera
    let mut camera = Camera {
        position: Vector3f::new(4.0, 4.0, 10.0),
        direction: Vector3f::new(0.0, 0.0, -1.0),
        near: 0.1,
        far: 100.0,
        size: 9.0,
        scale_mode: CameraScaleMode::Min,
        perspective: false,
    };

    // Create pieces manager
    let pieces_manager = PiecesManager::new();

    println!(
        "Startup took {:?}",
        SystemTime::now().duration_since(start_time)?
    );

    // Main game loop
    'main: loop {
        // Current time
        let now = SystemTime::now();

        // Total elapsed time
        let _elapsed_seconds = now.duration_since(start_time)?.as_fractional_secs() as f32;

        // Delta time
        let _delta_time = now.duration_since(last_time)?.as_fractional_secs() as f32;
        last_time = now;

        // Handle events
        for event in input_manager.update(sdl.event_pump()?) {
            match event {
                Event::Quit { .. } => break 'main,
                Event::Window { win_event, .. } => {
                    if let WindowEvent::SizeChanged(width, height) = win_event {
                        graphics_manager.resize(width, height);
                    }
                }
                _ => {}
            }
        }

        if input_manager.key(Keycode::Escape).pressed() {
            break 'main;
        }

        // Zoom
        camera.size -= input_manager.mouse_wheel() as f32;

        // Draw board
        for x in 0..8 {
            for y in 0..8 {
                let mut tile_transform =
                    Transform::from_position(Vector3f::new(x as f32 + 0.5, y as f32 + 0.5, 0.0));

                let tile_sprite = tiles_sheet.sprite(0, x + y + 1);
                graphics_manager.draw_sprite(&tile_sprite, &tile_transform, &camera);
            }
        }

        // Draw pieces
        for piece in &pieces_manager.pieces {
            graphics_manager.draw_sprite(&piece.sprite(&pieces_sheet), &piece.transform(), &camera);
        }

        // Draw text
        graphics_manager.draw_text(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-=!@#$%^&*()_+",
            &mut roboto,
            text_settings,
            &text_transform,
            &camera,
        )?;

        // Render
        graphics_manager.render()?;

        // Limit fps
        //std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
