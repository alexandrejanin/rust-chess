use gl;
use sdl2;
use std;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use config;
use resources::ResourceLoader;
use graphics::Error;
use graphics::shaders::Program;
use graphics::data::{Mesh, MeshBuilder, Vector2, Vector3, Vertex};

type TextureID = gl::types::GLuint;

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
    pub fn init(&mut self, resources_loader: &ResourceLoader) -> Result<(), Error> {
        //Enable depth testing
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
        }

        //Load shaders
        self.program = match Program::load_shaders(resources_loader, Path::new("shaders/triangle.vert"), Path::new("shaders/triangle.frag")) {
            Ok(program) => Some(program),
            Err(error) => return Err(error),
        };

        //Set GL clear color
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        //Build quad mesh
        let mesh_builder = MeshBuilder {
            vertices: vec![
                Vertex { position: Vector3(0.5, 0.5, 0.0), color: Vector3(0.0, 1.0, 0.0), uv: Vector2(1.0, 1.0) },  //Top right,
                Vertex { position: Vector3(0.5, -0.5, 0.0), color: Vector3(0.0, 0.0, 1.0), uv: Vector2(1.0, 0.0) },  //Bottom right
                Vertex { position: Vector3(-0.5, -0.5, 0.0), color: Vector3(0.0, 1.0, 0.0), uv: Vector2(0.0, 0.0) },  //Bottom left
                Vertex { position: Vector3(-0.5, 0.5, 0.0), color: Vector3(0.0, 0.0, 1.0), uv: Vector2(0.0, 1.0) },  //Top left,
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
    pub fn get_texture(&mut self, resource_loader: &ResourceLoader, path: &Path) -> Result<TextureID, Error> {
        //Find texture if it's loaded already
        match self.textures.get(path) {
            Some(texture_id) => return Ok(*texture_id),
            None => {},
        };

        //Texture wasn't found, load it
        self.load_texture(resource_loader, path)
    }


    ///Loads texture from file in "res". Returns OpenGL texture id.
    fn load_texture(&mut self, resource_loader: &ResourceLoader, path: &Path) -> Result<TextureID, Error> {
        //Load image
        let image = resource_loader.load_png(path)?;

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
    pub fn draw_sprite(&self, texture: TextureID) -> Result<(), String> {
        match self.quad {
            Some(ref quad) => self.draw_mesh(quad, texture),
            None => Err(String::from("ERROR: Quad mesh not initialized")),
        }
    }


    ///Draw textured quad
    pub fn draw_mesh(&self, mesh: &Mesh, texture: TextureID) -> Result<(), String> {
        //Bind texture
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture);
        }

        //Check and use program
        match self.program {
            Some(ref p) => p.set_used(),
            None => return Err("ERROR: OpenGL Program not loaded!".to_string()),
        }

        //Check and bind and draw vertices
        match mesh.ebo() {
            //EBO == 0: not initialized
            0 => return Err("ERROR: Quad EBO not initialized!".to_string()),
            //EBO != 0: ok
            _ => unsafe {
                //Bind mesh
                gl::BindVertexArray(mesh.vao());
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo());

                gl::DrawElements(
                    gl::TRIANGLES, //Draw mode
                    mesh.indices_count() as i32, //Number of indices
                    gl::UNSIGNED_INT,
                    0 as *const gl::types::GLvoid, //Starting index
                );
            },
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
