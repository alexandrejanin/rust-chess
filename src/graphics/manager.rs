use gl;
use sdl2;
use std;
use std::path::Path;

use config;
use graphics::Error;
use graphics::shaders::Program;
use resources::ResourceLoader;

///Manages everything related to graphics and rendering.
pub struct GraphicsManager<'a> {
    conf: &'a config::Config,
    sdl: &'a sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
    program: Option<Program>,
    vao: gl::types::GLuint,
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
            .build()
            .unwrap();

        //Initialize OpenGL
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

        //Build and return GraphicsManager
        GraphicsManager {
            conf,
            sdl,
            video,
            window,
            gl_context,
            program: None,
            vao: 0,
        }
    }

    ///Initializes graphics. Must be ran once before any rendering is done.
    pub fn init(&mut self, resources_loader: &ResourceLoader) -> Result<(), Error> {
        //Load shaders
        self.program = match Program::load_shaders(resources_loader, Path::new("res/shaders/triangle.vert"), Path::new("res/shaders/triangle.frag")) {
            Ok(program) => Some(program),
            Err(error) => return Err(error),
        };

        //Set GL clear color
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        //Triangle vertices
        let vertices: Vec<f32> = vec![
            //Position       //Colors
            -0.5,-0.5, 0.0,  1.0, 0.0, 0.0,  //Bottom right
            0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  //Bottom left
            0.0,  0.5, 0.0,  0.0, 0.0, 1.0   //Top
        ];

        //Create VBO
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, //Data length
                vertices.as_ptr() as *const gl::types::GLvoid, //Data location
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        unsafe {
            //Create and bind VAO
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            //Vertex location 0: Position
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0, //Location
                3, //Number of components per vertex
                gl::FLOAT,
                gl::FALSE, //Normalize
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint, //Stride
                std::ptr::null() //Offset
            );

            //Vertex location 1: Color
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1, //Location
                3, //Number of components per vertex
                gl::FLOAT,
                gl::FALSE, //Normalize
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint, //Stride
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid  //Offset
            );

            //Unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Ok(())
    }

    ///Renders the current frame
    pub fn render(&self) -> Result<(), String> {
        //Set and clear view
        unsafe {
            gl::Viewport(
                0,
                0,
                self.conf.display.width as i32,
                self.conf.display.height as i32,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        //Check and use program
        match self.program {
            None => return Err("ERROR: OpenGL Program not loaded!".to_string()),
            Some(ref p) => p.set_used(),
        }

        //Check and bind and draw vertices
        match self.vao {
            //VAO = 0: not initialized
            0 => return Err("ERROR: VAO not initialized!".to_string()),
            //VAO != 0: ok
            _ => unsafe {
                gl::BindVertexArray(self.vao);
                gl::DrawArrays(
                    gl::TRIANGLES, //Draw mode
                    0, //Starting index
                    3, //Number of vertices
                );
            },
        }

        //Swap buffers
        self.window.gl_swap_window();

        Ok(())
    }
}
