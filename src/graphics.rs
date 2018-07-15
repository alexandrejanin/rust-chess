extern crate sdl2;

use std::path::PathBuf;
use sdl2::rect::Rect;
use std::option::Option;

pub struct Sprite {
    pub path: PathBuf,
    pub src_rect: Option<Rect>,
    pub dst_rect: Rect
}

impl Sprite {
    pub fn new(img_path: &str) -> Sprite {
        Sprite {
            path: PathBuf::from(img_path),
            src_rect: None,
            dst_rect: Rect::new(0, 0, 200, 200) 
        }
    }
}

