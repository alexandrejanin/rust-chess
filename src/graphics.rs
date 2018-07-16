extern crate sdl2;

use sdl2::{Sdl, VideoSubsystem};
use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use std::path::PathBuf;

use config::DisplayConfig;

pub struct DisplayManager<'a> {
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    textures: Vec<Texture<'a>>
}

impl<'a> DisplayManager<'a> {
    pub fn new(name: &str, sdl_context: &Sdl, display_config: &DisplayConfig) -> DisplayManager<'a> {
        //Create context and init video
        let video_subsystem = sdl_context.video().unwrap();
        let image_context = sdl2::image::init(INIT_PNG).unwrap();

        //Create window
        let window = video_subsystem
            .window(name, display_config.width, display_config.height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        DisplayManager {
            video_subsystem: video_subsystem,
            canvas: canvas,
            texture_creator: texture_creator,
            textures: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn render(&mut self) {
        self.canvas.present();
    }

    //TODO: Queues a draw order draws the sprite at the specified position
    pub fn draw(&mut self, sprite: &Sprite, position: Point) {
        self.canvas.copy(
            &self.textures[sprite.texture_index],
            sprite.rect,
            Rect::new(position.x, position.y, sprite.rect.width(), sprite.rect.height())
        );
    }
}

///Represents a sprite: rectangle on a png image.
#[derive(Debug)]
struct Sprite {
    ///Index of texture in DisplayManager
    texture_index: usize,
    ///Position and size on the image (in pixels)
    rect: Rect
}

