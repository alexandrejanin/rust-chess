use config;
use gl;
use maths::{Vector2f, Vector2u, Vector3f};
use maths::transform::Transform;
use resources::{self, ResourceLoader};
use sdl2;
use std::{
    self,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
};
use super::{
    mesh::{Mesh, MeshBuilder, Vertex}, shaders::{self, Program}, sprites::Sprite, Texture,
};

///Error related to OpenGL drawing.
#[derive(Debug)]
pub enum DrawingError {
    ResourceError(resources::ResourceError),
    ///Error related to OpenGL shaders.
    ShaderError(shaders::ShaderError),
    ///Tried drawing a mesh that had no EBO set.
    MeshEBONotInitialized,
    ///Tried drawing a mesh that had no EBO set.
    MeshVAONotInitialized,
}

impl From<resources::ResourceError> for DrawingError {
    fn from(error: resources::ResourceError) -> Self {
        DrawingError::ResourceError(error)
    }
}

impl From<shaders::ShaderError> for DrawingError {
    fn from(error: shaders::ShaderError) -> Self {
        DrawingError::ShaderError(error)
    }
}

impl Display for DrawingError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Drawing failed: ")?;
        match self {
            DrawingError::ResourceError(error) => write!(f, "{}", error),
            DrawingError::ShaderError(error) => write!(f, "{}", error),
            DrawingError::MeshEBONotInitialized => write!(f, "Mesh EBO not initialized"),
            DrawingError::MeshVAONotInitialized => write!(f, "Mesh VAO not initialized"),
        }
    }
}


///Manages everything related to graphics and rendering.
pub struct GraphicsManager<'a> {
    resource_loader: &'a ResourceLoader,
    conf: &'a config::Config,
    sdl: &'a sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
    program: Program,
    quad: Mesh,
    textures: HashMap<PathBuf, Texture>,
}


impl<'a> GraphicsManager<'a> {
    ///Initializes graphics from SDL and Config object
    pub fn new(resource_loader: &'a ResourceLoader, conf: &'a config::Config, sdl: &'a sdl2::Sdl) -> Result<GraphicsManager<'a>, DrawingError> {
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

        unsafe {
            //Depth testing
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            //Blending
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            //Clear color
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        //Load shaders
        let program = Program::load_shaders(resource_loader, Path::new("shaders/triangle.vert"), Path::new("shaders/triangle.frag"))?;

        //Build quad mesh
        let mesh_builder = MeshBuilder {
            vertices: vec![
                Vertex { position: Vector3f::new(0.5, 0.5, 0.0), uv: Vector2f::new(1.0, 0.0) },  //Top right,
                Vertex { position: Vector3f::new(0.5, -0.5, 0.0), uv: Vector2f::new(1.0, 1.0) },  //Bottom right
                Vertex { position: Vector3f::new(-0.5, -0.5, 0.0), uv: Vector2f::new(0.0, 1.0) },  //Bottom left
                Vertex { position: Vector3f::new(-0.5, 0.5, 0.0), uv: Vector2f::new(0.0, 0.0) },  //Top left,
            ],

            indices: vec![
                0, 1, 2,
                0, 2, 3,
            ],
        };

        let quad = mesh_builder.build();

        //Build and return graphics manager
        Ok(GraphicsManager {
            resource_loader,
            conf,
            sdl,
            video,
            window,
            gl_context,
            program,
            quad,
            textures: HashMap::new(),
        })
    }

    ///Gets texture for given image, loading if it wasn't loaded yet.
    pub fn get_texture(&mut self, path: &Path) -> Result<Texture, DrawingError> {
        //Return texture id if it's loaded already
        if let Some(texture) = self.textures.get(path) {
            return Ok(*texture)
        };

        //Texture wasn't found, load it
        let image = self.resource_loader.load_png(path)?;

        //Get image size
        let (width, height) = image.dimensions();

        //Allocate texture
        let mut texture_id = 0;

        unsafe {
            //Create texture
            gl::GenTextures(1, &mut texture_id);

            //Bind texture
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

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

            //Texture wrapping
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as gl::types::GLint);

            //Texture filtering
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as gl::types::GLint);

            //Generate mipmaps
            gl::GenerateMipmap(gl::TEXTURE_2D);

            //Unbind texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        let texture = Texture {
            id: texture_id,
            size: Vector2u::new(width, height)
        };

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
    pub fn draw_sprite(&mut self, sprite: Sprite, transform: Transform) -> Result<(), DrawingError> {
        let mesh = self.quad;
        self.draw_mesh(mesh, sprite, transform)
    }


    ///Draw textured mesh
    pub fn draw_mesh(&mut self, mesh: Mesh, sprite: Sprite, transform: Transform) -> Result<(), DrawingError> {
        //Check that mesh is valid
        mesh.check()?;

        //Use program
        self.program.set_used();

        //Set sprite coordinates
        self.program.set_vec2("SourcePosition", sprite.gl_position());
        self.program.set_vec2("SourceSize", sprite.gl_size());

        //Set transform
        self.program.set_mat4("transform", &transform.matrix());

        unsafe {
            //Bind texture
            gl::BindTexture(gl::TEXTURE_2D, sprite.texture_id());

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
