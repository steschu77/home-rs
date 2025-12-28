use super::m4x4::M4x4;
use super::v4::V4;

// ----------------------------------------------------------------------------
#[rustfmt::skip]
pub fn translate(v: &V4) -> M4x4
{
    let [x0, x1, x2, x3] = v.as_array();

    M4x4::new([
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        x0,  x1,   x2,  x3
    ])
}

// ----------------------------------------------------------------------------
pub fn scale(v: &V4) -> M4x4 {
    M4x4::diag(v.as_array())
}

// ----------------------------------------------------------------------------
#[rustfmt::skip]
pub fn rotate_x0(rad: f32) -> M4x4
{
    let s = rad.sin();
    let c = rad.cos();

    M4x4::new([
        1.0, 0.0, 0.0, 0.0,
        0.0,   c,  -s, 0.0,
        0.0,   s,   c, 0.0,
        0.0, 0.0, 0.0, 1.0
    ])
}

// ----------------------------------------------------------------------------
#[rustfmt::skip]
pub fn rotate_x1(rad: f32) -> M4x4
{
    let s = rad.sin();
    let c = rad.cos();

    M4x4::new([
          c, 0.0,   s, 0.0,
        0.0, 1.0, 0.0, 0.0,
         -s, 0.0,   c, 0.0,
        0.0, 0.0, 0.0, 1.0
    ])
}

// ----------------------------------------------------------------------------
#[rustfmt::skip]
pub fn rotate_x2(rad: f32) -> M4x4 {
    let s = rad.sin();
    let c = rad.cos();

    M4x4::new([
          c,  -s, 0.0, 0.0,
          s,   c, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

// ----------------------------------------------------------------------------
#[rustfmt::skip]
pub fn rotate(v: &V4, rad: f32) -> M4x4 {
    let s = rad.sin();
    let c = rad.cos();

    let vs = s * *v;

    let a0 = v.x0() * v.x0();
    let a1 = v.x1() * v.x1();
    let a2 = v.x2() * v.x2();
    let a3 = v.x0() * v.x1();
    let a4 = v.x1() * v.x2();
    let a5 = v.x2() * v.x0();

    let c0 = a0 - a0 * c;
    let c1 = a1 - a1 * c;
    let c2 = a2 - a2 * c;
    let c3 = a3 - a3 * c;
    let c4 = a4 - a4 * c;
    let c5 = a5 - a5 * c;

    let b00 = c0 + c;
    let b01 = c3 - vs.x2();
    let b02 = c5 + vs.x1();
    let b10 = c3 + vs.x2();
    let b11 = c1 + c;
    let b12 = c4 - vs.x0();
    let b20 = c5 - vs.x1();
    let b21 = c4 + vs.x0();
    let b22 = c2 + c;

    M4x4::new([
        b00, b01, b02, 0.0,
        b10, b11, b12, 0.0,
        b20, b21, b22, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

//   [      1       0       0   0 ]   [ xaxis.x  yaxis.x  zaxis.x 0 ]
//   [      0       1       0   0 ] * [ xaxis.y  yaxis.y  zaxis.y 0 ]
//   [      0       0       1   0 ]   [ xaxis.z  yaxis.z  zaxis.z 0 ]
//   [ -eye.x  -eye.y  -eye.z   1 ]   [       0        0        0 1 ]
//
//   [         xaxis.x          yaxis.x          zaxis.x  0 ]
// = [         xaxis.y          yaxis.y          zaxis.y  0 ]
//   [         xaxis.z          yaxis.z          zaxis.z  0 ]
//   [ dot(xaxis,-eye)  dot(yaxis,-eye)  dot(zaxis,-eye)  1 ]
// ----------------------------------------------------------------------------
#[rustfmt::skip]
pub fn look_at(eye: V4, at: V4, up: V4) -> M4x4 {
    let zaxis = (at - eye).norm();              // Camera Forward vector
    let xaxis = V4::cross(&zaxis, &up).norm();  // Camera Side vector
    let yaxis = V4::cross(&xaxis, &zaxis);      // Camera Up vector

    M4x4::new([
        xaxis.x0(), yaxis.x0(), -zaxis.x0(), 0.0,
        xaxis.x1(), yaxis.x1(), -zaxis.x1(), 0.0,
        xaxis.x2(), yaxis.x2(), -zaxis.x2(), 0.0,
       -xaxis*eye, -yaxis*eye,   zaxis*eye,  1.0,
    ])
}

// ------------------------------------------------------------------------
pub fn ortho2d(aspect: f32, zoom: f32) -> M4x4 {
    let x00 = 1.0 / (aspect * zoom);
    let x11 = 1.0 / zoom;

    M4x4::zero()
        .with((0, 0), 2.0 * x00)
        .with((1, 1), 2.0 * x11)
        .with((3, 0), -x00)
        .with((3, 1), -x11)
        .with((3, 3), 1.0)
}

// ----------------------------------------------------------------------------
pub fn perspective(fov: f32, aspect: f32, zn: f32, zf: f32) -> M4x4 {
    let fov = fov.to_radians();
    let y = 1.0 / (0.5 * fov).tan();
    let dz = 1.0 / (zf - zn);

    M4x4::zero()
        .with((0, 0), y / aspect)
        .with((1, 1), y)
        .with((2, 2), -(zf + zn) * dz)
        .with((2, 3), -1.0)
        .with((3, 2), -2.0 * zn * zf * dz)
}
