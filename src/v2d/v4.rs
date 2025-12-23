use super::float_eq::float_eq_rel;
use super::v3::V3;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// ----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct V4 {
    m: [f32; 4],
}

// ----------------------------------------------------------------------------
impl Default for V4 {
    fn default() -> Self {
        V4::zero()
    }
}

// ----------------------------------------------------------------------------
impl PartialEq for V4 {
    #[rustfmt::skip]
    fn eq(&self, rhs: &Self) -> bool {
        float_eq_rel(self.x0(), rhs.x0()) &&
        float_eq_rel(self.x1(), rhs.x1()) &&
        float_eq_rel(self.x2(), rhs.x2()) &&
        float_eq_rel(self.x3(), rhs.x3())
    }
}

// ----------------------------------------------------------------------------
impl Add for V4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let x0 = self.x0() + rhs.x0();
        let x1 = self.x1() + rhs.x1();
        let x2 = self.x2() + rhs.x2();
        let x3 = self.x3() + rhs.x3();
        V4::new([x0, x1, x2, x3])
    }
}

// ----------------------------------------------------------------------------
impl Sub for V4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let x0 = self.x0() - rhs.x0();
        let x1 = self.x1() - rhs.x1();
        let x2 = self.x2() - rhs.x2();
        let x3 = self.x3() - rhs.x3();
        V4::new([x0, x1, x2, x3])
    }
}

// ----------------------------------------------------------------------------
// V4 * f32 -> V4
impl Mul<f32> for V4 {
    type Output = Self;

    fn mul(self, s: f32) -> Self {
        let x0 = self.x0() * s;
        let x1 = self.x1() * s;
        let x2 = self.x2() * s;
        let x3 = self.x3() * s;
        V4::new([x0, x1, x2, x3])
    }
}

// ----------------------------------------------------------------------------
// f32 * V4 -> V4
impl Mul<V4> for f32 {
    type Output = V4;

    fn mul(self, v: V4) -> V4 {
        let x0 = self * v.x0();
        let x1 = self * v.x1();
        let x2 = self * v.x2();
        let x3 = self * v.x3();
        V4::new([x0, x1, x2, x3])
    }
}

// ----------------------------------------------------------------------------
// V4 * V4 -> f32
impl Mul for V4 {
    type Output = f32;

    fn mul(self, rhs: Self) -> f32 {
        self.x0() * rhs.x0() + self.x1() * rhs.x1() + self.x2() * rhs.x2() + self.x3() * rhs.x3()
    }
}

// ----------------------------------------------------------------------------
impl Neg for V4 {
    type Output = Self;

    fn neg(self) -> Self {
        V4::new([-self.x0(), -self.x1(), -self.x2(), -self.x3()])
    }
}

// ----------------------------------------------------------------------------
impl AddAssign for V4 {
    fn add_assign(&mut self, rhs: Self) {
        self.m[0] += rhs.x0();
        self.m[1] += rhs.x1();
        self.m[2] += rhs.x2();
        self.m[3] += rhs.x3();
    }
}

// ----------------------------------------------------------------------------
impl SubAssign for V4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.m[0] -= rhs.x0();
        self.m[1] -= rhs.x1();
        self.m[2] -= rhs.x2();
        self.m[3] -= rhs.x3();
    }
}

// ----------------------------------------------------------------------------
impl MulAssign<f32> for V4 {
    fn mul_assign(&mut self, s: f32) {
        self.m[0] *= s;
        self.m[1] *= s;
        self.m[2] *= s;
        self.m[3] *= s;
    }
}

// ----------------------------------------------------------------------------
impl From<[f32; 4]> for V4 {
    fn from(m: [f32; 4]) -> Self {
        V4 { m }
    }
}

// ----------------------------------------------------------------------------
impl V4 {
    // ------------------------------------------------------------------------
    pub const fn new(m: [f32; 4]) -> Self {
        V4 { m }
    }

    // ------------------------------------------------------------------------
    pub const fn zero() -> Self {
        V4::new([0.0, 0.0, 0.0, 0.0])
    }

    // ------------------------------------------------------------------------
    pub const fn from_v3(v: &V3, w: f32) -> Self {
        V4::new([v.x0(), v.x1(), v.x2(), w])
    }

