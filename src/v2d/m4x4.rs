use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

use super::float_eq::float_eq_rel;
use super::m3x3::M3x3;
use super::v4::V4;

// ----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct M4x4 {
    m: [f32; 16],
}

// ----------------------------------------------------------------------------
impl Default for M4x4 {
    #[rustfmt::skip]
    fn default() -> Self {
        Self::zero()
    }
}

// ----------------------------------------------------------------------------
impl PartialEq for M4x4 {
    #[rustfmt::skip]
    fn eq(&self, rhs: &Self) -> bool {
        float_eq_rel(self.x00(), rhs.x00()) &&
        float_eq_rel(self.x01(), rhs.x01()) &&
        float_eq_rel(self.x02(), rhs.x02()) &&
        float_eq_rel(self.x03(), rhs.x03()) &&
        float_eq_rel(self.x10(), rhs.x10()) &&
        float_eq_rel(self.x11(), rhs.x11()) &&
        float_eq_rel(self.x12(), rhs.x12()) &&
        float_eq_rel(self.x13(), rhs.x13()) &&
        float_eq_rel(self.x20(), rhs.x20()) &&
        float_eq_rel(self.x21(), rhs.x21()) &&
        float_eq_rel(self.x22(), rhs.x22()) &&
        float_eq_rel(self.x23(), rhs.x23()) &&
        float_eq_rel(self.x30(), rhs.x30()) &&
        float_eq_rel(self.x31(), rhs.x31()) &&
        float_eq_rel(self.x32(), rhs.x32()) &&
        float_eq_rel(self.x33(), rhs.x33())
    }
}

// ----------------------------------------------------------------------------
impl Add for M4x4 {
    type Output = Self;

    #[rustfmt::skip]
    fn add(self, rhs: Self) -> Self::Output {
        M4x4::new([
            self.x00() + rhs.x00(), self.x01() + rhs.x01(), self.x02() + rhs.x02(), self.x03() + rhs.x03(),
            self.x10() + rhs.x10(), self.x11() + rhs.x11(), self.x12() + rhs.x12(), self.x13() + rhs.x13(),
            self.x20() + rhs.x20(), self.x21() + rhs.x21(), self.x22() + rhs.x22(), self.x23() + rhs.x23(),
            self.x30() + rhs.x30(), self.x31() + rhs.x31(), self.x32() + rhs.x32(), self.x33() + rhs.x33(),
        ])
    }
}

// ----------------------------------------------------------------------------
impl Sub for M4x4 {
    type Output = Self;

    #[rustfmt::skip]
    fn sub(self, rhs: Self) -> Self {
        M4x4::new([
            self.x00() - rhs.x00(), self.x01() - rhs.x01(), self.x02() - rhs.x02(), self.x03() - rhs.x03(),
            self.x10() - rhs.x10(), self.x11() - rhs.x11(), self.x12() - rhs.x12(), self.x13() - rhs.x13(),
            self.x20() - rhs.x20(), self.x21() - rhs.x21(), self.x22() - rhs.x22(), self.x23() - rhs.x23(),
            self.x30() - rhs.x30(), self.x31() - rhs.x31(), self.x32() - rhs.x32(), self.x33() - rhs.x33(),
        ])
    }
}

// ----------------------------------------------------------------------------
// M4x4 * f32 -> M4x4
impl Mul<f32> for M4x4 {
    type Output = Self;

    #[rustfmt::skip]
    fn mul(self, s: f32) -> Self::Output {
        M4x4::new([
            self.x00() * s, self.x01() * s, self.x02() * s, self.x03() * s,
            self.x10() * s, self.x11() * s, self.x12() * s, self.x13() * s,
            self.x20() * s, self.x21() * s, self.x22() * s, self.x23() * s,
            self.x30() * s, self.x31() * s, self.x32() * s, self.x33() * s
        ])
    }
}

// ----------------------------------------------------------------------------
// f32 * M4x4 -> M4x4
impl Mul<M4x4> for f32 {
    type Output = M4x4;

    #[rustfmt::skip]
    fn mul(self, m: M4x4) -> Self::Output {
        M4x4::new([
            self * m.x00(), self * m.x01(), self * m.x02(), self * m.x03(),
            self * m.x10(), self * m.x11(), self * m.x12(), self * m.x13(),
            self * m.x20(), self * m.x21(), self * m.x22(), self * m.x23(),
            self * m.x30(), self * m.x31(), self * m.x32(), self * m.x33()
        ])
    }
}

// ----------------------------------------------------------------------------
// M4x4 * V4 -> V4
impl Mul<V4> for M4x4 {
    type Output = V4;

