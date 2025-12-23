use std::ops::{Add, Mul, Neg, Sub};

use super::float_eq::float_eq_rel;
use super::v2::V2;

// ----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct M2x2 {
    m: [f32; 4],
}

// ----------------------------------------------------------------------------
impl Default for M2x2 {
    fn default() -> Self {
        Self::zero()
    }
}

// ----------------------------------------------------------------------------
impl PartialEq for M2x2 {
    #[rustfmt::skip]
    fn eq(&self, rhs: &Self) -> bool {
        float_eq_rel(self.x00(), rhs.x00()) &&
        float_eq_rel(self.x01(), rhs.x01()) &&
        float_eq_rel(self.x10(), rhs.x10()) &&
        float_eq_rel(self.x11(), rhs.x11())
    }
}

// ----------------------------------------------------------------------------
impl Add for M2x2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x00 = self.x00() + rhs.x00();
        let x01 = self.x01() + rhs.x01();
        let x10 = self.x10() + rhs.x10();
        let x11 = self.x11() + rhs.x11();
        M2x2::new([x00, x01, x10, x11])
    }
}

// ----------------------------------------------------------------------------
impl Sub for M2x2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let x00 = self.x00() - rhs.x00();
        let x01 = self.x01() - rhs.x01();
        let x10 = self.x10() - rhs.x10();
        let x11 = self.x11() - rhs.x11();
        M2x2::new([x00, x01, x10, x11])
    }
}

// ----------------------------------------------------------------------------
// M2x2 * f32 -> M2x2
impl Mul<f32> for M2x2 {
    type Output = Self;

    fn mul(self, s: f32) -> Self::Output {
        let x00 = self.x00() * s;
        let x01 = self.x01() * s;
        let x10 = self.x10() * s;
        let x11 = self.x11() * s;
        M2x2::new([x00, x01, x10, x11])
    }
}

// ----------------------------------------------------------------------------
// f32 * M2x2 -> M2x2
impl Mul<M2x2> for f32 {
    type Output = M2x2;

    fn mul(self, m: M2x2) -> Self::Output {
        let x00 = self * m.x00();
        let x01 = self * m.x01();
        let x10 = self * m.x10();
        let x11 = self * m.x11();
        M2x2::new([x00, x01, x10, x11])
    }
}

// ----------------------------------------------------------------------------
// M2x2 * V2 -> V2
impl Mul<V2> for M2x2 {
    type Output = V2;

    fn mul(self, v: V2) -> Self::Output {
        let x0 = self.x00() * v.x0() + self.x01() * v.x1();
        let x1 = self.x10() * v.x0() + self.x11() * v.x1();
        V2::new([x0, x1])
    }
}

// ----------------------------------------------------------------------------
// V2 * M2x2 -> V2
impl Mul<M2x2> for V2 {
    type Output = V2;

    fn mul(self, m: M2x2) -> Self::Output {
        let x0 = self.x0() * m.x00() + self.x1() * m.x10();
        let x1 = self.x0() * m.x01() + self.x1() * m.x11();
        V2::new([x0, x1])
    }
}

// ----------------------------------------------------------------------------
// M2x2 * M2x2 -> M2x2
impl Mul<M2x2> for M2x2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x00 = self.x00() * rhs.x00() + self.x01() * rhs.x10();
        let x01 = self.x00() * rhs.x01() + self.x01() * rhs.x11();
        let x10 = self.x10() * rhs.x00() + self.x11() * rhs.x10();
        let x11 = self.x10() * rhs.x01() + self.x11() * rhs.x11();
        M2x2::new([x00, x01, x10, x11])
    }
}

// ----------------------------------------------------------------------------
impl Neg for M2x2 {
    type Output = Self;

    fn neg(self) -> Self {
        M2x2::new([-self.x00(), -self.x01(), -self.x10(), -self.x11()])
    }
}

// ----------------------------------------------------------------------------
impl M2x2 {
    pub const fn new(m: [f32; 4]) -> Self {
        M2x2 { m }
    }

    // ------------------------------------------------------------------------
    pub const fn zero() -> Self {
        M2x2::new([0.0, 0.0, 0.0, 0.0])
    }

    // ------------------------------------------------------------------------
    pub const fn diag(d0: f32, d1: f32) -> Self {
        M2x2::new([d0, 0.0, 0.0, d1])
    }

    // ------------------------------------------------------------------------
    pub const fn identity() -> Self {
        M2x2::diag(1.0, 1.0)
    }

    // ------------------------------------------------------------------------
    pub fn rotation(r: f32) -> Self {
        let c = r.cos();
        let s = r.sin();
        M2x2::new([c, -s, s, c])
    }

    // ------------------------------------------------------------------------
    pub const fn scale(s: f32) -> Self {
        M2x2::diag(s, s)
    }