    // ------------------------------------------------------------------------
    pub const fn from_slice(m: &[f32; 4]) -> Self {
        V4 { m: *m }
    }

    // ------------------------------------------------------------------------
    pub const fn with_x0(mut self, value: f32) -> Self {
        self.m[0] = value;
        self
    }

    pub const fn with_x1(mut self, value: f32) -> Self {
        self.m[1] = value;
        self
    }

    pub const fn with_x2(mut self, value: f32) -> Self {
        self.m[2] = value;
        self
    }

    pub const fn with_x3(mut self, value: f32) -> Self {
        self.m[3] = value;
        self
    }

    // ------------------------------------------------------------------------
    pub const X0: Self = Self::new([1.0, 0.0, 0.0, 0.0]);
    pub const X1: Self = Self::new([0.0, 1.0, 0.0, 0.0]);
    pub const X2: Self = Self::new([0.0, 0.0, 1.0, 0.0]);
    pub const X3: Self = Self::new([0.0, 0.0, 0.0, 1.0]);

    // ------------------------------------------------------------------------
    pub const fn x0(&self) -> f32 {
        self.m[0]
    }

    pub const fn x1(&self) -> f32 {
        self.m[1]
    }

    pub const fn x2(&self) -> f32 {
        self.m[2]
    }

    pub const fn x3(&self) -> f32 {
        self.m[3]
    }

    // ------------------------------------------------------------------------
    pub fn as_array(&self) -> [f32; 4] {
        self.m
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.m.as_ptr()
    }

    // ------------------------------------------------------------------------
    pub const fn length2(&self) -> f32 {
        self.x0() * self.x0()
            + self.x1() * self.x1()
            + self.x2() * self.x2()
            + self.x3() * self.x3()
    }

    pub fn length(&self) -> f32 {
        self.length2().sqrt()
    }

    pub fn distance(x0: &Self, x1: &Self) -> f32 {
        let d = *x1 - *x0;
        d.length()
    }

    pub fn norm(&self) -> Self {
        let l2 = self.length2();
        if l2 < f32::EPSILON {
            V4::default()
        } else {
            let inv_l = 1.0 / l2.sqrt();
            let x0 = self.x0() * inv_l;
            let x1 = self.x1() * inv_l;
            let x2 = self.x2() * inv_l;
            let x3 = self.x3() * inv_l;
            V4::new([x0, x1, x2, x3])
        }
    }

    pub fn abs(&self) -> Self {
        V4::new([
            self.x0().abs(),
            self.x1().abs(),
            self.x2().abs(),
            self.x3().abs(),
        ])
    }

    pub const fn dot(v0: &Self, v1: &Self) -> f32 {
        v0.x0() * v1.x0() + v0.x1() * v1.x1() + v0.x2() * v1.x2() + v0.x3() * v1.x3()
    }

    pub const fn cross(v0: &Self, v1: &Self) -> Self {
        V4::new([
            v0.x1() * v1.x2() - v0.x2() * v1.x1(),
            v0.x2() * v1.x0() - v0.x0() * v1.x2(),
            v0.x0() * v1.x1() - v0.x1() * v1.x0(),
            0.0,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v4() {
        let v0 = V4::new([-1.0, 1.0, 5.0, 3.0]);
        let v1 = V4::new([-2.0, 2.0, 2.0, -2.0]);

        assert_eq!(v0 + v1, V4::new([-3.0, 3.0, 7.0, 1.0]));
        assert_eq!(v0 - v1, V4::new([1.0, -1.0, 3.0, 5.0]));
        assert_eq!(v0 * 2.0, V4::new([-2.0, 2.0, 10.0, 6.0]));
        assert_eq!(2.0 * v0, V4::new([-2.0, 2.0, 10.0, 6.0]));
        assert_eq!(v0 * v1, 8.0);
        assert_eq!(-v0, V4::new([1.0, -1.0, -5.0, -3.0]));
        assert_eq!(v1.length2(), 16.0);
        assert_eq!(v1.length(), 4.0);
        assert_eq!(v1.norm(), V4::new([-0.5, 0.5, 0.5, -0.5]));
        assert_eq!(v0.abs(), V4::new([1.0, 1.0, 5.0, 3.0]));
        assert_eq!(V4::distance(&v0, &v1), 6.0);
    }
}
