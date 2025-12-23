use std::ops::{Add, Mul, Neg, Sub};

use super::float_eq::float_eq_rel;
use super::m2x2::M2x2;
use super::v3::V3;

// ----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct M3x3 {
    m: [f32; 9],
}

// ----------------------------------------------------------------------------
impl Default for M3x3 {
    #[rustfmt::skip]
    fn default() -> Self {
        Self::zero()
    }
}

// ----------------------------------------------------------------------------
impl PartialEq for M3x3 {
    #[rustfmt::skip]
    fn eq(&self, rhs: &Self) -> bool {
        float_eq_rel(self.x00(), rhs.x00()) &&
        float_eq_rel(self.x01(), rhs.x01()) &&
        float_eq_rel(self.x02(), rhs.x02()) &&
        float_eq_rel(self.x10(), rhs.x10()) &&
        float_eq_rel(self.x11(), rhs.x11()) &&
        float_eq_rel(self.x12(), rhs.x12()) &&
        float_eq_rel(self.x20(), rhs.x20()) &&
        float_eq_rel(self.x21(), rhs.x21()) &&
        float_eq_rel(self.x22(), rhs.x22())
    }
}

// ----------------------------------------------------------------------------
impl Add for M3x3 {
    type Output = Self;

    #[rustfmt::skip]
    fn add(self, rhs: Self) -> Self::Output {
        M3x3::new([
            self.x00() + rhs.x00(), self.x01() + rhs.x01(), self.x02() + rhs.x02(),
            self.x10() + rhs.x10(), self.x11() + rhs.x11(), self.x12() + rhs.x12(),
            self.x20() + rhs.x20(), self.x21() + rhs.x21(), self.x22() + rhs.x22()
        ])
    }
}

// ----------------------------------------------------------------------------
impl Sub for M3x3 {
    type Output = Self;

    #[rustfmt::skip]
    fn sub(self, rhs: Self) -> Self::Output {
        M3x3::new([
            self.x00() - rhs.x00(), self.x01() - rhs.x01(), self.x02() - rhs.x02(),
            self.x10() - rhs.x10(), self.x11() - rhs.x11(), self.x12() - rhs.x12(),
            self.x20() - rhs.x20(), self.x21() - rhs.x21(), self.x22() - rhs.x22()
        ])
    }
}

// ----------------------------------------------------------------------------
// M3x3 * f32 -> M3x3
impl Mul<f32> for M3x3 {
    type Output = Self;

    #[rustfmt::skip]
    fn mul(self, s: f32) -> Self::Output {
        M3x3::new([
            self.x00() * s, self.x01() * s, self.x02() * s,
            self.x10() * s, self.x11() * s, self.x12() * s,
            self.x20() * s, self.x21() * s, self.x22() * s
        ])
    }
}

// ----------------------------------------------------------------------------
// f32 * M3x3 -> M3x3
impl Mul<M3x3> for f32 {
    type Output = M3x3;

    #[rustfmt::skip]
    fn mul(self, m: M3x3) -> Self::Output {
        M3x3::new([
            self * m.x00(), self * m.x01(), self * m.x02(),
            self * m.x10(), self * m.x11(), self * m.x12(),
            self * m.x20(), self * m.x21(), self * m.x22()
        ])
    }
}

// ----------------------------------------------------------------------------
// M3x3 * V3 -> V3
impl Mul<V3> for M3x3 {
    type Output = V3;

    fn mul(self, v: V3) -> Self::Output {
        V3::new([
            self.x00() * v.x0() + self.x01() * v.x1() + self.x02() * v.x2(),
            self.x10() * v.x0() + self.x11() * v.x1() + self.x12() * v.x2(),
            self.x20() * v.x0() + self.x21() * v.x1() + self.x22() * v.x2(),
        ])
    }
}

// ----------------------------------------------------------------------------
// V3 * M3x3 -> V3
impl Mul<M3x3> for V3 {
    type Output = V3;

