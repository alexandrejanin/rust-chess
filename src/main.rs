extern crate cuivre;
extern crate floating_duration;
extern crate rand;
extern crate ron;
#[macro_use]
extern crate serde;

use cuivre::{
    graphics::{
        camera::{Camera, CameraScaleMode},
        sprites::SpriteSheet,
        textures::{Texture, TextureOptions},
        GraphicsManager, WindowSettings,
    },
    input::{Event, InputManager, Keycode, MouseButton, WindowEvent},
    maths::Vector3f,
    resources::Loadable,
    transform::Transform,
};
use floating_duration::TimeAsFloat;
use game::PiecesManager;
use std::{cmp, error, time::SystemTime};

mod config;
mod game;

// Main
fn main() -> Result<(), Box<error::Error>> {
    // Initialize time
    let start_time = SystemTime::now();
    let mut last_time = start_time;

    // Load configuration from file
    let conf = config::Config::load_from_file("res/config.ron", ())??;

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
    let tiles_texture = Texture::load_from_file("res/sprites/tiles.png", texture_options)??;
    let tiles_sheet = SpriteSheet::new(tiles_texture, 16, 16);

    // Load pieces texture
    let pieces_texture = Texture::load_from_file("res/sprites/pieces.png", texture_options)??;
    let pieces_sheet = SpriteSheet::new(pieces_texture, 16, 16);

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
    let mut pieces_manager = PiecesManager::new();

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

        // Click to move pieces
        if input_manager.button(MouseButton::Left).pressed() {
            let pixels_per_unit = (cmp::min(
                graphics_manager.window_size().x,
                graphics_manager.window_size().y,
            ) as f32 / camera.size) as i32;

            // Find out which tile was clicked
            let window_size = graphics_manager.window_size();
            let mouse_position = input_manager.mouse_position();
            let tile_x = (4 * pixels_per_unit + mouse_position.x - window_size.x as i32 / 2)
                / pixels_per_unit;
            let tile_y = (4 * pixels_per_unit - mouse_position.y + window_size.y as i32 / 2)
                / pixels_per_unit;

            // Select piece if valid
            pieces_manager.on_click(tile_x as usize, tile_y as usize)
        }

        // Draw board
        for x in 0..8 {
            for y in 0..8 {
                let mut tile_transform =
                    Transform::from_position(Vector3f::new(x as f32 + 0.5, y as f32 + 0.5, 0.0));

                let tile_sprite = tiles_sheet.sprite(0, (x + y + 1) as i32);
                graphics_manager.draw_sprite(&tile_sprite, &tile_transform, &camera);
            }
        }

        // Draw possible moves
        for mov in pieces_manager.selected_moves() {
            let (x, y) = mov.target_pos();

            let mut tile_transform =
                Transform::from_position(Vector3f::new(x as f32 + 0.5, y as f32 + 0.5, 0.0));

            let tile_sprite = tiles_sheet.sprite(1, (x + y + 1) as i32);
            graphics_manager.draw_sprite(&tile_sprite, &tile_transform, &camera);
        }

        // Draw pieces
        for piece in pieces_manager.pieces() {
            graphics_manager.draw_sprite(&piece.sprite(&pieces_sheet), &piece.transform(), &camera)
        }

        // Render
        graphics_manager.render()?;

        // Limit fps
        //std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
