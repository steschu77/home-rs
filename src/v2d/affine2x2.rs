use super::m2x2::M2x2;

// ----------------------------------------------------------------------------
pub fn rotate2x2(deg: f32) -> M2x2 {
    let s = deg.sin();
    let c = deg.cos();

    M2x2::new([c, -s, s, c])
}
