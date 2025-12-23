#![allow(dead_code)]
use crate::gfx::color_format::ColorFormat;

// ----------------------------------------------------------------------------
pub struct ImageGeometry {
    pub cx: usize,
    pub cy: usize,
    pub cf: ColorFormat,
}

// ----------------------------------------------------------------------------
pub struct ImagePal {
    pub data: Vec<u8>,
    pub stride: usize,
    pub palette: Vec<u32>,
}

// ----------------------------------------------------------------------------
pub struct ImageRgb32 {
    pub data: Vec<u32>,
    pub stride: usize,
}

// ----------------------------------------------------------------------------
pub fn make_buffersize(stride: usize, cy: usize) -> usize {
    stride * cy
}

// ----------------------------------------------------------------------------
pub fn pal1_to_rgb32(pal1: ImagePal, geo: &ImageGeometry) -> ImageRgb32 {
    let mut rgb32 = ImageRgb32 {
        data: vec![0; geo.cx * geo.cy],
        stride: geo.cx,
    };

    for y in 0..geo.cy {
        let src = &pal1.data[y * pal1.stride..(y + 1) * pal1.stride];
        let dst = &mut rgb32.data[y * rgb32.stride..(y + 1) * rgb32.stride];

        for x in 0..geo.cx {
            let idx = (src[x / 8] >> (7 - (x & 7))) & 1;
            dst[x] = pal1.palette[idx as usize];
        }
    }

    rgb32
}

// ----------------------------------------------------------------------------
pub fn pal8_to_rgb32(pal8: ImagePal, geo: &ImageGeometry) -> ImageRgb32 {
    let mut rgb32 = ImageRgb32 {
        data: vec![0; geo.cx * geo.cy],
        stride: geo.cx,
    };

    for y in 0..geo.cy {
        let src = &pal8.data[y * pal8.stride..(y + 1) * pal8.stride];
        let dst = &mut rgb32.data[y * rgb32.stride..(y + 1) * rgb32.stride];

        for x in 0..geo.cx {
            let idx = src[x];
            dst[x] = pal8.palette[idx as usize];
        }
    }

    rgb32
}
