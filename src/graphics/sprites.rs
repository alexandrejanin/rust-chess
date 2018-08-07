use super::Texture;
use gl;
use maths::{Vector2f, Vector2i, Vector2u, Vector4f};
use std::mem::size_of;

//Max amount of instances in a batch
pub const MAX_INSTANCES: usize = 1000;

//Size of 1 object in the VBO, in floats. 16: matrix, 4: tex coordinates
pub const DATA_LENGTH: usize = 20;

///Represents an OpenGL texture sliced into sprites.
#[derive(Copy, Clone, Debug)]
pub struct SpriteSheet {
    texture: Texture,
    sprite_size: Vector2u,
    gl_size: Vector2f,

    ///VBO used for this spritesheet's batch.
    vbo: gl::types::GLuint,
}

impl SpriteSheet {
    ///Create a new sprite sheet from a mesh (quad), texture and sprite size (in pixels)
    pub fn new(vao: gl::types::GLuint, texture: Texture, sprite_size: Vector2u) -> SpriteSheet {
        let vbo = Self::empty_vbo(MAX_INSTANCES);
        Self::add_instanced_attribute(vao, vbo, 2, 4, DATA_LENGTH as i32, 0); //texture coordinates
        Self::add_instanced_attribute(vao, vbo, 3, 4, DATA_LENGTH as i32, 4); //1st column
        Self::add_instanced_attribute(vao, vbo, 4, 4, DATA_LENGTH as i32, 8); //2nd column
        Self::add_instanced_attribute(vao, vbo, 5, 4, DATA_LENGTH as i32, 12); //3rd column
        Self::add_instanced_attribute(vao, vbo, 6, 4, DATA_LENGTH as i32, 16); //4th column

        SpriteSheet {
            texture,
            sprite_size,
            gl_size: Vector2f::new(
                sprite_size.x as f32 / texture.width() as f32,
                sprite_size.y as f32 / texture.height() as f32,
            ),
            vbo,
        }
    }

    pub fn sprite_size(&self) -> Vector2u {
        self.sprite_size
    }

    pub fn sprite_width(&self) -> u32 {
        self.sprite_size.x
    }

    pub fn sprite_height(&self) -> u32 {
        self.sprite_size.y
    }

    pub fn gl_position(&self, position: Vector2i) -> Vector4f {
        Vector4f::new(
            (self.sprite_width() as i32 * position.x) as f32 / self.texture.width() as f32,
            (self.sprite_height() as i32 * position.y) as f32 / self.texture.height() as f32,
            self.gl_size.x,
            self.gl_size.y,
        )
    }

    ///Creates and returns an empty VBO than can fit 'floats' floats.
    fn empty_vbo(floats: usize) -> gl::types::GLuint {
        let mut vbo = 0;
        unsafe {
            //Create VBO
            gl::GenBuffers(1, &mut vbo);

            //Bind and allocate VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (floats * size_of::<f32>()) as gl::types::GLsizeiptr,
                0 as *const gl::types::GLvoid,
                gl::STREAM_DRAW,
            );

            //Unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        vbo
    }

    fn add_instanced_attribute(
        vao: gl::types::GLuint,
        vbo: gl::types::GLuint,
        location: gl::types::GLuint,
        size: gl::types::GLint,
        stride: gl::types::GLsizei,
        offset: usize,
    ) {
        unsafe {
            //Bind and setup location
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BindVertexArray(vao);
            gl::EnableVertexAttribArray(location);
            gl::VertexAttribPointer(
                location,
                size,
                gl::FLOAT,
                gl::FALSE,
                stride * size_of::<f32>() as i32,
                (offset * size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl::VertexAttribDivisor(location, 1);

            //Unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}

///Represents part of a sprite sheet drawn on a quad.
#[derive(Copy, Clone, Debug)]
pub struct Sprite {
    sheet: SpriteSheet,
    pub position: Vector2i,
}

impl Sprite {
    ///Create a new sprite from a sprite sheet and a position
    pub fn new(sheet: SpriteSheet, position: Vector2i) -> Sprite {
        Sprite { sheet, position }
    }

    pub fn texture(&self) -> Texture {
        self.sheet.texture
    }

    pub fn vbo(&self) -> gl::types::GLuint {
        self.sheet.vbo
    }

    pub fn gl_position(&self) -> Vector4f {
        self.sheet.gl_position(self.position)
    }
}
