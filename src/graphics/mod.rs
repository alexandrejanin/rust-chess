use cgmath::{Matrix4, Vector2, Vector3};
use gl;

pub mod manager;
pub mod sprites;
mod mesh;
mod shaders;

///Float Vector2
pub type Vector2f = Vector2<f32>;
///Float Vector3
pub type Vector3f = Vector3<f32>;
///Float Matrix4
pub type Matrix4f = Matrix4<f32>;

///Int Vector2
pub type Vector2i = Vector2<i32>;
///Unsigned int Vector2
pub type Vector2u = Vector2<u32>;

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