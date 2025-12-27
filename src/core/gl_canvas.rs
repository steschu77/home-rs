use crate::core::camera::Camera;
use crate::core::gl_graphics;
use crate::error::Result;
use crate::gl::opengl::{self as gl};
use crate::v2d::{m4x4::M4x4, v2::V2};
use std::rc::Rc;

// ----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct GlObject {
    pub mesh_id: usize,
    pub pipeline_id: usize,
    pub material_id: usize,
    pub transform: M4x4,
}

// ----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct GlTransition {
    pub mesh_id: usize,
    pub pipeline_id: usize,
    pub from_id: usize,
    pub to_id: usize,
    pub progress: f32,
    pub from_pos: V2,
    pub from_size: V2,
    pub to_pos: V2,
    pub to_size: V2,
}

// ----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub enum GlMaterial {
    Color([f32; 4]),
    Texture(gl::GLuint),
    YuvTexture(gl::GLuint, gl::GLuint, gl::GLuint),
}

// ----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: V2,
    pub tex: V2,
}

// --------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct GlMesh {
    pub vao: gl::GLuint,
    pub vbo: gl::GLuint,
    pub count: usize,
}

// ------------------------------------------------------------------------
pub fn create_mesh(gl: &gl::OpenGlFunctions, vertices: &[Vertex]) -> Result<GlMesh> {
    let vao = gl_graphics::create_vertex_array(gl);
    let vbo = unsafe {
        gl_graphics::create_buffer(
            gl,
            gl::ARRAY_BUFFER,
            vertices.as_ptr() as *const _,
            std::mem::size_of_val(vertices),
        )
    };

    let stride = std::mem::size_of::<Vertex>() as gl::GLint;
    let pos_ofs = std::mem::offset_of!(Vertex, pos) as gl::GLint;
    let tex_ofs = std::mem::offset_of!(Vertex, tex) as gl::GLint;

    unsafe {
        gl.EnableVertexAttribArray(0); // position
        gl.VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, pos_ofs as *const _);
        gl.EnableVertexAttribArray(1); // texture
        gl.VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, tex_ofs as *const _);
    }

    Ok(GlMesh {
        vao,
        vbo,
        count: vertices.len(),
    })
}

// --------------------------------------------------------------------------------
pub fn delete_mesh(gl: &gl::OpenGlFunctions, mesh: &GlMesh) {
    gl_graphics::delete_vertex_array(gl, mesh.vao);
    gl_graphics::delete_buffer(gl, mesh.vbo);
}

// ----------------------------------------------------------------------------
#[derive(Clone)]
pub struct Canvas {
    gl: Rc<gl::OpenGlFunctions>,
    aspect_ratio: f32,
    camera: Camera,
    objects: Vec<GlObject>,
    transitions: Vec<GlTransition>,
    materials: Vec<GlMaterial>,
    meshes: Vec<GlMesh>,
}

// ----------------------------------------------------------------------------
impl Canvas {
    pub fn new(gl: Rc<gl::OpenGlFunctions>, aspect_ratio: f32) -> Result<Self> {
        Ok(Self {
            gl,
            aspect_ratio,
            camera: Camera::default(),
            objects: Vec::new(),
            transitions: Vec::new(),
            materials: Vec::new(),
            meshes: Vec::new(),
        })
    }

    // ------------------------------------------------------------------------
    pub fn create_texture(
        &mut self,
        width: usize,
        height: usize,
        format: usize,
        data: &[u8],
    ) -> Result<GlMaterial> {
        let id = gl_graphics::create_texture(
            &self.gl,
            width,
            height,
            format,
            data,
            gl::LINEAR,
            gl::CLAMP_TO_EDGE,
        )?;
        Ok(GlMaterial::Texture(id))
    }

    // ------------------------------------------------------------------------
    pub fn create_yuv_texture(
        &mut self,
        width: usize,
        height: usize,
        format: usize,
        luma: &[u8],
        cb: &[u8],
        cr: &[u8],
    ) -> Result<GlMaterial> {
        let filter = gl::LINEAR;
        let wrap = gl::CLAMP_TO_EDGE;
        let id_luma =
            gl_graphics::create_texture(&self.gl, width, height, format, luma, filter, wrap)?;
        let id_cb =
            gl_graphics::create_texture(&self.gl, width / 2, height / 2, format, cb, filter, wrap)?;
        let id_cr =
            gl_graphics::create_texture(&self.gl, width / 2, height / 2, format, cr, filter, wrap)?;

        Ok(GlMaterial::YuvTexture(id_luma, id_cb, id_cr))
    }

    // ------------------------------------------------------------------------
    pub fn delete_material(&mut self, material: &GlMaterial) {
        match material {
            GlMaterial::Texture(id) => {
                gl_graphics::delete_texture(&self.gl, *id);
            }
            GlMaterial::YuvTexture(id_luma, id_cb, id_cr) => {
                gl_graphics::delete_texture(&self.gl, *id_luma);
                gl_graphics::delete_texture(&self.gl, *id_cb);
                gl_graphics::delete_texture(&self.gl, *id_cr);
            }
            _ => {}
        }
    }

    // ------------------------------------------------------------------------
    pub fn create_mesh(&mut self, verts: &[Vertex]) -> Result<GlMesh> {
        create_mesh(&self.gl, verts)
    }

    // ------------------------------------------------------------------------
    pub fn delete_mesh(&mut self, mesh: &GlMesh) {
        delete_mesh(&self.gl, mesh);
    }

    pub fn update(
        &mut self,
        objects: Vec<GlObject>,
        transitions: Vec<GlTransition>,
        materials: Vec<GlMaterial>,
        meshes: Vec<GlMesh>,
    ) {
        self.objects = objects;
        self.transitions = transitions;
        self.materials = materials;
        self.meshes = meshes;
    }

    pub fn resize(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn objects(&self) -> &[GlObject] {
        &self.objects
    }

    pub fn transitions(&self) -> &[GlTransition] {
        &self.transitions
    }

    pub fn materials(&self) -> &[GlMaterial] {
        &self.materials
    }

    pub fn mesh(&self, mesh_id: usize) -> Option<&GlMesh> {
        self.meshes.get(mesh_id)
    }
}