    fn mul(self, m: M3x3) -> Self::Output {
        V3::new([
            self.x0() * m.x00() + self.x1() * m.x10() + self.x2() * m.x20(),
            self.x0() * m.x01() + self.x1() * m.x11() + self.x2() * m.x21(),
            self.x0() * m.x02() + self.x1() * m.x12() + self.x2() * m.x22(),
        ])
    }
}

// ----------------------------------------------------------------------------
// M3x3 * M3x3 -> M3x3
impl Mul<M3x3> for M3x3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x00 = self.x00() * rhs.x00() + self.x01() * rhs.x10() + self.x02() * rhs.x20();
        let x01 = self.x00() * rhs.x01() + self.x01() * rhs.x11() + self.x02() * rhs.x21();
        let x02 = self.x00() * rhs.x02() + self.x01() * rhs.x12() + self.x02() * rhs.x22();
        let x10 = self.x10() * rhs.x00() + self.x11() * rhs.x10() + self.x12() * rhs.x20();
        let x11 = self.x10() * rhs.x01() + self.x11() * rhs.x11() + self.x12() * rhs.x21();
        let x12 = self.x10() * rhs.x02() + self.x11() * rhs.x12() + self.x12() * rhs.x22();
        let x20 = self.x20() * rhs.x00() + self.x21() * rhs.x10() + self.x22() * rhs.x20();
        let x21 = self.x20() * rhs.x01() + self.x21() * rhs.x11() + self.x22() * rhs.x21();
        let x22 = self.x20() * rhs.x02() + self.x21() * rhs.x12() + self.x22() * rhs.x22();
        M3x3::new([x00, x01, x02, x10, x11, x12, x20, x21, x22])
    }
}

// ----------------------------------------------------------------------------
impl Neg for M3x3 {
    type Output = Self;

    #[rustfmt::skip]
    fn neg(self) -> Self {
        M3x3::new([
            -self.x00(), -self.x01(), -self.x02(),
            -self.x10(), -self.x11(), -self.x12(),
            -self.x20(), -self.x21(), -self.x22()
        ])
    }
}

