use crate::core::gl_canvas::{GlMaterial, GlMesh};
use crate::core::gl_graphics;
use crate::error::Result;
use crate::gl::opengl as gl;
use crate::v2d::{m4x4::M4x4, v2::V2};
use std::rc::Rc;

// ----------------------------------------------------------------------------
pub enum GlPipelineType {
    RGBATex = 0,
    YUVTex = 1,
    MSDFTex = 2,
    YUVDual = 3,
    Colored = 4,
}

// ----------------------------------------------------------------------------
impl From<GlPipelineType> for usize {
    fn from(p: GlPipelineType) -> Self {
        match p {
            GlPipelineType::RGBATex => 0,
            GlPipelineType::YUVTex => 1,
            GlPipelineType::MSDFTex => 2,
            GlPipelineType::YUVDual => 3,
            GlPipelineType::Colored => 4,
        }
    }
}

// ----------------------------------------------------------------------------
pub struct GlUniforms {
    pub model: M4x4,
    pub camera: M4x4,
    pub mat_id: gl::GLint,
    pub progress: f32,
    pub from_pos: V2,
    pub from_size: V2,
    pub to_pos: V2,
    pub to_size: V2,
}

// --------------------------------------------------------------------------------
pub trait GlPipeline {
    fn render(&self, mesh: &GlMesh, material: &GlMaterial, unis: &GlUniforms) -> Result<()>;
}

// --------------------------------------------------------------------------------
pub trait GlTransition {
    fn render(
        &self,
        mesh: &GlMesh,
        from: &GlMaterial,
        to: &GlMaterial,
        unis: &GlUniforms,
    ) -> Result<()>;
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
        pub uid_yuv: gl::GLint,
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
            let uid_yuv = gl_graphics::get_uniform_location(&gl, shader, "yuv_tex").unwrap_or(-1);

