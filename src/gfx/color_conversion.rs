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

pub fn ycbcr420_to_ycbcr24(luma: &[u8], cb: &[u8], cr: &[u8], geo: &ImageGeometry) -> Vec<u8> {
    let mut yuv24 = vec![0; geo.cx * geo.cy * 3];
    let chroma_width = geo.cx.div_ceil(2);
    let chroma_height = geo.cy.div_ceil(2);

    for y in 0..geo.cy {
        let src_luma = &luma[y * geo.cx..(y + 1) * geo.cx];
        let chroma_y = (y / 2).min(chroma_height - 1);
        let src_cb = &cb[chroma_y * chroma_width..(chroma_y + 1) * chroma_width];
        let src_cr = &cr[chroma_y * chroma_width..(chroma_y + 1) * chroma_width];
        let dst = &mut yuv24[y * geo.cx * 3..(y + 1) * geo.cx * 3];

        for x in 0..geo.cx {
            let chroma_x = (x / 2).min(chroma_width - 1);
            let y_val = src_luma[x];
            let cb_val = src_cb[chroma_x];
            let cr_val = src_cr[chroma_x];

            dst[x * 3] = y_val;
            dst[x * 3 + 1] = cb_val;
            dst[x * 3 + 2] = cr_val;
        }
    }
    yuv24
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ycbcr420_to_ycbcr24() {
        #[rustfmt::skip]
        let luma = vec![
             10,  20,  30,  40,
             50,  60,  70,  80,
             90, 100, 110, 120,
            130, 140, 150, 160,
        ];
        let cb = vec![1, 2, 3, 4];
        let cr = vec![5, 6, 7, 8];

        let geo = ImageGeometry {
            cx: 4,
            cy: 4,
            cf: ColorFormat::YCbCr420,
        };

        let result = ycbcr420_to_ycbcr24(&luma, &cb, &cr, &geo);

        #[rustfmt::skip]
        let expected = vec![
             10, 1, 5,   20, 1, 5,   30, 2, 6,   40, 2, 6,
             50, 1, 5,   60, 1, 5,   70, 2, 6,   80, 2, 6,
             90, 3, 7,  100, 3, 7,  110, 4, 8,  120, 4, 8,
            130, 3, 7,  140, 3, 7,  150, 4, 8,  160, 4, 8,
        ];
        assert_eq!(result, expected);
    }
}
