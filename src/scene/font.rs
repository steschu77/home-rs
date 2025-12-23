use crate::error::{Error, Result};
use miniz::png_read;
use serde::Deserialize;

#[derive(Clone)]
pub struct Font {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
    pub meta: FontMeta,
    pub glyphs: FontGlyphs,
}

#[derive(Debug, Clone)]
pub struct FontMeta {
    pub line_height: f32,
}

#[derive(Debug, Clone)]
pub struct FontGlyph {
    pub uv: [f32; 4],
    pub xy: [f32; 4],
    pub advance: f32,
}

type FontGlyphs = std::collections::HashMap<u32, FontGlyph>;

impl FontGlyph {
    fn new(glyph: &JsonGlyph, size: (f32, f32)) -> Self {
        let uv = if let Some(b) = &glyph.atlas_bounds {
            [
                size.0 * b.left,
                size.1 * b.bottom,
                size.0 * b.right,
                size.1 * b.top,
            ]
        } else {
            [0.0, 0.0, 0.0, 0.0]
        };
        let xy = if let Some(bounds) = &glyph.plane_bounds {
            [bounds.left, bounds.bottom, bounds.right, bounds.top]
        } else {
            [0.0, 0.0, 0.0, 0.0]
        };
        Self {
            uv,
            xy,
            advance: glyph.advance,
        }
    }
}

impl Font {
    pub fn load(path: &std::path::Path) -> Result<Self> {
        let png_path = path.with_extension("png");
        let (width, height, data) = load_png(&png_path)?;

        let size = (1.0 / width as f32, 1.0 / height as f32);
        let json_path = path.with_extension("json");
        let (meta, glyphs) = load_json(&json_path, size)?;

        Ok(Self {
            width,
            height,
            data,
            meta,
            glyphs,
        })
    }
}

fn load_png(path: &std::path::Path) -> Result<(usize, usize, Vec<u8>)> {
    let contents = std::fs::read(path)?;
    let (png, _plte, data) = png_read::png_read(&contents)?;

    if png.color_type != png_read::PNGColorType::TrueColorAlpha {
        return Err(Error::InvalidColorFormat);
    }

    let tx_width = (png.width + 3) & !3;
    let tx_height = png.height;

    let mut aligned = vec![0u8; tx_width * tx_height * 4];
    for y in 0..png.height {
        let src_offset = y * (png.width * 4 + 1) + 1;
        let dst_offset = y * tx_width * 4;
        aligned[dst_offset..(dst_offset + png.width * 4)]
            .copy_from_slice(&data[src_offset..(src_offset + png.width * 4)]);
    }

    Ok((tx_width, tx_height, aligned))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonGlyphAtlas {
    pub metrics: JsonMetrics,
    pub glyphs: Vec<JsonGlyph>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonMetrics {
    pub line_height: f32,
    pub ascender: f32,
    pub descender: f32,
    pub underline_y: f32,
    pub underline_thickness: f32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonBounds {
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonGlyph {
    pub unicode: u32,
    pub advance: f32,
    pub plane_bounds: Option<JsonBounds>,
    pub atlas_bounds: Option<JsonBounds>,
}

fn load_json(path: &std::path::Path, size: (f32, f32)) -> Result<(FontMeta, FontGlyphs)> {
    let contents = std::fs::read_to_string(path)?;
    let atlas = serde_json::from_str::<JsonGlyphAtlas>(&contents)?;

    let mut glyphs = FontGlyphs::new();
    for glyph in atlas.glyphs.iter() {
        let g = FontGlyph::new(glyph, size);
        glyphs.insert(glyph.unicode, g);
    }

    let meta = FontMeta {
        line_height: atlas.metrics.line_height,
    };

    Ok((meta, glyphs))
}
