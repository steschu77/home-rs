use crate::core::gl_canvas::{Canvas, GlMaterial, GlMesh, GlObject, Vertex};
use crate::core::gl_pipeline::GlPipelineType;
use crate::error::Result;
use crate::scene::photo;
use crate::scene::{
    Element, Handle, Layout, Photo,
    font::{Font, FontGlyph},
};
use crate::util::utf8::next_code_point;
use crate::v2d::v2::V2;

// ----------------------------------------------------------------------------
pub struct Layouter {
    canvas: Canvas,
    font: Font,
    materials: Vec<Option<GlMaterial>>,
    meshes: Vec<Option<GlMesh>>,
    free_material_ids: Vec<usize>,
    free_mesh_ids: Vec<usize>,
    font_texture: GlMaterial,
    quad_mesh: GlMesh,
}

impl Layouter {
    // ------------------------------------------------------------------------
    pub fn new(canvas: Canvas) -> Result<Self> {
        let mut canvas = canvas;
        let font = Font::load(std::path::Path::new("assets/fonts/roboto.png"))?;
        let font_texture = canvas.create_texture(font.width, font.height, 0, &font.data);

        let verts = create_plane_mesh();
        let quad_mesh = canvas.create_mesh(&verts)?;

        Ok(Self {
            canvas,
            font,
            materials: Vec::new(),
            meshes: Vec::new(),
            free_material_ids: Vec::new(),
            free_mesh_ids: Vec::new(),
            font_texture,
            quad_mesh,
        })
    }

    // ------------------------------------------------------------------------
    pub fn load_photo(&mut self, photo: &Photo) -> Result<Handle> {
        let contents = std::fs::read(&photo.path)?;
        let frame = miniwebp::read_image(&contents)?;

        let tx_width = frame.mb_width * 16;
        let tx_height = frame.mb_height * 16;
        let material = self.canvas.create_yuv_texture(
            tx_width,
            tx_height,
            2,
            &frame.ybuf,
            &frame.ubuf,
            &frame.vbuf,
        );

        let material_id = self.insert_material(material);

        log::info!(
            "Loaded photo {:?} as texture {material_id} ({}x{})",
            photo.path,
            tx_width,
            tx_height
        );

        Ok(Handle {
            material_id: Some(material_id),
            mesh_id: None,
            aspect_ratio: tx_width as f32 / tx_height as f32,
        })
    }

    // ------------------------------------------------------------------------
    pub fn free_handle(&mut self, handle: Handle) {
        if let Some(id) = handle.material_id
            && let Some(material) = self.materials.get(id).and_then(|m| m.as_ref())
        {
            self.canvas.delete_material(material);
            self.materials[id] = None;
            self.free_material_ids.push(id);
        }

        if let Some(id) = handle.mesh_id
            && let Some(mesh) = self.meshes.get(id).and_then(|m| m.as_ref())
        {
            self.canvas.delete_mesh(mesh);
            self.meshes[id] = None;
            self.free_mesh_ids.push(id);
        }
    }

    // ------------------------------------------------------------------------
    pub fn create_text(&mut self, text: &str) -> Result<Handle> {
        let mut iter = text.as_bytes().iter();
        let mut pos = V2::new([0.0, 0.0]);
        let mut verts = Vec::new();
        while let Some(ch) = next_code_point(&mut iter) {
            if let Some(glyph) = self.font.glyphs.get(&ch) {
                Self::add_glyph(glyph, &mut pos, &mut verts);
            }
        }

        let mesh = self.canvas.create_mesh(&verts)?;
        let mesh_id = self.insert_mesh(mesh.clone());

        log::info!(
            "Created text mesh '{}' as id {mesh_id}, vao/vbo {}/{} ({} vertices)",
            text,
            mesh.vao,
            mesh.vbo,
            verts.len()
        );

        Ok(Handle {
            material_id: None,
            mesh_id: Some(mesh_id),
            aspect_ratio: 0.0,
        })
    }