    #[rustfmt::skip]
    fn mul(self, v: V4) -> Self::Output {
        let x0 = self.x00() * v.x0() + self.x01() * v.x1() + self.x02() * v.x2() + self.x03() * v.x3();
        let x1 = self.x10() * v.x0() + self.x11() * v.x1() + self.x12() * v.x2() + self.x13() * v.x3();
        let x2 = self.x20() * v.x0() + self.x21() * v.x1() + self.x22() * v.x2() + self.x23() * v.x3();
        let x3 = self.x30() * v.x0() + self.x31() * v.x1() + self.x32() * v.x2() + self.x33() * v.x3();
        V4::new([x0, x1, x2, x3])
    }
}

// ----------------------------------------------------------------------------
// V4 * M4x4 -> V4
impl Mul<M4x4> for V4 {
    type Output = V4;

    #[rustfmt::skip]
    fn mul(self, m: M4x4) -> Self::Output {
        let x0 = self.x0() * m.x00() + self.x1() * m.x10() + self.x2() * m.x20() + self.x3() * m.x30();
        let x1 = self.x0() * m.x01() + self.x1() * m.x11() + self.x2() * m.x21() + self.x3() * m.x31();
        let x2 = self.x0() * m.x02() + self.x1() * m.x12() + self.x2() * m.x22() + self.x3() * m.x32();
        let x3 = self.x0() * m.x03() + self.x1() * m.x13() + self.x2() * m.x23() + self.x3() * m.x33();
        V4::new([x0, x1, x2, x3])
    }
}

// ----------------------------------------------------------------------------
// M4x4 * M4x4 -> M4x4
impl Mul<M4x4> for M4x4 {
    type Output = Self;

    #[rustfmt::skip]
    fn mul(self, rhs: Self) -> Self::Output {
        let x00 = self.x00() * rhs.x00() + self.x01() * rhs.x10() + self.x02() * rhs.x20() + self.x03() * rhs.x30();
        let x01 = self.x00() * rhs.x01() + self.x01() * rhs.x11() + self.x02() * rhs.x21() + self.x03() * rhs.x31();
        let x02 = self.x00() * rhs.x02() + self.x01() * rhs.x12() + self.x02() * rhs.x22() + self.x03() * rhs.x32();
        let x03 = self.x00() * rhs.x03() + self.x01() * rhs.x13() + self.x02() * rhs.x23() + self.x03() * rhs.x33();
        let x10 = self.x10() * rhs.x00() + self.x11() * rhs.x10() + self.x12() * rhs.x20() + self.x13() * rhs.x30();
        let x11 = self.x10() * rhs.x01() + self.x11() * rhs.x11() + self.x12() * rhs.x21() + self.x13() * rhs.x31();
        let x12 = self.x10() * rhs.x02() + self.x11() * rhs.x12() + self.x12() * rhs.x22() + self.x13() * rhs.x32();
        let x13 = self.x10() * rhs.x03() + self.x11() * rhs.x13() + self.x12() * rhs.x23() + self.x13() * rhs.x33();
        let x20 = self.x20() * rhs.x00() + self.x21() * rhs.x10() + self.x22() * rhs.x20() + self.x23() * rhs.x30();
        let x21 = self.x20() * rhs.x01() + self.x21() * rhs.x11() + self.x22() * rhs.x21() + self.x23() * rhs.x31();
        let x22 = self.x20() * rhs.x02() + self.x21() * rhs.x12() + self.x22() * rhs.x22() + self.x23() * rhs.x32();
        let x23 = self.x20() * rhs.x03() + self.x21() * rhs.x13() + self.x22() * rhs.x23() + self.x23() * rhs.x33();
        let x30 = self.x30() * rhs.x00() + self.x31() * rhs.x10() + self.x32() * rhs.x20() + self.x33() * rhs.x30();
        let x31 = self.x30() * rhs.x01() + self.x31() * rhs.x11() + self.x32() * rhs.x21() + self.x33() * rhs.x31();
        let x32 = self.x30() * rhs.x02() + self.x31() * rhs.x12() + self.x32() * rhs.x22() + self.x33() * rhs.x32();
        let x33 = self.x30() * rhs.x03() + self.x31() * rhs.x13() + self.x32() * rhs.x23() + self.x33() * rhs.x33();
        M4x4::new([
            x00, x10, x20, x30,
            x01, x11, x21, x31,
            x02, x12, x22, x32,
            x03, x13, x23, x33])
    }
}

// ----------------------------------------------------------------------------
impl Neg for M4x4 {
    type Output = Self;

