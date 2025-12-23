use crate::error::{Error, Result};
use crate::gl::opengl::{self as gl, GLint, GLuint, GLvoid};
use std::ffi::CString;

// --------------------------------------------------------------------------------
pub fn print_opengl_info(gl: &gl::OpenGlFunctions) {
    unsafe {
        let version = std::ffi::CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
            .to_str()
            .unwrap();
        let vendor = std::ffi::CStr::from_ptr(gl.GetString(gl::VENDOR) as *const _)
            .to_str()
            .unwrap();
        let renderer = std::ffi::CStr::from_ptr(gl.GetString(gl::RENDERER) as *const _)
            .to_str()
            .unwrap();

        println!("OpenGL Version: {version}");
        println!("Vendor: {vendor}");
        println!("Renderer: {renderer}");
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
pub fn create_color_vao(gl: &gl::OpenGlFunctions) -> gl::GLuint {
    unsafe {
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);

        let verts = vec![-1.0, -1.0, -0.5, -1.0, -1.0, -0.5];
        let colors = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        create_vertex_buffer(gl, &verts);
        gl.EnableVertexAttribArray(0); // position
        gl.VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

        create_vertex_buffer(gl, &colors);
        gl.EnableVertexAttribArray(1); // color
        gl.VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

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
) -> GLuint {
    const INTERNAL: [gl::GLint; 3] = [gl::RGBA8, gl::RGB8, gl::R8];
    const FORMAT: [gl::GLenum; 3] = [gl::RGBA, gl::RGB, gl::RED];

    unsafe {
        let mut texture = 0;
        gl.GenTextures(1, &mut texture);
        gl.BindTexture(gl::TEXTURE_2D, texture);
        gl.TexImage2D(
            gl::TEXTURE_2D,
            0,
            INTERNAL[format],
            width as i32,
            height as i32,
            0,
            FORMAT[format],
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const _,
        );
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filter);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap);
        texture
    }
}

// --------------------------------------------------------------------------------
pub fn create_framebuffer(
    gl: &gl::OpenGlFunctions,
    width: usize,
    height: usize,
) -> (gl::GLuint, gl::GLuint, gl::GLuint) {
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
            width as i32,
            height as i32,
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
            gl::DEPTH_COMPONENT24 as i32,
            width as i32,
            height as i32,
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
            panic!("Framebuffer is not complete");
        }

        (fbo, color_tex, depth_tex)
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
