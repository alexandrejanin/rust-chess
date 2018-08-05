use super::manager::DrawingError;
use gl;
use maths::{Vector2f, Vector3f};
use std;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: Vector3f,
    pub uv: Vector2f,
}

impl Vertex {
    pub fn attrib_arrays() {
        let stride = std::mem::size_of::<Self>();

        unsafe {
            //Position
            Vertex::attrib_array(stride, 0, 0, 3);
            //Color
            Vertex::attrib_array(stride, 1, std::mem::size_of::<Vector3f>(), 2);
        }
    }

    unsafe fn attrib_array(stride: usize, location: gl::types::GLuint, offset: usize, length: i32) {
        //Vertex location 0: Position
        gl::EnableVertexAttribArray(location);
        gl::VertexAttribPointer(
            location, //Location
            length,   //Number of components per vertex
            gl::FLOAT,
            gl::FALSE,                          //Normalize
            stride as gl::types::GLint,         //Stride
            offset as *const gl::types::GLvoid, //Offset
        );
    }
}

///Represents a mesh that is loaded in OpenGL.
#[derive(Copy, Clone, Debug)]
pub struct Mesh {
    vbo: gl::types::GLuint,
    vao: gl::types::GLuint,
    ebo: gl::types::GLuint,
    vertex_count: usize,
    indices_count: usize,
}

impl Mesh {
    pub fn vbo(&self) -> gl::types::GLuint {
        self.vbo
    }
    pub fn vao(&self) -> gl::types::GLuint {
        self.vao
    }
    pub fn ebo(&self) -> gl::types::GLuint {
        self.ebo
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }
    pub fn indices_count(&self) -> usize {
        self.indices_count
    }

    pub fn check(&self) -> Result<(), DrawingError> {
        if self.ebo == 0 {
            return Err(DrawingError::MeshEBONotInitialized);
        }
        if self.vao == 0 {
            return Err(DrawingError::MeshVAONotInitialized);
        }

        Ok(())
    }
}

impl PartialEq for Mesh {
    fn eq(&self, other: &Self) -> bool {
        self.ebo == other.ebo && self.vao == other.vao
    }
}

impl Eq for Mesh {}

impl PartialOrd for Mesh {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Mesh {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //Sort by EBO, then VAO
        if self.ebo != other.ebo {
            return self.ebo.cmp(&other.ebo);
        } else {
            return self.vao.cmp(&other.vao);
        }
    }
}

#[derive(Debug)]
pub struct MeshBuilder {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<gl::types::GLuint>,
}

impl MeshBuilder {
    ///Initializes an empty MeshBuilder.
    pub fn new() -> MeshBuilder {
        MeshBuilder {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    ///Builds a mesh from current vertices and indices.
    pub fn build(&self) -> Mesh {
        let mut vao: gl::types::GLuint = 0;
        let mut vbo: gl::types::GLuint = 0;
        let mut ebo: gl::types::GLuint = 0;

        unsafe {
            //Create objects
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            //Bind VAO
            gl::BindVertexArray(vao);

            //Bind and buffer VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr, //Data length
                self.vertices.as_ptr() as *const gl::types::GLvoid, //Data location
                gl::STATIC_DRAW,
            );

            //Bind and buffer EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<gl::types::GLuint>())
                    as gl::types::GLsizeiptr, //Data length
                self.indices.as_ptr() as *const gl::types::GLvoid, //Data location
                gl::STATIC_DRAW,
            );
        }

        //Fill VAO
        Vertex::attrib_arrays();

        //Unbind everything
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        //Create mesh
        Mesh {
            vbo,
            vao,
            ebo,
            vertex_count: self.vertices.len(),
            indices_count: self.indices.len(),
        }
    }
}
