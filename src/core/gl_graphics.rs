use crate::error::{Error, Result};
use crate::gl::opengl::{self as gl, GLint, GLuint, GLvoid};
use std::ffi::CString;

// --------------------------------------------------------------------------------
pub fn print_opengl_info(gl: &gl::OpenGlFunctions) {
    unsafe {
        let version = std::ffi::CStr::from_ptr(gl.GetString(gl::VERSION) as *const _).to_str();
        let vendor = std::ffi::CStr::from_ptr(gl.GetString(gl::VENDOR) as *const _).to_str();
        let renderer = std::ffi::CStr::from_ptr(gl.GetString(gl::RENDERER) as *const _).to_str();
        let mut max_size = 0;
        gl.GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut max_size);
        let mut max_units = 0;
        gl.GetIntegerv(gl::MAX_TEXTURE_UNITS, &mut max_units);

        println!("OpenGL Version:  {}", version.unwrap_or("<error>"));
        println!("OpenGL Vendor:   {}", vendor.unwrap_or("<error>"));
        println!("OpenGL Renderer: {}", renderer.unwrap_or("<error>"));
        println!("OpenGL Max Texture Size:  {max_size}");
        println!("OpenGL Max Texture Units: {max_units}");
    }
}

// --------------------------------------------------------------------------------
pub fn check_gl_error(gl: &gl::OpenGlFunctions) -> Result<()> {
    unsafe {
        let error = gl.GetError();
        match error {
            0 => Ok(()),
            gl::OUT_OF_MEMORY => Err(Error::GpuOutOfMemory),
            _ => Err(Error::OpenGl { code: error }),
        }
    }
}

// --------------------------------------------------------------------------------
pub fn create_shader(
    gl: &gl::OpenGlFunctions,
    shader_type: gl::GLenum,
    name: &str,
    source: &str,
) -> Result<gl::GLuint> {
    unsafe {
        let Ok(csource) = CString::new(source) else {
            return Err(Error::InvalidCString);
        };
        let shader = gl.CreateShader(shader_type);
        let csource_ptr = csource.as_ptr();
        gl.ShaderSource(shader, 1, &csource_ptr, std::ptr::null());
        gl.CompileShader(shader);

        let mut is_compiled = 0;
        gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut is_compiled);
        if is_compiled == 0 {
            let mut log_length = 0;
            gl.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
            let mut log = vec![0; log_length as usize];
            gl.GetShaderInfoLog(
                shader,
                log_length,
                std::ptr::null_mut(),
                log.as_mut_ptr() as *mut _,
            );
            let log_str = String::from_utf8_lossy(&log);
            gl.DeleteShader(shader);
            return Err(Error::ShaderLoad {
                name: name.to_string(),
                log: log_str.to_string(),
            });
        }

        Ok(shader)
    }
}

// --------------------------------------------------------------------------------
pub fn create_program(
    gl: &gl::OpenGlFunctions,
    name: &str,
    vs: &str,
    fs: &str,
) -> Result<gl::GLuint> {
    unsafe {
        let vs = create_shader(gl, gl::VERTEX_SHADER, format!("{name}/vertex").as_str(), vs)?;
        let fs = create_shader(gl, gl::FRAGMENT_SHADER, format!("{name}/frag").as_str(), fs)?;

        let program = gl.CreateProgram();
        gl.AttachShader(program, vs);
        gl.AttachShader(program, fs);
        gl.LinkProgram(program);
        gl.DeleteShader(vs);
        gl.DeleteShader(fs);

        let mut is_linked = 0;
        gl.GetProgramiv(program, gl::LINK_STATUS, &mut is_linked);
        if is_linked == 0 {
            let mut log_length = 0;
            gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);
            let mut log = vec![0; log_length as usize];
            gl.GetProgramInfoLog(
                program,
                log_length,
                std::ptr::null_mut(),
                log.as_mut_ptr() as *mut _,
            );
            let log_str = String::from_utf8_lossy(&log);
            gl.DeleteProgram(program);
            return Err(Error::ShaderLoad {
                name: name.to_string(),
                log: log_str.to_string(),
            });
        }
        Ok(program)
    }
}

// --------------------------------------------------------------------------------
pub fn delete_buffer(gl: &gl::OpenGlFunctions, vbo: gl::GLuint) {
    unsafe {
        gl.DeleteBuffers(1, &vbo);
    }
}

// --------------------------------------------------------------------------------
/// Creates an OpenGL buffer and uploads data to it.
///
/// # Safety
/// The caller must ensure that `data` points to valid memory of at least `size` bytes.
pub unsafe fn create_buffer(
    gl: &gl::OpenGlFunctions,
    target: gl::GLenum,
    data: *const GLvoid,
    size: usize,
) -> gl::GLuint {
    unsafe {
        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(target, vbo);
        gl.BufferData(target, size, data, gl::STATIC_DRAW);
        vbo
    }
}

// --------------------------------------------------------------------------------
pub fn create_vertex_buffer(gl: &gl::OpenGlFunctions, data: &[gl::GLfloat]) -> gl::GLuint {
    unsafe {
        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(data),
            data.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        vbo
    }
}

