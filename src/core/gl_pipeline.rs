use crate::core::gl_canvas::{GlMaterial, GlMesh};
use crate::core::gl_graphics;
use crate::error::Result;
use crate::gl::opengl as gl;
use crate::v2d::m4x4::M4x4;
use std::rc::Rc;

// ----------------------------------------------------------------------------
pub enum GlPipelineType {
    RGBATex = 0,
    YUVTex = 1,
    MSDFTex = 2,
}

// ----------------------------------------------------------------------------
impl From<GlPipelineType> for usize {
    fn from(p: GlPipelineType) -> Self {
        match p {
            GlPipelineType::RGBATex => 0,
            GlPipelineType::YUVTex => 1,
            GlPipelineType::MSDFTex => 2,
        }
    }
}

// ----------------------------------------------------------------------------
pub struct GlUniforms {
    pub model: M4x4,
    pub camera: M4x4,
    pub mat_id: gl::GLint,
}

// --------------------------------------------------------------------------------
pub trait GlPipeline {
    fn render(&self, bindings: &GlMesh, material: &GlMaterial, unis: &GlUniforms) -> Result<()>;
}

pub mod v_pos_tex {
    use crate::core::gl_canvas::GlMaterial;

    use super::*;

    // ----------------------------------------------------------------------------
    pub struct Pipeline {
        pub gl: Rc<gl::OpenGlFunctions>,
        pub shader: gl::GLuint,
        pub uid_model: gl::GLint,
        pub uid_camera: gl::GLint,
        pub uid_mat_id: gl::GLint,
    }

    // ----------------------------------------------------------------------------
    impl Pipeline {
        pub fn new(gl: Rc<gl::OpenGlFunctions>) -> Result<Self> {
            let shader = gl_graphics::create_program(&gl, "pos_tex", VS_TEXTURE, FS_TEXTURE);
            if let Err(e) = shader {
                println!("Error creating shader: {e:?}");
                return Err(e);
            };
            let shader = shader.unwrap();
            let uid_model = gl_graphics::get_uniform_location(&gl, shader, "model").unwrap_or(-1);
            let uid_camera = gl_graphics::get_uniform_location(&gl, shader, "camera").unwrap_or(-1);
            let uid_mat_id = gl_graphics::get_uniform_location(&gl, shader, "mat_id").unwrap_or(-1);
            Ok(Pipeline {
                gl,
                shader,
                uid_model,
                uid_camera,
                uid_mat_id,
            })
        }
    }

    // ----------------------------------------------------------------------------
    impl GlPipeline for Pipeline {
        fn render(
            &self,
            bindings: &GlMesh,
            material: &GlMaterial,
            unis: &GlUniforms,
        ) -> Result<()> {
            let gl = &self.gl;
            let texture = if let GlMaterial::Texture(id) = material {
                *id
            } else {
                1
            };
            unsafe {
                gl.UseProgram(self.shader);
                gl.BindVertexArray(bindings.vao);
                gl.UniformMatrix4fv(self.uid_model, 1, gl::FALSE, unis.model.as_ptr());
                gl.UniformMatrix4fv(self.uid_camera, 1, gl::FALSE, unis.camera.as_ptr());
                gl.Uniform1i(self.uid_mat_id, unis.mat_id);
                gl.ActiveTexture(gl::TEXTURE0);
                gl.BindTexture(gl::TEXTURE_2D, texture);
                gl.DrawArrays(gl::TRIANGLES, 0, bindings.count as gl::GLint);
            }
            Ok(())
        }
    }

    // ----------------------------------------------------------------------------
    impl Drop for Pipeline {
        fn drop(&mut self) {
            unsafe {
                self.gl.DeleteProgram(self.shader);
            }
        }
    }

    // ----------------------------------------------------------------------------
    const VS_TEXTURE: &str = r#"
    #version 300 es
    uniform mat4 model;
    uniform mat4 camera;

    layout (location = 0) in vec2 a_pos;
    layout (location = 1) in vec2 a_tex;

    out vec2 v_tex;

    void main() {
        gl_Position = camera * model * vec4(a_pos, 0.0, 1.0);
        v_tex = a_tex;
    }"#;

    // ----------------------------------------------------------------------------
    const FS_TEXTURE: &str = r#"
    #version 300 es
    uniform sampler2D txtre;

    in mediump vec2 v_tex;
    out mediump vec4 FragColor;