            Ok(Pipeline {
                gl,
                shader,
                uid_model,
                uid_camera,
                uid_mat_id,
                uid_yuv,
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
            let tex = if let GlMaterial::Texture(id) = material {
                *id
            } else {
                0
            };
            unsafe {
                gl.UseProgram(self.shader);
                gl.BindVertexArray(bindings.vao);
                gl.UniformMatrix4fv(self.uid_model, 1, gl::FALSE, unis.model.as_ptr());
                gl.UniformMatrix4fv(self.uid_camera, 1, gl::FALSE, unis.camera.as_ptr());
                gl.Uniform1i(self.uid_mat_id, unis.mat_id);
                gl.Uniform1i(self.uid_yuv, 0);
                gl.ActiveTexture(gl::TEXTURE0);
                gl.BindTexture(gl::TEXTURE_2D, tex);
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
    uniform sampler2D yuv_tex;

    in mediump vec2 v_tex;
    out mediump vec4 FragColor;

    void main() {
        mediump vec3 yuv;
        yuv.x = texture(yuv_tex, v_tex.st).r;
        yuv.y = texture(yuv_tex, v_tex.st).g - 0.5;
        yuv.z = texture(yuv_tex, v_tex.st).b - 0.5;
        mediump vec3 rgb;
        rgb.r = yuv.x + 1.402 * yuv.z;
        rgb.g = yuv.x - 0.344 * yuv.y - 0.714 * yuv.z;
        rgb.b = yuv.x + 1.772 * yuv.y;
        FragColor = vec4(rgb, 1.0);
    }"#;
}

pub mod yuv_dual {
    use super::*;
    use crate::core::gl_canvas::GlMaterial;

    // ----------------------------------------------------------------------------
    pub struct Transition {
        pub gl: Rc<gl::OpenGlFunctions>,
        pub shader: gl::GLuint,
        pub uid_model: gl::GLint,
        pub uid_camera: gl::GLint,
        pub uid_from_tex: gl::GLint,
        pub uid_to_tex: gl::GLint,
        pub uid_progress: gl::GLint,
        pub uid_from_pos: gl::GLint,
        pub uid_from_size: gl::GLint,
        pub uid_to_pos: gl::GLint,
        pub uid_to_size: gl::GLint,
    }

    // ----------------------------------------------------------------------------
    impl Transition {
        pub fn new(gl: Rc<gl::OpenGlFunctions>) -> Result<Self> {
            let shader = gl_graphics::create_program(&gl, "yuv_dual", VS_TEXTURE, FS_TEXTURE);
            if let Err(e) = shader {
                println!("Error creating shader: {e:?}");
                return Err(e);
            };
            let shader = shader.unwrap();

            use gl_graphics::get_uniform_location;
            let uid_model = get_uniform_location(&gl, shader, "model").unwrap_or(-1);
            let uid_camera = get_uniform_location(&gl, shader, "camera").unwrap_or(-1);
            let uid_from_tex = get_uniform_location(&gl, shader, "from_tex").unwrap_or(-1);
            let uid_to_tex = get_uniform_location(&gl, shader, "to_tex").unwrap_or(-1);
            let uid_progress = get_uniform_location(&gl, shader, "progress").unwrap_or(-1);
            let uid_from_pos = get_uniform_location(&gl, shader, "from_pos").unwrap_or(-1);
            let uid_from_size = get_uniform_location(&gl, shader, "from_size").unwrap_or(-1);
            let uid_to_pos = get_uniform_location(&gl, shader, "to_pos").unwrap_or(-1);
            let uid_to_size = get_uniform_location(&gl, shader, "to_size").unwrap_or(-1);

            Ok(Transition {
                gl,
                shader,
                uid_model,
                uid_camera,
                uid_from_tex,
                uid_to_tex,
                uid_progress,
                uid_from_pos,
                uid_from_size,
                uid_to_pos,
                uid_to_size,
            })
        }
    }

    // ----------------------------------------------------------------------------
    impl GlTransition for Transition {
        fn render(
            &self,
            bindings: &GlMesh,
            from: &GlMaterial,
            to: &GlMaterial,
            unis: &GlUniforms,
        ) -> Result<()> {
            let gl = &self.gl;
            let from_tex = if let GlMaterial::Texture(id) = from {
                *id
            } else {
                0
            };
            let to_tex = if let GlMaterial::Texture(id) = to {
                *id
            } else {
                0
            };
            unsafe {
                gl.UseProgram(self.shader);
                gl.BindVertexArray(bindings.vao);
                gl.UniformMatrix4fv(self.uid_model, 1, gl::FALSE, unis.model.as_ptr());
                gl.UniformMatrix4fv(self.uid_camera, 1, gl::FALSE, unis.camera.as_ptr());
                gl.Uniform1i(self.uid_from_tex, 0);
                gl.Uniform1i(self.uid_to_tex, 1);
                gl.Uniform1f(self.uid_progress, unis.progress);
                gl.Uniform2f(self.uid_from_pos, unis.from_pos.x0(), unis.from_pos.x1());
                gl.Uniform2f(self.uid_from_size, unis.from_size.x0(), unis.from_size.x1());
                gl.Uniform2f(self.uid_to_pos, unis.to_pos.x0(), unis.to_pos.x1());
                gl.Uniform2f(self.uid_to_size, unis.to_size.x0(), unis.to_size.x1());
                gl.ActiveTexture(gl::TEXTURE0);
                gl.BindTexture(gl::TEXTURE_2D, from_tex);
                gl.ActiveTexture(gl::TEXTURE1);
                gl.BindTexture(gl::TEXTURE_2D, to_tex);
                gl.DrawArrays(gl::TRIANGLE_STRIP, 0, bindings.count as gl::GLint);
            }
            Ok(())
        }
    }

    // ----------------------------------------------------------------------------
    impl Drop for Transition {
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
    uniform vec2 from_pos;
    uniform vec2 from_size;
    uniform vec2 to_pos;
    uniform vec2 to_size;

    layout (location = 0) in vec2 a_pos;
    layout (location = 1) in vec2 a_tex;

    out vec2 v_tex0;
    out vec2 v_tex1;

    void main() {
        gl_Position = camera * model * vec4(a_pos, 0.0, 1.0);
        v_tex0 = (a_tex - from_pos) / from_size;
        v_tex1 = (a_tex - to_pos) / to_size;
    }"#;

    // ----------------------------------------------------------------------------
    const FS_TEXTURE: &str = r#"
    #version 300 es
    uniform sampler2D from_tex;
    uniform sampler2D to_tex;
    uniform mediump float progress;

    in mediump vec2 v_tex0;
    in mediump vec2 v_tex1;
    out mediump vec4 FragColor;

    void main() {
        mediump vec3 from_yuv;
        if (v_tex0.x >= 0.0 && v_tex0.x <= 1.0 &&
            v_tex0.y >= 0.0 && v_tex0.y <= 1.0) {
            from_yuv = texture(from_tex, v_tex0.st).rgb - vec3(0.0, 0.5, 0.5);
        } else {
            from_yuv = vec3(0.1, 0.0, 0.0);
        }

        mediump vec3 to_yuv;
        if (v_tex1.x >= 0.0 && v_tex1.x <= 1.0 &&
            v_tex1.y >= 0.0 && v_tex1.y <= 1.0) {
            to_yuv = texture(to_tex, v_tex1.st).rgb - vec3(0.0, 0.5, 0.5);
        } else {
            to_yuv = vec3(0.1, 0.0, 0.0);
        }

        mediump vec3 yuv = mix(from_yuv, to_yuv, progress);

        mediump vec3 rgb;
        rgb.r = yuv.x + 1.402 * yuv.z;
        rgb.g = yuv.x - 0.344 * yuv.y - 0.714 * yuv.z;
        rgb.b = yuv.x + 1.772 * yuv.y;
        FragColor = vec4(rgb, 1.0);
    }"#;
}