// --------------------------------------------------------------------------------
pub fn delete_vertex_array(gl: &gl::OpenGlFunctions, vao: gl::GLuint) {
    unsafe {
        gl.DeleteVertexArrays(1, &vao);
    }
}

// --------------------------------------------------------------------------------
pub fn create_vertex_array(gl: &gl::OpenGlFunctions) -> gl::GLuint {
    unsafe {
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);
        vao
    }
}

// --------------------------------------------------------------------------------
pub fn create_texture_vao(gl: &gl::OpenGlFunctions) -> gl::GLuint {
    unsafe {
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);

        let verts = vec![-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
        let texcoords = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        create_vertex_buffer(gl, &verts);
        gl.EnableVertexAttribArray(0); // position
        gl.VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

        create_vertex_buffer(gl, &texcoords);
        gl.EnableVertexAttribArray(1); // texcoord
        gl.VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

        vao
    }
}

// --------------------------------------------------------------------------------
pub fn delete_texture(gl: &gl::OpenGlFunctions, texture: GLuint) {
    let textures = [texture];
    unsafe {
        gl.DeleteTextures(textures.len() as i32, textures.as_ptr());
    }
}

// --------------------------------------------------------------------------------
pub fn create_texture(
    gl: &gl::OpenGlFunctions,
    width: usize,
    height: usize,
    format: usize,
    data: &[u8],
    filter: GLint,
    wrap: GLint,
) -> Result<GLuint> {
    let mut max_size = 0;
    unsafe {
        gl.GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut max_size);
    }

    let width = check_texture_size(width, max_size)?;
    let height = check_texture_size(height, max_size)?;

    const INTERNAL_FMT: [(gl::GLint, gl::GLenum); 3] = [
        (gl::RGBA8, gl::RGBA),
        (gl::RGB8, gl::RGB),
        (gl::R8, gl::RED),
    ];
    let Some((internal, format)) = INTERNAL_FMT.get(format) else {
        return Err(Error::InvalidTextureFormat);
    };

    let mut texture = 0;
    unsafe {
        gl.GenTextures(1, &mut texture);
        gl.BindTexture(gl::TEXTURE_2D, texture);
        gl.TexImage2D(
            gl::TEXTURE_2D,
            0,
            *internal,
            width,
            height,
            0,
            *format,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const _,
        );

        if let Err(e) = check_gl_error(gl) {
            gl.DeleteTextures(1, &texture);
            return Err(e);
        }

        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap);
    }

    Ok(texture)
}

// --------------------------------------------------------------------------------
pub fn create_framebuffer(
    gl: &gl::OpenGlFunctions,
    width: usize,
    height: usize,
) -> Result<(gl::GLuint, gl::GLuint, gl::GLuint)> {
    let mut max_size = 0;
    unsafe {
        gl.GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut max_size);
    }

    let width = check_texture_size(width, max_size)?;
    let height = check_texture_size(height, max_size)?;

    unsafe {
        let mut fbo = 0;

        gl.GenFramebuffers(1, &mut fbo);
        gl.BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut color_tex = 0;
        gl.GenTextures(1, &mut color_tex);
        gl.BindTexture(gl::TEXTURE_2D, color_tex);
        gl.TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8,
            width,
            height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);
        gl.FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT,
            gl::TEXTURE_2D,
            color_tex,
            0,
        );

        // --- Create depth texture ---
        let mut depth_tex = 0;
        gl.GenTextures(1, &mut depth_tex);
        gl.BindTexture(gl::TEXTURE_2D, depth_tex);
        gl.TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::DEPTH_COMPONENT24,
            width,
            height,
            0,
            gl::DEPTH_COMPONENT,
            gl::FLOAT,
            std::ptr::null(),
        );
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
        gl.FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::DEPTH_ATTACHMENT,
            gl::TEXTURE_2D,
            depth_tex,
            0,
        );

        gl.DrawBuffers(1, [gl::COLOR_ATTACHMENT].as_ptr());

        let status = gl.CheckFramebufferStatus(gl::FRAMEBUFFER);
        if status != gl::FRAMEBUFFER_COMPLETE {
            gl.DeleteFramebuffers(1, &fbo);
            gl.DeleteTextures(1, &color_tex);
            gl.DeleteTextures(1, &depth_tex);
            return Err(Error::Framebuffer { status });
        }

        Ok((fbo, color_tex, depth_tex))
    }
}

// --------------------------------------------------------------------------------
pub fn get_uniform_location(
    gl: &gl::OpenGlFunctions,
    program: gl::GLuint,
    name: &str,
) -> Result<gl::GLint> {
    let cname = CString::new(name).map_err(|_| Error::InvalidCString)?;
    let location = unsafe { gl.GetUniformLocation(program, cname.as_ptr()) };
    (location != -1)
        .then_some(location)
        .ok_or(Error::InvalidLocation)
}

// --------------------------------------------------------------------------------
fn check_texture_size(size: usize, max_size: i32) -> Result<i32> {
    let size = size.try_into().map_err(|_| Error::InvalidTextureSize)?;
    if size == 0 || size > max_size {
        Err(Error::InvalidTextureSize)
    } else {
        Ok(size)
    }
}
