use gl;
use std;
use std::ffi::{CStr, CString};
use std::path::Path;

use resources::ResourceLoader;

use graphics::Error;

///Represents an OpenGL Shader Program.
pub struct Program {
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

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    ///Create Program from vertex and fragment shader paths.
    pub fn load_shaders(resource_loader: &ResourceLoader, v_path: &Path, f_path: &Path) -> Result<Program, Error> {
        //Create shaders and program
        let v_shader = match Shader::from_file(resource_loader, gl::VERTEX_SHADER, v_path) {
            Ok(shader) => shader,
            Err(error) => return Err(Error::ResourceError { path: v_path.into(), message: error.into() }),
        };

        let f_shader = match Shader::from_file(resource_loader, gl::FRAGMENT_SHADER, f_path) {
            Ok(shader) => shader,
            Err(error) => return Err(Error::ResourceError { path: f_path.into(), message: error.into() }),
        };

        let program = Program::from_shaders(&[v_shader, f_shader])?;

        Ok(program)
    }

    ///Create Program from Shaders.
    fn from_shaders(shaders: &[Shader]) -> Result<Program, Error> {
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

            return Err(Error::ProgramLinkingFailed {
                message: error.to_string_lossy().into_owned()
            });
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id })
    }
}


///Represents an OpenGL Shader
pub struct Shader {
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

    ///Creates shader from source file.
    ///shader_type: usually gl::VERTEX_SHADER or gl::FRAGMENT_SHADER
    pub fn from_file(resource_loader: &ResourceLoader, shader_type: gl::types::GLuint, path: &Path) -> Result<Shader, Error> {
        let text = match resource_loader.load_cstring(path) {
            Ok(text) => text,
            Err(error) => return Err(Error::ResourceError { path: path.into(), message: error.into() }),
        };

        Shader::from_source(shader_type, &text)
            .map_err(|error| Error::ResourceError { path: path.into(), message: error.into() })
    }

    ///Create a new shader from GLSL source (provided as a CString), returns Shader object or OpenGL error log.
    ///shader_type: usually gl::VERTEX_SHADER or gl::FRAGMENT_SHADER
    fn from_source(shader_type: gl::types::GLuint, source: &CStr) -> Result<Shader, Error> {
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
        Err(Error::ShaderCompilationFailed {
            message: error_log.to_string_lossy().into_owned(),
        })
    }
}

///Creates and returns a `CString` filled with spaces, of length `length`.
fn empty_cstring(length: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(length as usize + 1);
    buffer.extend([b' '].iter().cycle().take(length as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}