    void main() {
        FragColor = texture(txtre, v_tex.st);
    }"#;
}

pub mod msdf_tex {
    use crate::core::gl_canvas::GlMaterial;

    use super::*;

    // ----------------------------------------------------------------------------
    pub struct Pipeline {
        pub gl: Rc<gl::OpenGlFunctions>,
        pub shader: gl::GLuint,
        pub uid_model: gl::GLint,
        pub uid_camera: gl::GLint,
        pub uid_mat_id: gl::GLint,
    }

    // ----------------------------------------------------------------------------
    impl Pipeline {
        pub fn new(gl: Rc<gl::OpenGlFunctions>) -> Result<Self> {
            let shader = gl_graphics::create_program(&gl, "msdf_tex", VS_TEXTURE, FS_TEXTURE);
            if let Err(e) = shader {
                println!("Error creating shader: {e:?}");
                return Err(e);
            };
            let shader = shader.unwrap();
            let uid_model = gl_graphics::get_uniform_location(&gl, shader, "model").unwrap_or(-1);
            let uid_camera = gl_graphics::get_uniform_location(&gl, shader, "camera").unwrap_or(-1);
            let uid_mat_id = gl_graphics::get_uniform_location(&gl, shader, "mat_id").unwrap_or(-1);
            Ok(Pipeline {
                gl,
                shader,
                uid_model,
                uid_camera,
                uid_mat_id,
            })
        }
    }

    // ----------------------------------------------------------------------------
    impl GlPipeline for Pipeline {
        fn render(
            &self,
            bindings: &GlMesh,
            material: &GlMaterial,
            unis: &GlUniforms,
        ) -> Result<()> {
            let gl = &self.gl;
            let texture = if let GlMaterial::Texture(id) = material {
                *id
            } else {
                1
            };
            unsafe {
                gl.UseProgram(self.shader);
                gl.BindVertexArray(bindings.vao);
                gl.UniformMatrix4fv(self.uid_model, 1, gl::FALSE, unis.model.as_ptr());
                gl.UniformMatrix4fv(self.uid_camera, 1, gl::FALSE, unis.camera.as_ptr());
                gl.Uniform1i(self.uid_mat_id, unis.mat_id);
                gl.Enable(gl::BLEND);
                gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                gl.ActiveTexture(gl::TEXTURE0);
                gl.BindTexture(gl::TEXTURE_2D, texture);
                gl.DrawArrays(gl::TRIANGLES, 0, bindings.count as gl::GLint);
            }
            Ok(())
        }
    }

    // ----------------------------------------------------------------------------
    impl Drop for Pipeline {
        fn drop(&mut self) {
            unsafe {
                self.gl.DeleteProgram(self.shader);
            }
        }
    }

    // ----------------------------------------------------------------------------
    const VS_TEXTURE: &str = r#"
    #version 300 es
    uniform mat4 model;
    uniform mat4 camera;

    layout (location = 0) in vec2 a_pos;
    layout (location = 1) in vec2 a_tex;

    out vec2 v_tex;

    void main() {
        gl_Position = camera * model * vec4(a_pos, 0.0, 1.0);
        v_tex = a_tex;
    }"#;

    // ----------------------------------------------------------------------------
    const FS_TEXTURE: &str = r#"
    #version 300 es
    uniform sampler2D txtre;

    in mediump vec2 v_tex;
    out mediump vec4 FragColor;

    void main() {
        mediump vec4 color = texture(txtre, v_tex.st);
        mediump float sig_dist = color.a * 2.0 - 1.0;
        mediump float alpha = smoothstep(-0.1, 0.1, sig_dist);
        FragColor = vec4(alpha, alpha, alpha, alpha);
    }"#;
}

pub mod v_yuv_tex {
    use crate::core::gl_canvas::GlMaterial;

    use super::*;

    // ----------------------------------------------------------------------------
    pub struct Pipeline {
        pub gl: Rc<gl::OpenGlFunctions>,
        pub shader: gl::GLuint,
        pub uid_model: gl::GLint,
        pub uid_camera: gl::GLint,
        pub uid_mat_id: gl::GLint,
        pub uid_tex_y: gl::GLint,
        pub uid_tex_cb: gl::GLint,
        pub uid_tex_cr: gl::GLint,
    }

