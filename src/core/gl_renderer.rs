use crate::core::gl_canvas::Canvas;
use crate::core::gl_graphics::{
    create_framebuffer, create_program, create_texture_vao, print_opengl_info,
};
use crate::core::gl_pipeline::{self, GlUniforms, msdf_tex, v_pos_tex, v_yuv_tex, yuv_dual};
use crate::error::Result;
use crate::gl::opengl as gl;
use crate::v2d::{affine4x4, m4x4::M4x4, v2::V2};
use std::rc::Rc;

// --------------------------------------------------------------------------------
const VS_TEXTURE: &str = r#"
#version 300 es
layout (location = 0) in vec2 aPosition;
layout (location = 1) in vec2 aTexCoord;
out mediump vec2 TexCoord;
void main() {
    gl_Position = vec4(aPosition, 0.0, 1.0);
    TexCoord = aTexCoord;
}"#;

// --------------------------------------------------------------------------------
const FS_TEXTURE: &str = r#"
#version 300 es
in mediump vec2 TexCoord;
out mediump vec4 FragColor;
uniform mediump sampler2D screen;

void main() {
    FragColor = texture(screen, TexCoord.st);
}"#;

// --------------------------------------------------------------------------------
pub struct Renderer {
    gl: Rc<gl::OpenGlFunctions>,
    pipelines: Vec<Box<dyn gl_pipeline::GlPipeline>>,
    transition_pipelines: Vec<Box<dyn gl_pipeline::GlTransition>>,
    texture_vao: gl::GLuint,
    texture_program: gl::GLuint,
    fbo: gl::GLuint,
    color_tex: gl::GLuint,
    depth_tex: gl::GLuint,
}

impl Renderer {
    // ----------------------------------------------------------------------------
    pub fn new(gl: Rc<gl::OpenGlFunctions>, cx: usize, cy: usize) -> Result<Self> {
        print_opengl_info(&gl);

        let texture_vao = create_texture_vao(&gl);
        let texture_program = create_program(&gl, "texture", VS_TEXTURE, FS_TEXTURE)?;
        let (fbo, color_tex, depth_tex) = create_framebuffer(&gl, cx, cy)?;

        let rgb_pipe = Box::new(v_pos_tex::Pipeline::new(Rc::clone(&gl))?);
        let yuv_pipe = Box::new(v_yuv_tex::Pipeline::new(Rc::clone(&gl))?);
        let msdf_pipe = Box::new(msdf_tex::Pipeline::new(Rc::clone(&gl))?);
        let dual_pipe = Box::new(yuv_dual::Transition::new(Rc::clone(&gl))?);

        Ok(Self {
            gl,
            pipelines: vec![rgb_pipe, yuv_pipe, msdf_pipe],
            transition_pipelines: vec![dual_pipe],
            texture_vao,
            texture_program,
            fbo,
            color_tex,
            depth_tex,
        })
    }

    // ----------------------------------------------------------------------------
    fn render_1st_pass(&self, canvas: &Canvas) -> Result<()> {
        let gl = &self.gl;

        let camera = canvas.camera();
        let zoom = camera.zoom();
        let camera = affine4x4::ortho2d(1.0, zoom);

        unsafe {
            gl.BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl.Disable(gl::DEPTH_TEST);
            gl.Disable(gl::CULL_FACE);
            gl.Disable(gl::BLEND);
            gl.ClearColor(0.1, 0.1, 0.1, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let mut uniforms = GlUniforms {
            model: M4x4::identity(),
            camera,
            mat_id: 0,
            progress: 0.0,
            from_pos: V2::zero(),
            from_size: V2::zero(),
            to_pos: V2::zero(),
            to_size: V2::zero(),
        };

        uniforms.model = M4x4::identity();
        uniforms.mat_id = 0;
        for transition in canvas.transitions() {
            uniforms.from_pos = transition.from_pos;
            uniforms.from_size = transition.from_size;
            uniforms.to_pos = transition.to_pos;
            uniforms.to_size = transition.to_size;
            uniforms.progress = transition.progress;
            let mesh = canvas.mesh(transition.mesh_id);
            let pipe = self.transition_pipelines.get(transition.pipeline_id);
            let from = canvas.materials().get(transition.from_id);
            let to = canvas.materials().get(transition.to_id);
            match (mesh, pipe, from, to) {
                (Some(mesh), Some(pipe), Some(from), Some(to)) => {
                    pipe.render(mesh, from, to, &uniforms)?;
                }
                _ => {
                    continue;
                }
            }
        }

        for obj in canvas.objects() {
            let mesh = canvas.mesh(obj.mesh_id);
            let pipe = self.pipelines.get(obj.pipeline_id);
            let material = canvas.materials().get(obj.material_id);
            match (mesh, pipe, material) {
                (Some(mesh), Some(pipe), Some(material)) => {
                    uniforms.model = obj.transform;
                    uniforms.mat_id = obj.material_id as gl::GLint;
                    pipe.render(mesh, material, &uniforms)?;
                }
                _ => {
                    continue;
                }
            }
        }

        Ok(())
    }

    // ----------------------------------------------------------------------------
    fn render_2nd_pass(&self) -> Result<()> {
        let gl = &self.gl;
        unsafe {
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl.Disable(gl::DEPTH_TEST);

            gl.UseProgram(self.texture_program);
            gl.BindVertexArray(self.texture_vao);
            gl.ActiveTexture(gl::TEXTURE0);
            gl.BindTexture(gl::TEXTURE_2D, self.color_tex);
            gl.DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
        Ok(())
    }

    // ----------------------------------------------------------------------------
    pub fn render(&self, canvas: &Canvas) -> Result<()> {
        self.render_1st_pass(canvas)?;
        self.render_2nd_pass()?;
        Ok(())
    }

    // ----------------------------------------------------------------------------
    pub fn resize(&self, cx: i32, cy: i32) {
        println!("Resize to {cx} x {cy}");
        unsafe { self.gl.Viewport(0, 0, cx, cy) };
    }
}
