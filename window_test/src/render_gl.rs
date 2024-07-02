use gl;
use std;
use std::ffi::{ CString, CStr };

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        // create program
        let id: u32 = unsafe { gl.CreateProgram() };
        // attach shaders via opengl
        for shader in shaders {
            unsafe { gl.AttachShader(id, shader.id()); }
        }
        // link program
        unsafe { gl.LinkProgram(id); }

        // check for errors
        let mut success :gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            // Read length of error message
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstr_of_len(len as usize);
            unsafe {
                gl.GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
        return Err(error.to_string_lossy().into_owned());
        }
        // detach shaders so they can be freed later
        for shader in shaders {
            unsafe {gl.DetachShader(id, shader.id()); }
        }
        Ok(Program{gl: gl.clone(), id})
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    pub fn id(&mut self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id)
        }
    }
}

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader {
    fn from_source(
        gl: &gl::Gl,
        source: &CStr,
        kind: gl::types::GLenum,
    ) -> Result<Shader, String> {
        let id = shader_from_source(gl, source, kind)?;
        Ok(Shader { gl: gl.clone(), id })
    }
    pub fn from_vert_source(
        gl: &gl::Gl,
        source: &CStr,
    ) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }
    pub fn from_frag_source(
        gl: &gl::Gl,
        source: &CStr,
    ) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}
// function takes in source as text + kind as GLuint, creates and compiles shader.
// Returns OK or error
// this method is considered safe despite use of unsafe code because it will only ever
// return Ok if everything else has finished successfully?
fn shader_from_source(
    gl: &gl::Gl,
    source: &CStr,
    kind: gl::types::GLuint,
) -> Result<gl::types::GLuint, String> {
    // Create and compile Shader
    let id = unsafe { gl.CreateShader( kind ) };
    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    // Determine Success
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    // Return Error on failure
    if success == 0 {
        // Read length of error message
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        let error = create_whitespace_cstr_of_len(len as usize);
        unsafe {
            gl.GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }
        return Err(error.to_string_lossy().into_owned());
    }
    Ok( id )
}
// first thing, SIN/COS/TAN visualizations

fn create_whitespace_cstr_of_len(len: usize) -> CString {
    // allocate buffer<u8> to contain data (len + 1 for null char)
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    // fill it with empty spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));
    // convert to cstr
    unsafe { CString::from_vec_unchecked(buffer) }
}
