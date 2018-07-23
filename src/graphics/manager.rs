use gl;
use cgmath::One;
use sdl2;
use std;
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use config;
use resources::{self, ResourceLoader};
use super::{
    data::{Mesh, MeshBuilder, Vertex, Matrix4f, Vector2f, Vector3f},
    shaders::{self, Program},
};

type TextureID = gl::types::GLuint;


///Error related to OpenGL drawing.
pub enum DrawingError {
    ResourceError(resources::ResourceError),
    ///Error related to OpenGL shaders.
    ShaderError(shaders::ShaderError),
    ///Default quad mesh not initialized.
    QuadMeshNotInitialized,
    ///Tried drawing a mesh that had no EBO set.
    MeshEBONotInitialized,
    ///Tried drawing a mesh that had no EBO set.
    MeshVAONotInitialized,
    ///No OpenGL program loaded.
    ProgramNotInitialized,
}

impl Display for DrawingError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Drawing failed: ")?;
        match self {
            DrawingError::ResourceError(error) => write!(f, "{}", error),
            DrawingError::ShaderError(error) => write!(f, "{}", error),
            DrawingError::QuadMeshNotInitialized => write!(f, "Quad mesh not initialized"),
            DrawingError::MeshEBONotInitialized => write!(f, "Mesh EBO not initialized"),
            DrawingError::MeshVAONotInitialized => write!(f, "Mesh VAO not initialized"),
            DrawingError::ProgramNotInitialized => write!(f, "Program not initialized"),
        }
    }
}


///Manages everything related to graphics and rendering.
pub struct GraphicsManager<'a> {
    conf: &'a config::Config,
    sdl: &'a sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
    program: Option<Program>,
    textures: HashMap<PathBuf, TextureID>,
    quad: Option<Mesh>,
}


impl<'a> GraphicsManager<'a> {
    ///Initializes graphics from SDL and Config object
    pub fn new(conf: &'a config::Config, sdl: &'a sdl2::Sdl) -> GraphicsManager<'a> {
        //Initialize VideoSubsystem
        let video = sdl.video().unwrap();

        //Set OpenGL parameters
        {
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 3);
        }

        //Create Window
        let window = video
            .window("RustChess", conf.display.width, conf.display.height)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        //Initialize OpenGL
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

        //Enable/disable vsync
        video.gl_set_swap_interval(match conf.display.vsync {
            true => sdl2::video::SwapInterval::VSync,
            false => sdl2::video::SwapInterval::Immediate,
        });

        //Build and return GraphicsManager
        GraphicsManager {
            conf,
            sdl,
            video,
            window,
            gl_context,
            program: None,
            textures: HashMap::new(),
            quad: None,
        }
    }


    ///Initializes graphics. Must be ran once before any rendering is done.
    pub fn init(&mut self, resources_loader: &ResourceLoader) -> Result<(), DrawingError> {
        //Enable depth testing
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
        }

        //Load shaders
        self.program = match Program::load_shaders(resources_loader, Path::new("shaders/triangle.vert"), Path::new("shaders/triangle.frag")) {
            Ok(program) => Some(program),
            Err(error) => return Err(DrawingError::ShaderError(error)),
        };

        //Set GL clear color
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        //Build quad mesh
        let mesh_builder = MeshBuilder {
            vertices: vec![
                Vertex { position: Vector3f::new(0.5, 0.5, 0.0), uv: Vector2f::new(1.0, 1.0) },  //Top right,
                Vertex { position: Vector3f::new(0.5, -0.5, 0.0), uv: Vector2f::new(1.0, 0.0) },  //Bottom right
                Vertex { position: Vector3f::new(-0.5, -0.5, 0.0), uv: Vector2f::new(0.0, 0.0) },  //Bottom left
                Vertex { position: Vector3f::new(-0.5, 0.5, 0.0), uv: Vector2f::new(0.0, 1.0) },  //Top left,
            ],

            indices: vec![
                0, 1, 2,
                0, 2, 3,
            ],
        };

        self.quad = Some(mesh_builder.build());

        //Texture parameters
        unsafe {
            //Texture wrapping
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as gl::types::GLint);

            //Texture filtering
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as gl::types::GLint);
        }

        Ok(())
    }


    ///Gets texture id for given path, loading if it wasn't loaded yet.
    pub fn get_texture(&mut self, resource_loader: &ResourceLoader, path: &Path) -> Result<TextureID, DrawingError> {
        //Find texture if it's loaded already
        match self.textures.get(path) {
            Some(texture_id) => return Ok(*texture_id),
            None => {},
        };

        //Texture wasn't found, load it
        self.load_texture(resource_loader, path)
    }


    ///Loads texture from file in "res". Returns OpenGL texture id.
    fn load_texture(&mut self, resource_loader: &ResourceLoader, path: &Path) -> Result<TextureID, DrawingError> {
        //Load image
        let image = resource_loader.load_png(path).map_err(|error| DrawingError::ResourceError(error))?;

        //Get image size
        let (width, height) = image.dimensions();

        //Allocate texture
        let mut texture: TextureID = 0;

        unsafe {
            gl::GenTextures(1, &mut texture);

            //Bind texture
            gl::BindTexture(gl::TEXTURE_2D, texture);

            //Fill texture
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as gl::types::GLint,
                width as gl::types::GLint,
                height as gl::types::GLint,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.as_ptr() as *const std::os::raw::c_void,
            );

            //Generate mipmaps
            gl::GenerateMipmap(gl::TEXTURE_2D);

            //Unbind texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        //Save texture so we don't have to load it again
        self.textures.insert(path.into(), texture);

        Ok(texture)
    }


    ///Call when the window is resized
    pub fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(
                0,
                0,
                width as gl::types::GLint,
                height as gl::types::GLint,
            );
        }
    }


    ///Clears the frame for drawing
    pub fn clear(&self) {
        //Set and clear view
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }


    ///Renders the current frame
    pub fn render(&self) {
        //Swap buffers
        self.window.gl_swap_window();
    }


    ///Draw textured quad
    pub fn draw_sprite(&mut self, texture: TextureID) -> Result<(), DrawingError> {
        match self.quad {
            None => Err(DrawingError::QuadMeshNotInitialized),
            Some(quad) => self.draw_mesh(quad, texture),
        }
    }


    ///Draw textured mesh
    pub fn draw_mesh(&mut self, mesh: Mesh, texture: TextureID) -> Result<(), DrawingError> {
        //Check that mesh is valid
        if mesh.ebo() == 0 { return Err(DrawingError::MeshEBONotInitialized) }
        if mesh.vao() == 0 { return Err(DrawingError::MeshVAONotInitialized) }

        //Check program
        let program = match self.program {
            Some(ref mut p) => p,
            None => return Err(DrawingError::ProgramNotInitialized),
        };

        //Use program
        program.set_used();

        //Set transform matrix
        let transform = Matrix4f::one();
        program.set_mat4("transform", &transform);

        unsafe {
            //Bind texture
            gl::BindTexture(gl::TEXTURE_2D, texture);

            //Bind mesh
            gl::BindVertexArray(mesh.vao());
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo());

            gl::DrawElements(
                gl::TRIANGLES, //Draw mode
                mesh.indices_count() as i32, //Number of indices
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid, //Starting index
            );
        }

        //Unbind everything
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        Ok(())
    }
}
