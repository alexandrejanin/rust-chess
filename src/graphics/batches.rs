use cgmath::One;
use maths::{Matrix4f, Vector2f};
use std::slice::Iter;
use super::{mesh::Mesh, shaders::Program, Texture};


//Max number of objects in a draw call
const BATCH_SIZE: usize = 150;

#[derive(Debug)]
pub struct DrawCall {
    pub program: Program,
    pub mesh: Mesh,
    pub texture: Texture,
    pub tex_position: Vector2f,
    pub tex_size: Vector2f,
    pub matrix: Matrix4f,
}

///A queued draw call to be rendered.
pub struct Batch {
    ///Shader program to use to render.
    program: Program,
    ///Mesh to be rendered.
    mesh: Mesh,
    ///Texture to be rendered.
    texture: Texture,

    ///Array of each object's upper right corner on the texture to sample (in OpenGL coordinates)
    tex_positions: [Vector2f; BATCH_SIZE],
    ///Array of each object's size on the texture to sample (in OpenGL coordinates)
    tex_sizes: [Vector2f; BATCH_SIZE],

    ///Transform matrix to apply to each object.
    matrices: [Matrix4f; BATCH_SIZE],

    ///Current amount of objects in the batch
    obj_count: usize,
}

impl Batch {
    pub fn program(&self) -> Program { self.program }
    pub fn mesh(&self) -> Mesh { self.mesh }
    pub fn texture(&self) -> Texture { self.texture }

    pub fn tex_positions(&self) -> &[Vector2f] { &self.tex_positions[..self.obj_count] }
    pub fn tex_sizes(&self) -> &[Vector2f] { &self.tex_sizes[..self.obj_count] }
    pub fn matrices(&self) -> &[Matrix4f] { &self.matrices[..self.obj_count] }

    pub fn obj_count(&self) -> usize { self.obj_count }

    ///Creates an empty batch from specified drawcall.
    pub fn new(drawcall: &DrawCall) -> Self {
        let mut batch = Self {
            program: drawcall.program,
            mesh: drawcall.mesh,
            texture: drawcall.texture,
            tex_positions: [Vector2f::new(0.0, 0.0); BATCH_SIZE],
            tex_sizes: [Vector2f::new(1.0, 1.0); BATCH_SIZE],
            matrices: [Matrix4f::one(); BATCH_SIZE],
            obj_count: 0,
        };

        batch.add(drawcall);

        batch
    }

    ///Adds an object to the batch. Returns false if the batch is full.
    pub fn add(&mut self, drawcall: &DrawCall) -> bool {
        //Check if batch is full
        if self.obj_count >= BATCH_SIZE {
            return false;
        }

        //Add object to batch
        self.tex_positions[self.obj_count] = drawcall.tex_position;
        self.tex_sizes[self.obj_count] = drawcall.tex_size;
        self.matrices[self.obj_count] = drawcall.matrix;

        self.obj_count += 1;

        true
    }
}

///Contains and manages batches to be drawn.
pub struct BatchList {
    batches: Vec<Batch>,
}

impl BatchList {
    ///Initializes and empty BatchList.
    pub fn new() -> Self {
        Self {
            batches: Vec::new()
        }
    }

    ///Initializes and empty BatchList with specified capacity before needing to be resized.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            batches: Vec::with_capacity(capacity)
        }
    }


    pub fn insert(&mut self, drawcall: &DrawCall) {
        for batch in &mut self.batches {
            if batch.program == drawcall.program && batch.mesh == drawcall.mesh && batch.texture == drawcall.texture {
                //Attempts to add drawcall to batch
                if batch.add(drawcall) {
                    return
                }
            }
        }

        //Could not find suitable batch, create a new one
        self.batches.push(Batch::new(drawcall));
    }

    pub fn clear(&mut self) {
        self.batches.clear()
    }

    pub fn len(&self) -> usize {
        self.batches.len()
    }

    pub fn iter(&self) -> Iter<Batch> {
        self.batches.iter()
    }
}
