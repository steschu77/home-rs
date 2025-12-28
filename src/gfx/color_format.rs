#![allow(dead_code)]

// ----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorFormat {
    Y1,  // 1 bit Grayscale
    Y2,  // 2 bit Grayscale
    Y4,  // 4 bit Grayscale
    Y8,  // 8 bit Grayscale
    Y16, // 16 bit Greyscale

    YA8,  // 8 bit Grayscale with 8 bit alpha
    YA16, // 16 bit Grayscale with 16 bit alpha

    Pal1, // 1 bit paletted
    Pal2, // 2 bit paletted
    Pal4, // 4 bit paletted
    Pal8, // 8 bit paletted

    RGB4444, // 16 bit RGB with 4 bit for alpha, red, green and blue
    RGB0555, // 16 bit RGB with 5 bit for red, green and blue
    RGB0565, // 16 bit RGB with 5 bit for red and blue and 6 bit for green
    RGB1555, // 16 bit RGB with 5 bit for red, green and blue and 1 bit for alpha
    RGB0888, // 24 bit RGB with 8 bit for red, green and blue
    RGB8888, // 32 bit RGBA with 8 bit for alpha, red, green and blue
    BGR0888, // 24 bit RGB reversed with 8 bit for red, green and blue
    BGR8888, // 32 bit RGBA reversed with 8 bit for alpha, red, green and blue
    RGB0ggg, // 48 bit RGB with 16 bit for red, green and blue
    RGBgggg, // 64 bit RGBA with 16 bit for red, green and blue

    YCbCr420, // 12 bit YCbCr 4:2:0
}

// ----------------------------------------------------------------------------
impl ColorFormat {
    // ------------------------------------------------------------------------
    pub fn is_indexed(&self) -> bool {
        matches!(
            self,
            ColorFormat::Pal1 | ColorFormat::Pal2 | ColorFormat::Pal4 | ColorFormat::Pal8
        )
    }

    // ------------------------------------------------------------------------
    pub fn bpp(&self) -> usize {
        match self {
            ColorFormat::Y1 | ColorFormat::Pal1 => 1,
            ColorFormat::Y2 | ColorFormat::Pal2 => 2,
            ColorFormat::Y4 | ColorFormat::Pal4 => 4,
            ColorFormat::Y8 | ColorFormat::Pal8 => 8,
            ColorFormat::YCbCr420 => 12,
            ColorFormat::RGB4444
            | ColorFormat::RGB0555
            | ColorFormat::RGB0565
            | ColorFormat::RGB1555
            | ColorFormat::YA8
            | ColorFormat::Y16 => 16,
            ColorFormat::BGR0888 | ColorFormat::RGB0888 => 24,
            ColorFormat::BGR8888 | ColorFormat::RGB8888 | ColorFormat::YA16 => 32,
            ColorFormat::RGB0ggg => 48,
            ColorFormat::RGBgggg => 64,
        }
    }

    // ------------------------------------------------------------------------
    pub fn stride(&self, cx: usize, alignment: usize) -> usize {
        assert!((alignment & (alignment - 1)) == 0); // Make sure alignment is a power of 2
        let mask = !(alignment - 1);
        ((cx * self.bpp()).div_ceil(8) + alignment - 1) & mask
    }
}
