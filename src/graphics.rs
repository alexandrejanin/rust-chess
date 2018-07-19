use config;
use gl;
use sdl2;
use std;
use std::ffi::{CStr, CString};
use std::fs;
use std::path::Path;


//==================
// Graphics Manager
//==================

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

    pub fn init(&mut self) -> Result<(), String> {
        //Load shaders
        self.program = match self.load_shaders(Path::new("res/shaders/triangle.vert"), Path::new("res/shaders/triangle.frag")) {
            Ok(program) => Some(program),
            Err(error) => return Err(format!("{}", error)),
        };

        //Set GL clear color
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        //Triangle vertices
        let vertices: Vec<f32> = vec![
            //Position          //Colors
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,  //Bottom right
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0,  //Bottom left
            0.0, 0.5, 0.0, 0.0, 0.0, 1.0   //Top
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

    ///Create and link OpenGL Program, from Vertex and Fragment shaders.
    fn load_shaders(&mut self, v_path: &Path, f_path: &Path) -> Result<Program, String> {
        //Create shaders and program
        let v_shader = match Shader::from_file(gl::VERTEX_SHADER, v_path) {
            Ok(shader) => shader,
            Err(error) => return Err(format!("Could not load vertex shader from {:?}\nError: {}", v_path, error)),
        };

        let f_shader = match Shader::from_file(gl::FRAGMENT_SHADER, f_path) {
            Ok(shader) => shader,
            Err(error) => return Err(format!("Could not load fragment shader from {:?}\nError: {}", f_path, error)),
        };

        let program = Program::from_shaders(&[v_shader, f_shader])?;

        Ok(program)
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


//==============
// Program Type
//==============

struct Program {
    id: gl::types::GLuint,
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Program {
    fn id(&self) -> gl::types::GLuint {
        self.id
    }

    fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut error_length: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut error_length);
            }

            let error = empty_cstring(error_length as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    error_length,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id })
    }
}


//=============
// Shader Type
//=============

///Represents an OpenGL Shader
struct Shader {
    id: gl::types::GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Shader {
    ///Gets the Shader's OpenGL shader ID.
    fn id(&self) -> gl::types::GLuint {
        self.id
    }

    fn from_file(shader_type: gl::types::GLuint, path: &Path) -> Result<Shader, String> {
        let source = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(error) => return Err(format!("Could not create shader from {:?}\nError: {}", path, error)),
        };

        let source_cstring = match CString::new(source) {
            Ok(cstring) => cstring,
            Err(error) => return Err(format!("Could not convert shader source to CString: {}", error)),
        };

        Shader::from_source(shader_type, &source_cstring)
    }

    ///Create a new shader from GLSL source (provided as a CString), returns Shader object or
    ///OpenGL error log
    ///shader_type: usually gl::VERTEX_SHADER or gl::FRAGMENT_SHADER
    fn from_source(shader_type: gl::types::GLuint, source: &CStr) -> Result<Shader, String> {
        //Create shader and get ID
        let id = unsafe { gl::CreateShader(shader_type) };

        //Compile shader from source
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        //Check shader compile status
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        //Shader compiled successfully
        if success == 1 {
            return Ok(Shader { id });
        }

        //Compilation failed, get error message
        let mut error_length: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut error_length);
        }

        //Allocate CString for error log
        let error_log = empty_cstring(error_length as usize);

        //Fill error log
        unsafe {
            gl::GetShaderInfoLog(
                id,
                error_length,
                std::ptr::null_mut(),
                error_log.as_ptr() as *mut gl::types::GLchar,
            );
        }

        //Return error log
        Err(error_log.to_string_lossy().into_owned())
    }
}

fn empty_cstring(length: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(length as usize + 1);
    buffer.extend([b' '].iter().cycle().take(length as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}