    // ------------------------------------------------------------------------
    pub const fn x<const I0: usize, const I1: usize>(&self) -> f32 {
        self.m[I0 * 2 + I1]
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
    pub const fn x10(&self) -> f32 {
        self.x::<1, 0>()
    }

    // ------------------------------------------------------------------------
    pub const fn x11(&self) -> f32 {
        self.x::<1, 1>()
    }

    // ------------------------------------------------------------------------
    pub const fn col<const I: usize>(&self) -> V2 {
        V2::new([self.m[I], self.m[I + 2]])
    }

    // ------------------------------------------------------------------------
    pub const fn col0(&self) -> V2 {
        self.col::<0>()
    }

    // ------------------------------------------------------------------------
    pub const fn col1(&self) -> V2 {
        self.col::<1>()
    }

    // ------------------------------------------------------------------------
    pub const fn row<const I: usize>(&self) -> V2 {
        V2::new([self.m[I * 2], self.m[I * 2 + 1]])
    }

    // ------------------------------------------------------------------------
    pub const fn row0(&self) -> V2 {
        self.row::<0>()
    }

    // ------------------------------------------------------------------------
    pub const fn row1(&self) -> V2 {
        self.row::<1>()
    }

    // ------------------------------------------------------------------------
    #[rustfmt::skip]
    pub const fn transpose(&self) -> Self {
        M2x2::new([
            self.x00(), self.x10(),
            self.x01(), self.x11()
        ])
    }

    // ------------------------------------------------------------------------
    pub fn abs(&self) -> Self {
        let x00 = self.x00().abs();
        let x01 = self.x01().abs();
        let x10 = self.x10().abs();
        let x11 = self.x11().abs();
        M2x2::new([x00, x01, x10, x11])
    }

    // ------------------------------------------------------------------------
    pub const fn det(&self) -> f32 {
        self.x00() * self.x11() - self.x01() * self.x10()
    }

    // ------------------------------------------------------------------------
    pub fn inverse(&self) -> Self {
        let d = self.det();
        if d.abs() < f32::EPSILON {
            Self::zero()
        } else {
            let inv_d = 1.0 / d;
            let x00 = inv_d * self.x11();
            let x01 = -inv_d * self.x01();
            let x10 = -inv_d * self.x10();
            let x11 = inv_d * self.x00();
            M2x2::new([x00, x01, x10, x11])
        }
    }

    // ------------------------------------------------------------------------
    pub fn solve(&self, v: V2) -> V2 {
        let d = self.det();
        if d.abs() < f32::EPSILON {
            V2::zero()
        } else {
            let inv_d = 1.0 / d;
            let x0 = inv_d * (self.x11() * v.x0() - self.x01() * v.x1());
            let x1 = inv_d * (self.x00() * v.x1() - self.x10() * v.x0());
            V2::new([x0, x1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_m2x2_getters() {
        let m = M2x2::new([1.0, 2.0, 3.0, 4.0]);

        assert_eq!(m.x::<0, 0>(), 1.0);
        assert_eq!(m.x::<0, 1>(), 2.0);
        assert_eq!(m.x::<1, 0>(), 3.0);
        assert_eq!(m.x::<1, 1>(), 4.0);

        assert_eq!(m.x00(), 1.0);
        assert_eq!(m.x01(), 2.0);
        assert_eq!(m.x10(), 3.0);
        assert_eq!(m.x11(), 4.0);

        assert_eq!(m.col::<0>(), V2::new([1.0, 3.0]));
        assert_eq!(m.col::<1>(), V2::new([2.0, 4.0]));

        assert_eq!(m.col0(), V2::new([1.0, 3.0]));
        assert_eq!(m.col1(), V2::new([2.0, 4.0]));

        assert_eq!(m.row::<0>(), V2::new([1.0, 2.0]));
        assert_eq!(m.row::<1>(), V2::new([3.0, 4.0]));

        assert_eq!(m.row0(), V2::new([1.0, 2.0]));
        assert_eq!(m.row1(), V2::new([3.0, 4.0]));
    }

    #[test]
    fn test_m2x2_constructors() {
        let zero = M2x2::zero();
        assert_eq!(zero, M2x2::new([0.0, 0.0, 0.0, 0.0]));

        let id = M2x2::identity();
        assert_eq!(id, M2x2::new([1.0, 0.0, 0.0, 1.0]));

        let r = M2x2::rotation(0.5 * std::f32::consts::PI);
        assert_eq!(r, M2x2::new([0.0, -1.0, 1.0, 0.0]));

        let s = M2x2::scale(2.0);
        assert_eq!(s, M2x2::new([2.0, 0.0, 0.0, 2.0]));
    }

    #[test]
    fn test_m2x2_ops() {
        let m = M2x2::new([-1.0, 3.0, 2.0, -5.0]);
        let v = V2::new([1.0, 2.0]);

        assert_eq!(m.transpose(), M2x2::new([-1.0, 2.0, 3.0, -5.0]));
        assert_eq!(m.abs(), M2x2::new([1.0, 3.0, 2.0, 5.0]));
        assert_eq!(m.det(), -1.0);
        assert_eq!(m.inverse(), M2x2::new([5.0, 3.0, 2.0, 1.0]));
        assert_eq!(m * m.inverse(), M2x2::identity());

        assert_eq!(v * m, V2::new([3.0, -7.0]));
        assert_eq!(m * v, V2::new([5.0, -8.0]));
        assert_eq!(m.solve(V2::new([5.0, -8.0])), v);

        assert_eq!(M2x2::zero().inverse(), M2x2::zero());
        assert_eq!(M2x2::zero().solve(v), V2::zero());
        assert_eq!(m.solve(V2::zero()), V2::zero());

        assert_eq!(m + m, M2x2::new([-2.0, 6.0, 4.0, -10.0]));
        assert_eq!(m - m, M2x2::zero());
        assert_eq!(-m, M2x2::new([1.0, -3.0, -2.0, 5.0]));
        assert_eq!(m * 2.0, M2x2::new([-2.0, 6.0, 4.0, -10.0]));
        assert_eq!(2.0 * m, M2x2::new([-2.0, 6.0, 4.0, -10.0]));
    }
}
