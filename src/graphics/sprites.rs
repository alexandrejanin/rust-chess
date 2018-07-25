use super::{
    Texture,
    TextureID,
    Vector2f,
    Vector2u
};

///Represents an OpenGL texture sliced into sprites.
#[derive(Copy, Clone, Debug)]
pub struct SpriteSheet {
    texture: Texture,
    sprite_size: Vector2u,
    gl_size: Vector2f,
}

impl SpriteSheet {
    ///Create a new sprite sheet from a texture and a sprite size
    pub fn new(texture: Texture, sprite_size: Vector2u) -> SpriteSheet {
        SpriteSheet {
            texture,
            sprite_size,
            gl_size: Vector2f::new(
                sprite_size.x as f32 / texture.width() as f32,
                sprite_size.y as f32 / texture.height() as f32,
            )
        }
    }

    pub fn texture_id(&self) -> TextureID { self.texture.id() }

    pub fn sprite_size(&self) -> Vector2u { self.sprite_size }

    pub fn sprite_width(&self) -> u32 { self.sprite_size.x }

    pub fn sprite_height(&self) -> u32 { self.sprite_size.y }

    pub fn gl_size(&self) -> Vector2f { self.gl_size }

    pub fn gl_position(&self, position: Vector2u) -> Vector2f {
        Vector2f::new(
            (self.sprite_width() * position.x) as f32 / self.texture.width() as f32,
            (self.sprite_height() * position.y) as f32 / self.texture.height() as f32,
        )
    }
}

///Represents part of a sprite sheet drawn on a quad.
#[derive(Copy, Clone, Debug)]
pub struct Sprite {
    sheet: SpriteSheet,
    sheet_position: Vector2u,
    gl_position: Vector2f,
}

impl Sprite {
    ///Create a new sprite from a sprite sheet and a position
    pub fn new(sheet: SpriteSheet, position: Vector2u) -> Sprite {
        Sprite {
            sheet,
            sheet_position: position,
            gl_position: sheet.gl_position(position),
        }
    }

    pub fn texture_id(&self) -> TextureID { self.sheet.texture_id() }

    pub fn gl_position(&self) -> Vector2f { self.gl_position }

    pub fn gl_size(&self) -> Vector2f { self.sheet.gl_size() }

    //pub fn pixel_position(&self) -> Vector2u { self.pixel_position }

    pub fn set_position(&mut self, position: Vector2u) {
        self.sheet_position = position;
        self.gl_position = self.sheet.gl_position(position);
    }
}