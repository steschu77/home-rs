use super::float_eq::float_eq_rel;
use super::v2::V2;
use std::ops::{Add, Mul, Neg, Sub};

// ----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct V3 {
    m: [f32; 3],
}

// ----------------------------------------------------------------------------
impl Default for V3 {
    fn default() -> Self {
        V3::zero()
    }
}

// ----------------------------------------------------------------------------
impl PartialEq for V3 {
    #[rustfmt::skip]
    fn eq(&self, rhs: &Self) -> bool {
        float_eq_rel(self.x0(), rhs.x0()) &&
        float_eq_rel(self.x1(), rhs.x1()) &&
        float_eq_rel(self.x2(), rhs.x2())
    }
}

// ----------------------------------------------------------------------------
impl Add for V3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let x0 = self.x0() + rhs.x0();
        let x1 = self.x1() + rhs.x1();
        let x2 = self.x2() + rhs.x2();
        V3::new([x0, x1, x2])
    }
}

// ----------------------------------------------------------------------------
impl Sub for V3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let x0 = self.x0() - rhs.x0();
        let x1 = self.x1() - rhs.x1();
        let x2 = self.x2() - rhs.x2();
        V3::new([x0, x1, x2])
    }
}

// ----------------------------------------------------------------------------
// V3 * f32 -> V3
impl Mul<f32> for V3 {
    type Output = Self;

    fn mul(self, s: f32) -> Self {
        let x0 = self.x0() * s;
        let x1 = self.x1() * s;
        let x2 = self.x2() * s;
        V3::new([x0, x1, x2])
    }
}

// ----------------------------------------------------------------------------
// f32 * V3 -> V3
impl Mul<V3> for f32 {
    type Output = V3;

    fn mul(self, v: V3) -> V3 {
        let x0 = self * v.x0();
        let x1 = self * v.x1();
        let x2 = self * v.x2();
        V3::new([x0, x1, x2])
    }
}

// ----------------------------------------------------------------------------
// V3 * V3 -> f32
impl Mul for V3 {
    type Output = f32;

    fn mul(self, rhs: Self) -> f32 {
        self.x0() * rhs.x0() + self.x1() * rhs.x1() + self.x2() * rhs.x2()
    }
}

// ----------------------------------------------------------------------------
impl Neg for V3 {
    type Output = Self;

    fn neg(self) -> Self {
        V3::new([-self.x0(), -self.x1(), -self.x2()])
    }
}

// ----------------------------------------------------------------------------
impl From<[f32; 3]> for V3 {
    fn from(m: [f32; 3]) -> Self {
        V3 { m }
    }
}

// ----------------------------------------------------------------------------
impl V3 {
    // ------------------------------------------------------------------------
    pub const fn new(m: [f32; 3]) -> Self {
        V3 { m }
    }

    // ------------------------------------------------------------------------
    pub const fn zero() -> Self {
        V3::new([0.0, 0.0, 0.0])
    }

    // ------------------------------------------------------------------------
    pub const fn from_v2(v: &V2, z: f32) -> Self {
        V3::new([v.x0(), v.x1(), z])
    }

    // ------------------------------------------------------------------------
    pub const fn from_slice(m: &[f32; 3]) -> Self {
        V3 { m: *m }
    }

    // ------------------------------------------------------------------------
    pub fn as_ptr(&self) -> *const f32 {
        self.m.as_ptr()
    }

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

    // ------------------------------------------------------------------------
    pub const fn length2(&self) -> f32 {
        self.x0() * self.x0() + self.x1() * self.x1() + self.x2() * self.x2()
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
            V3::default()
        } else {
            let inv_l = 1.0 / l2.sqrt();
            let x0 = self.x0() * inv_l;
            let x1 = self.x1() * inv_l;
            let x2 = self.x2() * inv_l;
            V3::new([x0, x1, x2])
        }
    }

    // ------------------------------------------------------------------------
    pub fn abs(&self) -> Self {
        V3::new([self.x0().abs(), self.x1().abs(), self.x2().abs()])
    }

    // ------------------------------------------------------------------------
    pub const fn dot(v0: &Self, v1: &Self) -> f32 {
        v0.x0() * v1.x0() + v0.x1() * v1.x1() + v0.x2() * v1.x2()
    }

    // ------------------------------------------------------------------------
    pub const fn cross(v0: &Self, v1: &Self) -> Self {
        let x0 = v0.x1() * v1.x2() - v0.x2() * v1.x1();
        let x1 = v0.x2() * v1.x0() - v0.x0() * v1.x2();
        let x2 = v0.x0() * v1.x1() - v0.x1() * v1.x0();
        V3::new([x0, x1, x2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v3() {
        let v0 = V3::new([3.0, 4.0, 0.0]);
        let v1 = V3::new([1.0, 2.0, 1.0]);

        assert_eq!(v0.x0(), 3.0);
        assert_eq!(v0.x1(), 4.0);
        assert_eq!(v0.x2(), 0.0);
        assert_eq!(v0 + v1, V3::new([4.0, 6.0, 1.0]));
        assert_eq!(v0 - v1, V3::new([2.0, 2.0, -1.0]));
        assert_eq!(v0 * 2.0, V3::new([6.0, 8.0, 0.0]));
        assert_eq!(2.0 * v0, V3::new([6.0, 8.0, 0.0]));
        assert_eq!(v0 * v1, 11.0);
        assert_eq!(-v0, V3::new([-3.0, -4.0, 0.0]));
        assert_eq!(v0.length2(), 25.0);
        assert_eq!(v0.length(), 5.0);
        assert_eq!(v0.norm(), V3::new([0.6, 0.8, 0.0]));
        assert_eq!(v0.abs(), V3::new([3.0, 4.0, 0.0]));
        assert_eq!(V3::distance(&v0, &v1), 3.0);
        assert_eq!(V3::dot(&v0, &v1), 11.0);
        assert_eq!(V3::cross(&v0, &v1), V3::new([4.0, -3.0, 2.0]));
    }
}
