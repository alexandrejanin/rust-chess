use maths::Vector2f;
use std::{
    cmp::Ordering,
    slice::Iter
};
use super::{mesh::Mesh, shaders::Program, Texture};
use transform::Transform;


///A queued draw call to be rendered.
#[derive(Debug)]
pub struct DrawCall {
    ///Shader program to use to render.
    pub program: Program,
    ///Mesh to be rendered.
    pub mesh: Mesh,
    ///Texture to be rendered.
    pub texture: Texture,
    ///Upper right corner of the texture to sample (in OpenGL coordinates)
    pub tex_position: Vector2f,
    ///Size of the texture to sample (in OpenGL coordinates)
    pub tex_size: Vector2f,
    ///Transform to apply to the mesh.
    pub transform: Transform,
}

impl PartialEq for DrawCall {
    fn eq(&self, other: &Self) -> bool {
        self.program == other.program &&
            self.texture == other.texture &&
            self.mesh == other.mesh &&
            self.tex_position == other.tex_position &&
            self.tex_size == other.tex_size &&
            self.transform == other.transform
    }
}

impl Eq for DrawCall {}

impl PartialOrd for DrawCall {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DrawCall {
    fn cmp(&self, other: &Self) -> Ordering {
        //Sort by shader program, then texture, then mesh
        if self.program != other.program {
            return self.program.cmp(&other.program);
        } else {
            if self.texture != other.texture {
                return self.texture.cmp(&other.texture);
            } else {
                return self.mesh.cmp(&other.mesh);
            }
        }
    }
}

///Contains sorted DrawCalls.
pub struct DrawCallQueue {
    drawcalls: Vec<DrawCall>,
}

impl DrawCallQueue {
    pub fn new() -> Self {
        Self {
            drawcalls: Vec::new()
        }
    }

    pub fn add(&mut self, drawcall: DrawCall) {
        let index = match self.drawcalls.binary_search(&drawcall) {
            Ok(found_index) => found_index,
            Err(new_index) => new_index,
        };

        self.drawcalls.insert(index, drawcall);
    }

    pub fn clear(&mut self) {
        self.drawcalls.clear()
    }

    pub fn iter(&self) -> Iter<DrawCall> {
        self.drawcalls.iter()
    }
}
