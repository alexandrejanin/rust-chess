use cgmath::{Array, Matrix};
use gl;
use maths::{Matrix4f, Vector2f};
use resources::{self, ResourceLoader};
use std::{
    self,
    ffi::{self, CStr, CString},
    fmt::{self, Display, Formatter},
    path::Path,
};
use super::ProgramID;

///Error related to shaders.
#[derive(Debug)]
pub enum ShaderError {
    NulError(ffi::NulError),
    ///An error related to resources handling.
    ResourceError(resources::ResourceError),
    ///OpenGL Shader could not compile. Contains OpenGL Error log.
    ShaderCompilationFailed(String),
    ///OpenGL Program could not link. Contains OpenGL Error log.
    ProgramLinkingFailed(String),
}

impl From<resources::ResourceError> for ShaderError {
    fn from(error: resources::ResourceError) -> Self { ShaderError::ResourceError(error) }
}

impl From<ffi::NulError> for ShaderError {
    fn from(error: ffi::NulError) -> Self { ShaderError::NulError(error) }
}

impl Display for ShaderError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Shader error: ")?;
        match self {
            ShaderError::NulError(error) => write!(f, "{}", error),
            ShaderError::ResourceError(error) => write!(f, "{}", error),
            ShaderError::ShaderCompilationFailed(message) => write!(f, "Shader could not compile: {}", message),
            ShaderError::ProgramLinkingFailed(message) => write!(f, "Program could not link: {}", message),
        }
    }
}


///Represents an OpenGL Shader Program.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Program {
    id: ProgramID,
}

impl Program {
    pub fn id(&self) -> ProgramID {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id());
        }
    }


    ///Attempts to set uniform mat4. Returns success value.
    pub fn set_mat4(&self, name: &str, mat4: &Matrix4f) -> bool { self.set_mat4_arr(name, std::slice::from_ref(mat4)) }

    ///Attempts to set uniform mat4. Returns success value.
    pub fn set_mat4_arr(&self, name: &str, mat4s: &[Matrix4f]) -> bool {
        let loc = self.get_uniform_location(name);
        if loc == -1 { return false }

        unsafe { gl::UniformMatrix4fv(loc, mat4s.len() as gl::types::GLint, gl::FALSE, mat4s[0].as_ptr()); }

        true
    }

    ///Attempts to set uniform vec2. Returns false if the uniform was not found.
    pub fn set_vec2(&self, name: &str, vec2: &Vector2f) -> bool { self.set_vec2_arr(name, std::slice::from_ref(vec2)) }

    ///Attempts to set uniform vec2. Returns false if the uniform was not found.
    pub fn set_vec2_arr(&self, name: &str, vec2s: &[Vector2f]) -> bool {
        let loc = self.get_uniform_location(name);
        if loc == -1 { return false }

        unsafe { gl::Uniform2fv(loc, vec2s.len() as gl::types::GLint, vec2s[0].as_ptr()); }

        true
    }

    ///Returns uniform location in program from uniform name.
    fn get_uniform_location(&self, name: &str) -> gl::types::GLint {
        let uniform_name = CString::new(name).unwrap();

        unsafe { gl::GetUniformLocation(self.id, uniform_name.as_ptr()) }
    }


    ///Create Program from vertex and fragment shader paths.
    pub fn load_shaders(resource_loader: &ResourceLoader, v_path: &Path, f_path: &Path) -> Result<Program, ShaderError> {
        //Create shaders and program
        let v_shader = Shader::from_file(resource_loader, gl::VERTEX_SHADER, v_path)?;

        let f_shader = Shader::from_file(resource_loader, gl::FRAGMENT_SHADER, f_path)?;

        let program = Program::from_shaders(&[v_shader, f_shader])?;

        Ok(program)
    }

    ///Create Program from Shaders.
    fn from_shaders(shaders: &[Shader]) -> Result<Program, ShaderError> {
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

            return Err(ShaderError::ProgramLinkingFailed(
                error.to_string_lossy().into_owned()
            ));
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
                gl::DeleteShader(shader.id())
            }
        }

        Ok(Program { id: program_id })
    }
}


///Represents an OpenGL Shader
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub fn from_file(resource_loader: &ResourceLoader, shader_type: gl::types::GLuint, path: &Path) -> Result<Shader, ShaderError> {
        let text = CString::new(resource_loader.load_string(path)?)?;

        Shader::from_source(shader_type, &text)
    }

    ///Create a new shader from GLSL source (provided as a CString), returns Shader object or OpenGL error log.
    ///shader_type: usually gl::VERTEX_SHADER or gl::FRAGMENT_SHADER
    fn from_source(shader_type: gl::types::GLuint, source: &CStr) -> Result<Shader, ShaderError> {
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
        Err(ShaderError::ShaderCompilationFailed(
            error_log.to_string_lossy().into_owned(),
        ))
    }
}

///Creates and returns a CString filled with 'length' spaces.
fn empty_cstring(length: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(length as usize + 1);
    buffer.extend([b' '].iter().cycle().take(length as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}