// ----------------------------------------------------------------------------
impl M3x3 {
    // ------------------------------------------------------------------------
    pub const fn new(m: [f32; 9]) -> Self {
        M3x3 { m }
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub const fn from_cols(c0: V3, c1: V3, c2: V3) -> Self {
        M3x3::new([
            c0.x0(), c1.x0(), c2.x0(),
            c0.x1(), c1.x1(), c2.x1(),
            c0.x2(), c1.x2(), c2.x2()
        ])
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub const fn from_rows(r0: V3, r1: V3, r2: V3) -> Self {
        M3x3::new([
            r0.x0(), r0.x1(), r0.x2(),
            r1.x0(), r1.x1(), r1.x2(),
            r2.x0(), r2.x1(), r2.x2()
        ])
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub const fn zero() -> Self {
        M3x3::new([
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0
        ])
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub const fn diag(d0: f32, d1: f32, d2: f32) -> Self {
        M3x3::new([
            d0, 0.0, 0.0,
            0.0, d1, 0.0,
            0.0, 0.0, d2
        ])
    }

    // ------------------------------------------------------------------------
    pub const fn identity() -> Self {
        M3x3::diag(1.0, 1.0, 1.0)
    }

    // ------------------------------------------------------------------------
    pub const fn scale(s: f32) -> Self {
        M3x3::diag(s, s, s)
    }

    // ------------------------------------------------------------------------
    pub const fn x<const I0: usize, const I1: usize>(&self) -> f32 {
        self.m[I0 + I1 * 3]
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
    pub const fn col<const I: usize>(&self) -> V3 {
        V3::new([self.m[I], self.m[I + 3], self.m[I + 6]])
    }

    // ------------------------------------------------------------------------
    pub const fn col0(&self) -> V3 {
        self.col::<0>()
    }

    // ------------------------------------------------------------------------
    pub const fn col1(&self) -> V3 {
        self.col::<1>()
    }

    // ------------------------------------------------------------------------
    pub const fn col2(&self) -> V3 {
        self.col::<2>()
    }

    // ------------------------------------------------------------------------
    pub const fn row<const I: usize>(&self) -> V3 {
        V3::new([self.m[I * 3], self.m[I * 3 + 1], self.m[I * 3 + 2]])
    }

    // ------------------------------------------------------------------------
    pub const fn row0(&self) -> V3 {
        self.row::<0>()
    }

    // ------------------------------------------------------------------------
    pub const fn row1(&self) -> V3 {
        self.row::<1>()
    }

    // ------------------------------------------------------------------------
    pub const fn row2(&self) -> V3 {
        self.row::<2>()
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub const fn transpose(&self) -> Self {
        M3x3::new([
            self.x00(), self.x10(), self.x20(),
            self.x01(), self.x11(), self.x21(),
            self.x02(), self.x12(), self.x22(),
        ])
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub fn abs(&self) -> Self {
        M3x3::new([
            self.x00().abs(), self.x01().abs(), self.x02().abs(),
            self.x10().abs(), self.x11().abs(), self.x12().abs(),
            self.x20().abs(), self.x21().abs(), self.x22().abs(),
        ])
    }

    // ------------------------------------------------------------------------
    // https://en.wikipedia.org/wiki/Minor_(linear_algebra)
    pub fn minor<const I: usize, const J: usize>(&self) -> M2x2 {
        let mut m = [0.0; 4];
        let mut k = 0;
        for i in 0..3 {
            if i == I {
                continue;
            }
            for j in 0..3 {
                if j == J {
                    continue;
                }
                m[k] = self.m[i + j * 3];
                k += 1;
            }
        }
        M2x2::new(m)
    }

    // ------------------------------------------------------------------------
    pub const fn det(&self) -> f32 {
        let det00 = self.x11() * self.x22() - self.x12() * self.x21();
        let det01 = self.x12() * self.x20() - self.x10() * self.x22();
        let det02 = self.x10() * self.x21() - self.x11() * self.x20();
        self.x00() * det00 + self.x01() * det01 + self.x02() * det02
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub fn inverse(&self) -> Self {
        let d = self.det();
        if d.abs() < f32::EPSILON {
            Self::zero()
        } else {
            let inv_d = 1.0 / d;
            let x00 = self.x11() * self.x22() - self.x12() * self.x21();
            let x01 = self.x02() * self.x21() - self.x01() * self.x22();
            let x02 = self.x01() * self.x12() - self.x02() * self.x11();
            let x10 = self.x12() * self.x20() - self.x10() * self.x22();
            let x11 = self.x00() * self.x22() - self.x02() * self.x20();
            let x12 = self.x02() * self.x10() - self.x00() * self.x12();
            let x20 = self.x10() * self.x21() - self.x11() * self.x20();
            let x21 = self.x01() * self.x20() - self.x00() * self.x21();
            let x22 = self.x00() * self.x11() - self.x01() * self.x10();
            inv_d * M3x3::new([
                x00, x01, x02,
                x10, x11, x12,
                x20, x21, x22
            ])
        }
    }

    // ------------------------------------------------------------------------
    pub fn solve(&self, v: V3) -> V3 {
        let d = self.det();
        if d.abs() < f32::EPSILON {
            V3::zero()
        } else {
            let inv_d = 1.0 / d;
            let x00 = self.x11() * self.x22() - self.x12() * self.x21();
            let x01 = self.x02() * self.x21() - self.x01() * self.x22();
            let x02 = self.x01() * self.x12() - self.x02() * self.x11();
            let x10 = self.x12() * self.x20() - self.x10() * self.x22();
            let x11 = self.x00() * self.x22() - self.x02() * self.x20();
            let x12 = self.x02() * self.x10() - self.x00() * self.x12();
            let x20 = self.x10() * self.x21() - self.x11() * self.x20();
            let x21 = self.x01() * self.x20() - self.x00() * self.x21();
            let x22 = self.x00() * self.x11() - self.x01() * self.x10();
            inv_d
                * V3::new([
                    x00 * v.x0() + x01 * v.x1() + x02 * v.x2(),
                    x10 * v.x0() + x11 * v.x1() + x12 * v.x2(),
                    x20 * v.x0() + x21 * v.x1() + x22 * v.x2(),
                ])
        }
    }
}
