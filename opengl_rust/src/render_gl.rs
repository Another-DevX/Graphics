use gl::types::*;
use gl::*;
use std::ffi::{CStr, CString};

pub struct Shader {
    id: GLuint,
}

pub struct Program {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, FRAGMENT_SHADER)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            DeleteShader(self.id);
        }
    }
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let id = program_from_shaders(shaders)?;
        Ok(Program { id })
    }

    pub fn set_used(&self) {
        unsafe {
            UseProgram(self.id);
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            DeleteProgram(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: GLenum) -> Result<GLuint, String> {
    let id = unsafe { CreateShader(kind) };
    unsafe {
        ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        CompileShader(id);
    }
    let mut sucess: GLint = 1;
    unsafe {
        GetShaderiv(id, COMPILE_STATUS, &mut sucess);
    }

    if sucess == 0 {
        let mut len: GLint = 0;
        unsafe {
            GetShaderiv(id, INFO_LOG_LENGTH, &mut len);
        }
        let error = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }
        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn program_from_shaders(shaders: &[Shader]) -> Result<GLuint, String> {
    let program_id = unsafe { CreateProgram() };
    for shader in shaders {
        unsafe {
            AttachShader(program_id, shader.id());
        }
    }
    unsafe {
        LinkProgram(program_id);
    }

    let mut success: GLint = 1;

    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
    }
    if success == 0 {
        let mut len: GLint = 0;
        unsafe {
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
        }
        let error = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            GetProgramInfoLog(
                program_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            )
        }

        return Err(error.to_string_lossy().into_owned());
    }
    for shader in shaders {
        unsafe {
            DetachShader(program_id, shader.id());
        }
    }

    Ok(program_id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
