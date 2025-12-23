use crate::core::gl_canvas::{Canvas, GlMaterial, GlMesh, GlObject, Vertex};
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
    resources: Vec<Handle>,
    font: Font,
    current: Layout,
    objects: Vec<GlObject>,
    materials: Vec<Option<GlMaterial>>,
    free_ids: Vec<usize>,
    meshes: Vec<Option<GlMesh>>,
    free_mesh_ids: Vec<usize>,
}

// ----------------------------------------------------------------------------
#[derive(Clone)]
pub struct LayoutItem {
    pub object_id: usize,
    pub material_id: usize,
}

impl Layouter {
    pub fn new(canvas: Canvas) -> Result<Self> {
        let mut canvas = canvas;
        let font = Font::load(std::path::Path::new("assets/fonts/roboto.png"))?;
        let material = canvas.create_texture(font.width, font.height, 0, &font.data);

        let verts = create_plane_mesh();
        let quad = canvas.create_mesh(&verts)?;

        Ok(Self {
            canvas,
            resources: Vec::new(),
            font,
            current: Layout::empty(),
            objects: Vec::new(),
            materials: vec![Some(material)],
            free_ids: Vec::new(),
            meshes: vec![Some(quad)],
            free_mesh_ids: Vec::new(),
        })
    }

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

        let id = self.insert_material(material);

        log::info!(
            "Loaded photo {:?} as texture {} ({}x{})",
            photo.path,
            id,
            tx_width,
            tx_height
        );

        Ok(Handle {
            id,
            aspect_ratio: tx_width as f32 / tx_height as f32,
        })
    }

    pub fn free_handle(&mut self, handle: Handle) {
        if let Some(material) = &self.materials[handle.id] {
            self.canvas.delete_material(material);
            self.materials[handle.id] = None;
            self.free_ids.push(handle.id);
        }
    }

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
        let id = self.insert_mesh(mesh.clone());

        log::info!(
            "Created text mesh '{}' as id {id}, vao/vbo {}/{} ({} vertices)",
            text,
            mesh.vao,
            mesh.vbo,
            verts.len()
        );

        Ok(Handle {
            id,
            aspect_ratio: 0.0,
        })
    }

    pub fn free_text(&mut self, handle: Handle) {
        if let Some(mesh) = &self.meshes[handle.id] {
            self.canvas.delete_mesh(mesh);
            self.meshes[handle.id] = None;
            self.free_mesh_ids.push(handle.id);
        }
    }

    pub fn update_layout(&mut self, layout: &Layout) {
        let mut objects = Vec::new();
        // Keep font material at index 0
        let mut materials = vec![self.materials[0].clone().unwrap()];
        let meshes = self
            .meshes
            .iter()
            .filter_map(|m| m.clone())
            .collect::<Vec<GlMesh>>();

        log::info!("Updating layout with {:?} items", layout.items);

        for item in &layout.items {
            match &item.element {
                Element::Picture(picture) => {
                    if let Some(material) = &self.materials[picture.handle.id] {
                        let material_id = materials.len();
                        materials.push(material.clone());

                        let object = GlObject {
                            mesh_id: 0,     // Use a plane mesh
                            pipeline_id: 1, // Use YUV pipeline
                            material_id,
                            transform: photo::transform(&picture.dst),
                        };
                        objects.push(object);
                    }
                }
                Element::Text(text) => {
                    let object = GlObject {
                        mesh_id: text.handle.id,
                        pipeline_id: 2, // Use MSDF pipeline
                        material_id: 0, // Use font material
                        transform: photo::transform(&text.dst),
                    };
                    objects.push(object);
                }
                _ => {} // Unsupported element types
            }
        }

        self.canvas.update_objects(objects);
        self.canvas.update_materials(materials);
        self.canvas.update_meshes(meshes);
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
        if let Some(id) = self.free_ids.pop() {
            assert!(id < self.materials.len());
            assert!(self.materials[id].is_none());
            self.materials[id] = Some(material);
            id
        } else {
            self.materials.push(Some(material));
            self.materials.len() - 1
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