    // ------------------------------------------------------------------------
    pub fn update_layout(&mut self, layout: &Layout) {
        let mut objects = Vec::new();

        let mut materials = vec![self.font_texture.clone()];
        let font_material_id = 0;

        let mut meshes = vec![self.quad_mesh.clone()];
        let quad_mesh_id = 0;

        for item in &layout.items {
            match &item.element {
                Element::Picture(picture) => {
                    if let Some(material) = self.get_material(&picture.handle) {
                        let material_id = materials.len();
                        materials.push(material.clone());

                        let object = GlObject {
                            mesh_id: quad_mesh_id,
                            pipeline_id: GlPipelineType::YUVTex.into(),
                            material_id,
                            transform: photo::transform(&picture.dst),
                        };
                        objects.push(object);
                    }
                }
                Element::Text(text) => {
                    if let Some(mesh) = self.get_mesh(&text.handle) {
                        let mesh_id = meshes.len();
                        meshes.push(mesh.clone());
                        let object = GlObject {
                            mesh_id,
                            pipeline_id: GlPipelineType::MSDFTex.into(),
                            material_id: font_material_id,
                            transform: photo::transform(&text.dst),
                        };
                        objects.push(object);
                    }
                }
                _ => {} // Unsupported element types
            }
        }

        self.canvas.update(objects, materials, meshes);
    }

    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.canvas.aspect_ratio()
    }

    pub fn resize(&mut self, aspect_ratio: f32) {
        self.canvas.resize(aspect_ratio);
    }

    fn insert_material(&mut self, material: GlMaterial) -> usize {
        if let Some(id) = self.free_material_ids.pop() {
            assert!(id < self.materials.len());
            assert!(self.materials[id].is_none());
            self.materials[id] = Some(material);
            id
        } else {
            self.materials.push(Some(material));
            self.materials.len() - 1
        }
    }

    fn get_material(&self, handle: &Handle) -> Option<&GlMaterial> {
        if let Some(material_id) = handle.material_id {
            self.materials.get(material_id).and_then(|m| m.as_ref())
        } else {
            None
        }
    }

    fn insert_mesh(&mut self, mesh: GlMesh) -> usize {
        if let Some(id) = self.free_mesh_ids.pop() {
            assert!(id < self.meshes.len());
            assert!(self.meshes[id].is_none());
            self.meshes[id] = Some(mesh);
            id
        } else {
            self.meshes.push(Some(mesh));
            self.meshes.len() - 1
        }
    }

    fn get_mesh(&self, handle: &Handle) -> Option<&GlMesh> {
        if let Some(mesh_id) = handle.mesh_id {
            self.meshes.get(mesh_id).and_then(|m| m.as_ref())
        } else {
            None
        }
    }

    fn add_glyph(glyph: &FontGlyph, pos: &mut V2, verts: &mut Vec<Vertex>) {
        let uv_u = glyph.uv[0];
        let uv_v = 1.0 - glyph.uv[3];
        let uv_width = glyph.uv[2] - glyph.uv[0];
        let uv_height = glyph.uv[3] - glyph.uv[1];
        let uv_pos = V2::new([uv_u, uv_v]);
        let uv_size = V2::new([uv_width, uv_height]);

        let xy_x = glyph.xy[0];
        let xy_y = glyph.xy[1];
        let xy_width = glyph.xy[2] - glyph.xy[0];
        let xy_height = glyph.xy[3] - glyph.xy[1];
        let xy = *pos + V2::new([xy_x, xy_y]);
        let xy_size = V2::new([xy_width, xy_height]);

        add_plane_quad(
            verts,
            uv_pos,
            uv_size.x0(),
            uv_size.x1(),
            xy,
            xy_size.x0(),
            xy_size.x1(),
        );

        *pos += V2::new([glyph.advance, 0.0]);
    }
}

// --------------------------------------------------------------------------------
fn add_plane_quad(verts: &mut Vec<Vertex>, uv: V2, u: f32, v: f32, xy: V2, x: f32, y: f32) {
    #[rustfmt::skip]
    verts.extend_from_slice(&[
        Vertex { pos: xy + V2::new([0.0, 0.0]), tex: uv + V2::new([0.0,   v]) },
        Vertex { pos: xy + V2::new([  x, 0.0]), tex: uv + V2::new([  u,   v]) },
        Vertex { pos: xy + V2::new([0.0,   y]), tex: uv + V2::new([0.0, 0.0]) },
        Vertex { pos: xy + V2::new([0.0,   y]), tex: uv + V2::new([0.0, 0.0]) },
        Vertex { pos: xy + V2::new([  x, 0.0]), tex: uv + V2::new([  u,   v]) },
        Vertex { pos: xy + V2::new([  x,   y]), tex: uv + V2::new([  u, 0.0]) },
    ]);
}

// --------------------------------------------------------------------------------
#[rustfmt::skip]
fn create_plane_mesh() -> Vec<Vertex> {
    vec![
        Vertex { pos: V2::new([0.0, 0.0]), tex: V2::new([0.0, 1.0]) },
        Vertex { pos: V2::new([1.0, 0.0]), tex: V2::new([1.0, 1.0]) },
        Vertex { pos: V2::new([0.0, 1.0]), tex: V2::new([0.0, 0.0]) },
        Vertex { pos: V2::new([1.0, 1.0]), tex: V2::new([1.0, 0.0]) },
    ]
}
