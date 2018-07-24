use cgmath::{Matrix4, Vector2, Vector3};
use gl;

pub mod manager;
mod mesh;
mod shaders;

//Define useful types
pub type Vector2f = Vector2<f32>;
pub type Vector3f = Vector3<f32>;
pub type Matrix4f = Matrix4<f32>;

pub type TextureID = gl::types::GLuint;