    #[rustfmt::skip]
    fn neg(self) -> Self {
        M4x4::new([
            -self.x00(), -self.x01(), -self.x02(), -self.x03(),
            -self.x10(), -self.x11(), -self.x12(), -self.x13(),
            -self.x20(), -self.x21(), -self.x22(), -self.x23(),
            -self.x30(), -self.x31(), -self.x32(), -self.x33(),
        ])
    }
}

// ----------------------------------------------------------------------------
impl Index<(usize, usize)> for M4x4 {
    type Output = f32;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.m[row * 4 + col]
    }
}

// ----------------------------------------------------------------------------
impl IndexMut<(usize, usize)> for M4x4 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.m[row * 4 + col]
    }
}

// ----------------------------------------------------------------------------
impl M4x4 {
    // ------------------------------------------------------------------------
    pub const fn new(m: [f32; 16]) -> Self {
        M4x4 { m }
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub const fn zero() -> Self {
        Self::new([
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0])
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub const fn diag(d: [f32; 4]) -> Self {
        M4x4::new([
            d[0], 0.0, 0.0, 0.0,
            0.0, d[1], 0.0, 0.0,
            0.0, 0.0, d[2], 0.0,
            0.0, 0.0, 0.0, d[3]
        ])
    }

    // ------------------------------------------------------------------------
    pub const fn identity() -> Self {
        M4x4::diag([1.0, 1.0, 1.0, 1.0])
    }

    // ------------------------------------------------------------------------
    pub fn from_slice(m: &[f32; 16]) -> Self {
        M4x4 { m: *m }
    }

    // ------------------------------------------------------------------------
    pub fn with(mut self, (row, col): (usize, usize), value: f32) -> Self {
        assert!(row < 4 && col < 4, "Index out of bounds");
        self.m[row * 4 + col] = value;
        self
    }

    // ------------------------------------------------------------------------
    pub fn as_array(&self) -> [f32; 16] {
        self.m
    }

    // ------------------------------------------------------------------------
    pub fn as_ptr(&self) -> *const f32 {
        self.m.as_ptr()
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub const fn from_cols(c0: V4, c1: V4, c2: V4, c3: V4) -> Self {
        M4x4::new([
            c0.x0(), c1.x0(), c2.x0(), c3.x0(),
            c0.x1(), c1.x1(), c2.x1(), c3.x1(),
            c0.x2(), c1.x2(), c2.x2(), c3.x2(),
            c0.x3(), c1.x3(), c2.x3(), c3.x3()])
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub const fn from_rows(r0: V4, r1: V4, r2: V4, r3: V4) -> Self {
        M4x4::new([
            r0.x0(), r0.x1(), r0.x2(), r0.x3(),
            r1.x0(), r1.x1(), r1.x2(), r1.x3(),
            r2.x0(), r2.x1(), r2.x2(), r2.x3(),
            r3.x0(), r3.x1(), r3.x2(), r3.x3()])
    }

    // ------------------------------------------------------------------------
    pub const fn x<const I0: usize, const I1: usize>(&self) -> f32 {
        self.m[I0 + I1 * 4]
    }

    // ------------------------------------------------------------------------
    pub const fn x00(&self) -> f32 {
        self.x::<0, 0>()
    }

    // ------------------------------------------------------------------------
    pub const fn x01(&self) -> f32 {
        self.x::<0, 1>()
    }

    // ------------------------------------------------------------------------
    pub const fn x02(&self) -> f32 {
        self.x::<0, 2>()
    }

    // ------------------------------------------------------------------------
    pub const fn x03(&self) -> f32 {
        self.x::<0, 3>()
    }

    // ------------------------------------------------------------------------
    pub const fn x10(&self) -> f32 {
        self.x::<1, 0>()
    }

    // ------------------------------------------------------------------------
    pub const fn x11(&self) -> f32 {
        self.x::<1, 1>()
    }

    // ------------------------------------------------------------------------
    pub const fn x12(&self) -> f32 {
        self.x::<1, 2>()
    }

    // ------------------------------------------------------------------------
    pub const fn x13(&self) -> f32 {
        self.x::<1, 3>()
    }

    // ------------------------------------------------------------------------
    pub const fn x20(&self) -> f32 {
        self.x::<2, 0>()
    }

    // ------------------------------------------------------------------------
    pub const fn x21(&self) -> f32 {
        self.x::<2, 1>()
    }

    // ------------------------------------------------------------------------
    pub const fn x22(&self) -> f32 {
        self.x::<2, 2>()
    }

    // ------------------------------------------------------------------------
    pub const fn x23(&self) -> f32 {
        self.x::<2, 3>()
    }

    // ------------------------------------------------------------------------
    pub const fn x30(&self) -> f32 {
        self.x::<3, 0>()
    }

    // ------------------------------------------------------------------------
    pub const fn x31(&self) -> f32 {
        self.x::<3, 1>()
    }

    // ------------------------------------------------------------------------
    pub const fn x32(&self) -> f32 {
        self.x::<3, 2>()
    }

    // ------------------------------------------------------------------------
    pub const fn x33(&self) -> f32 {
        self.x::<3, 3>()
    }

    // ------------------------------------------------------------------------
    pub const fn col<const I: usize>(&self) -> V4 {
        V4::new([self.m[I], self.m[I + 4], self.m[I + 8], self.m[I + 12]])
    }

    // ------------------------------------------------------------------------
    pub const fn col0(&self) -> V4 {
        self.col::<0>()
    }

    // ------------------------------------------------------------------------
    pub const fn col1(&self) -> V4 {
        self.col::<1>()
    }

    // ------------------------------------------------------------------------
    pub const fn col2(&self) -> V4 {
        self.col::<2>()
    }

    // ------------------------------------------------------------------------
    pub const fn col3(&self) -> V4 {
        self.col::<3>()
    }

    // ------------------------------------------------------------------------
    pub const fn row<const I: usize>(&self) -> V4 {
        V4::new([
            self.m[I * 4],
            self.m[I * 4 + 1],
            self.m[I * 4 + 2],
            self.m[I * 4 + 3],
        ])
    }

    // ------------------------------------------------------------------------
    pub const fn row0(&self) -> V4 {
        self.row::<0>()
    }

    // ------------------------------------------------------------------------
    pub const fn row1(&self) -> V4 {
        self.row::<1>()
    }

    // ------------------------------------------------------------------------
    pub const fn row2(&self) -> V4 {
        self.row::<2>()
    }

    // ------------------------------------------------------------------------
    pub const fn row3(&self) -> V4 {
        self.row::<3>()
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub const fn transpose(&self) -> Self {
        M4x4::new([
            self.x00(), self.x10(), self.x20(), self.x30(),
            self.x01(), self.x11(), self.x21(), self.x31(),
            self.x02(), self.x12(), self.x22(), self.x32(),
            self.x03(), self.x13(), self.x23(), self.x33(),
        ])
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub fn abs(&self) -> Self {
        M4x4::new([
            self.x00().abs(), self.x01().abs(), self.x02().abs(), self.x03().abs(),
            self.x10().abs(), self.x11().abs(), self.x12().abs(), self.x13().abs(),
            self.x20().abs(), self.x21().abs(), self.x22().abs(), self.x23().abs(),
            self.x30().abs(), self.x31().abs(), self.x32().abs(), self.x33().abs(),
        ])
    }

    // ------------------------------------------------------------------------
    // https://en.wikipedia.org/wiki/Minor_(linear_algebra)
    pub fn minor<const I: usize, const J: usize>(&self) -> M3x3 {
        let mut m = [0.0; 9];
        let mut k = 0;
        for i in 0..4 {
            if i == I {
                continue;
            }
            for j in 0..4 {
                if j == J {
                    continue;
                }
                m[k] = self.m[i + j * 4];
                k += 1;
            }
        }
        M3x3::new(m)
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub fn det(&self) -> f32 {
        self.x00() * self.minor::<0, 0>().det() -
        self.x01() * self.minor::<0, 1>().det() +
        self.x02() * self.minor::<0, 2>().det() -
        self.x03() * self.minor::<0, 3>().det()
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub fn inverse(&self) -> Self {
        let d = self.det();
        if d.abs() < f32::EPSILON {
            Self::zero()
        } else {
            let inv_d = 1.0 / d;
            let x00 =  self.minor::<0, 0>().det();
            let x01 = -self.minor::<1, 0>().det();
            let x02 =  self.minor::<2, 0>().det();
            let x03 = -self.minor::<3, 0>().det();
            let x10 = -self.minor::<0, 1>().det();
            let x11 =  self.minor::<1, 1>().det();
            let x12 = -self.minor::<2, 1>().det();
            let x13 =  self.minor::<3, 1>().det();
            let x20 =  self.minor::<0, 2>().det();
            let x21 = -self.minor::<1, 2>().det();
            let x22 =  self.minor::<2, 2>().det();
            let x23 = -self.minor::<3, 2>().det();
            let x30 = -self.minor::<0, 3>().det();
            let x31 =  self.minor::<1, 3>().det();
            let x32 = -self.minor::<2, 3>().det();
            let x33 =  self.minor::<3, 3>().det();
            inv_d
                * M4x4::new([
                    x00, x01, x02, x03,
                    x10, x11, x12, x13,
                    x20, x21, x22, x23,
                    x30, x31, x32, x33,
                ])
        }
    }
}
