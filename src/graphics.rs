use std;
use std::ffi::{CString, CStr};
use std::fs;
use std::path::Path;

use gl;
use sdl2;

use config;


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
            .build().unwrap();

        //Initialize OpenGL
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);
   

        //Build and return GraphicsManager
        GraphicsManager {conf, sdl, video, window, gl_context, program: None}
    }

    ///Create and link OpenGL Program, from Vertex and Fragment shaders.
    pub fn load_shaders(&mut self, v_path: &Path, f_path: &Path) {
        //Create shaders and program
        let v_shader = Shader::from_file(gl::VERTEX_SHADER, v_path).unwrap();
        let f_shader = Shader::from_file(gl::FRAGMENT_SHADER, f_path).unwrap();

        let program = Program::from_shaders(&[v_shader, f_shader]).unwrap();

        program.set_used();
        self.program = Some(program);
    }

    ///Renders the current frame
    pub fn render(&self) {
        unsafe {
            gl::Viewport(0, 0, self.conf.display.width as i32, self.conf.display.height as i32);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.window.gl_swap_window(); 
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
        unsafe { gl::DeleteProgram(self.id); }
    }
}

impl Program {
    fn id(&self) -> gl::types::GLuint {
        self.id
    }

    fn set_used(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        unsafe { gl::LinkProgram(program_id); }

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
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        Ok(Program {id: program_id})
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
        unsafe { gl::DeleteShader(self.id); }
    }
}

impl Shader {
    ///Gets the Shader's OpenGL shader ID.
    fn id(&self) -> gl::types::GLuint {
        self.id
    }

    fn from_file(shader_type: gl::types::GLuint, path: &Path) -> Result<Shader, String> {
        let source = fs::read_to_string(path).unwrap();
        Shader::from_source(shader_type, &CString::new(source).unwrap())
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
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success); }

        //Shader compiled successfully
        if success == 1 { return Ok(Shader{id}); }

        //Compilation failed, get error message
        let mut error_length: gl::types::GLint = 0;
        unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut error_length); }

        //Allocate CString for error log
        let error_log = empty_cstring(error_length as usize);
        
        //Fill error log
        unsafe {
            gl::GetShaderInfoLog(
                id,
                error_length,
                std::ptr::null_mut(),
                error_log.as_ptr() as *mut gl::types::GLchar
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

