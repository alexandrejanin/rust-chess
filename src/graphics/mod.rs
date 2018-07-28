use gl;
use maths::Vector2u;

pub mod manager;
pub mod sprites;
mod mesh;
mod shaders;

///ID of loaded OpenGL Texture
pub type TextureID = gl::types::GLuint;

///Represents a texture loaded in OpenGL.
#[derive(Copy, Clone, Debug)]
pub struct Texture {
    id: TextureID,
    size: Vector2u,
}

impl Texture {
    pub fn id(&self) -> gl::types::GLuint { self.id }

    pub fn size(&self) -> Vector2u { self.size }

    pub fn width(&self) -> u32 { self.size.x }

    pub fn height(&self) -> u32 { self.size.y }
}