    // ----------------------------------------------------------------------------
    impl Pipeline {
        pub fn new(gl: Rc<gl::OpenGlFunctions>) -> Result<Self> {
            let shader = gl_graphics::create_program(&gl, "yuv_tex", VS_TEXTURE, FS_TEXTURE);
            if let Err(e) = shader {
                println!("Error creating shader: {e:?}");
                return Err(e);
            };
            let shader = shader.unwrap();
            let uid_model = gl_graphics::get_uniform_location(&gl, shader, "model").unwrap_or(-1);
            let uid_camera = gl_graphics::get_uniform_location(&gl, shader, "camera").unwrap_or(-1);
            let uid_mat_id = gl_graphics::get_uniform_location(&gl, shader, "mat_id").unwrap_or(-1);
            let uid_tex_y = gl_graphics::get_uniform_location(&gl, shader, "tex_y").unwrap_or(-1);
            let uid_tex_cb = gl_graphics::get_uniform_location(&gl, shader, "tex_cb").unwrap_or(-1);
            let uid_tex_cr = gl_graphics::get_uniform_location(&gl, shader, "tex_cr").unwrap_or(-1);

            Ok(Pipeline {
                gl,
                shader,
                uid_model,
                uid_camera,
                uid_mat_id,
                uid_tex_y,
                uid_tex_cb,
                uid_tex_cr,
            })
        }
    }

    // ----------------------------------------------------------------------------
    impl GlPipeline for Pipeline {
        fn render(
            &self,
            bindings: &GlMesh,
            material: &GlMaterial,
            unis: &GlUniforms,
        ) -> Result<()> {
            let gl = &self.gl;
            let (tex_y, tex_cb, tex_cr) = if let GlMaterial::YuvTexture(y, cb, cr) = material {
                (*y, *cb, *cr)
            } else {
                (0, 0, 0)
            };
            unsafe {
                gl.UseProgram(self.shader);
                gl.BindVertexArray(bindings.vao);
                gl.UniformMatrix4fv(self.uid_model, 1, gl::FALSE, unis.model.as_ptr());
                gl.UniformMatrix4fv(self.uid_camera, 1, gl::FALSE, unis.camera.as_ptr());
                gl.Uniform1i(self.uid_mat_id, unis.mat_id);
                gl.Uniform1i(self.uid_tex_y, 0);
                gl.Uniform1i(self.uid_tex_cb, 1);
                gl.Uniform1i(self.uid_tex_cr, 2);
                gl.ActiveTexture(gl::TEXTURE0);
                gl.BindTexture(gl::TEXTURE_2D, tex_y);
                gl.ActiveTexture(gl::TEXTURE1);
                gl.BindTexture(gl::TEXTURE_2D, tex_cb);
                gl.ActiveTexture(gl::TEXTURE2);
                gl.BindTexture(gl::TEXTURE_2D, tex_cr);
                gl.DrawArrays(gl::TRIANGLE_STRIP, 0, bindings.count as gl::GLint);
            }
            Ok(())
        }
    }

    // ----------------------------------------------------------------------------
    impl Drop for Pipeline {
        fn drop(&mut self) {
            unsafe {
                self.gl.DeleteProgram(self.shader);
            }
        }
    }

    // ----------------------------------------------------------------------------
    const VS_TEXTURE: &str = r#"
    #version 300 es
    uniform mat4 model;
    uniform mat4 camera;

    layout (location = 0) in vec2 a_pos;
    layout (location = 1) in vec2 a_tex;

    out vec2 v_tex;

    void main() {
        gl_Position = camera * model * vec4(a_pos, 0.0, 1.0);
        v_tex = a_tex;
    }"#;

    // ----------------------------------------------------------------------------
    const FS_TEXTURE: &str = r#"
    #version 300 es
    uniform sampler2D tex_y;
    uniform sampler2D tex_cb;
    uniform sampler2D tex_cr;

    in mediump vec2 v_tex;
    out mediump vec4 FragColor;

    void main() {
        mediump vec3 yuv;
        yuv.x = texture(tex_y, v_tex.st).r;
        yuv.y = texture(tex_cb, v_tex.st).r - 0.5;
        yuv.z = texture(tex_cr, v_tex.st).r - 0.5;
        mediump vec3 rgb;
        rgb.r = yuv.x + 1.402 * yuv.z;
        rgb.g = yuv.x - 0.344 * yuv.y - 0.714 * yuv.z;
        rgb.b = yuv.x + 1.772 * yuv.y;
        FragColor = vec4(rgb, 1.0);
    }"#;
}
