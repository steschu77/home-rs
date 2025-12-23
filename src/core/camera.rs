use crate::error::Result;
use crate::v2d::m4x4::M4x4;
use crate::v2d::v4::V4;

// ----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct Camera {
    position: V4,
    zoom: f32,
}

// ----------------------------------------------------------------------------
impl Default for Camera {
    fn default() -> Self {
        Self {
            position: V4::new([0.0, 0.0, 0.0, 1.0]),
            zoom: 1.0,
        }
    }
}

// ----------------------------------------------------------------------------
impl Camera {
    pub fn new(position: V4, zoom: f32) -> Self {
        Self { position, zoom }
    }

    pub fn update(&mut self, _dt: &std::time::Duration) -> Result<()> {
        Ok(())
    }

    pub fn position(&self) -> V4 {
        self.position
    }

    pub fn transform(&self) -> M4x4 {
        M4x4::identity()
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }
}
