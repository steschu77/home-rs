use std::ops::{Add, Mul, Neg, Sub};

use super::float_eq::float_eq_rel;
use super::v2::V2;

// ----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct R2 {
    radians: f32,
    sin: f32,
    cos: f32,
}

// ----------------------------------------------------------------------------
impl Default for R2 {
    fn default() -> Self {
        R2 {
            radians: 0.0,
            sin: 0.0,
            cos: 1.0,
        }
    }
}

// ----------------------------------------------------------------------------
impl PartialEq for R2 {
    fn eq(&self, rhs: &Self) -> bool {
        float_eq_rel(self.radians, rhs.radians)
    }
}

// ----------------------------------------------------------------------------
impl Add for R2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        R2::new(self.radians + rhs.radians)
    }
}

// ----------------------------------------------------------------------------
impl Sub for R2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        R2::new(self.radians - rhs.radians)
    }
}

// ----------------------------------------------------------------------------
// R2 * f32 -> R2
impl Mul<f32> for R2 {
    type Output = Self;

    fn mul(self, s: f32) -> Self {
        R2::new(self.radians * s)
    }
}

// ----------------------------------------------------------------------------
// f32 * R2 -> R2
impl Mul<R2> for f32 {
    type Output = R2;

    fn mul(self, q: R2) -> R2 {
        R2::new(self * q.radians)
    }
}

// ----------------------------------------------------------------------------
// R2 * V2 -> V2
impl Mul<V2> for R2 {
    type Output = V2;

    fn mul(self, v: V2) -> V2 {
        V2::new([
            self.cos * v.x0() - self.sin * v.x1(),
            self.sin * v.x0() + self.cos * v.x1(),
        ])
    }
}

// ----------------------------------------------------------------------------
impl Mul<R2> for V2 {
    type Output = V2;

    fn mul(self, q: R2) -> V2 {
        V2::new([
            q.cos * self.x0() - q.sin * self.x1(),
            q.sin * self.x0() + q.cos * self.x1(),
        ])
    }
}

// ----------------------------------------------------------------------------
impl Neg for R2 {
    type Output = Self;

    fn neg(self) -> Self {
        R2::new(-self.radians)
    }
}

// ----------------------------------------------------------------------------
impl R2 {
    pub fn new(radians: f32) -> Self {
        R2 {
            radians,
            sin: radians.sin(),
            cos: radians.cos(),
        }
    }

    pub fn get(&self) -> f32 {
        self.radians
    }

    pub fn x_axis(&self) -> V2 {
        V2::new([self.cos, self.sin])
    }

    pub fn y_axis(&self) -> V2 {
        V2::new([-self.sin, self.cos])
    }
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r2_dot() {
        let res = R2::new(0.5 * std::f32::consts::PI) * V2::new([1.0, 2.0]);
        assert_eq!(res, V2::new([-2.0, 1.0]));
    }
}
