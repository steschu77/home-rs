// Quaternion
use super::float_eq::float_eq_rel;
use super::v3::V3;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// ----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Q {
    m: [f32; 4],
}

// ----------------------------------------------------------------------------
impl Default for Q {
    fn default() -> Self {
        Q::identity()
    }
}

// ----------------------------------------------------------------------------
impl PartialEq for Q {
    fn eq(&self, rhs: &Self) -> bool {
        float_eq_rel(self.x(), rhs.x())
            && float_eq_rel(self.y(), rhs.y())
            && float_eq_rel(self.z(), rhs.z())
            && float_eq_rel(self.w(), rhs.w())
    }
}

// ----------------------------------------------------------------------------
// Q + Q -> Q
impl Add for Q {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Q::new([
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
            self.w() + rhs.w(),
        ])
    }
}

// ----------------------------------------------------------------------------
// Q - Q -> Q
impl Sub for Q {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Q::new([
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
            self.w() - rhs.w(),
        ])
    }
}

// ----------------------------------------------------------------------------
// Q * f32 -> Q
impl Mul<f32> for Q {
    type Output = Self;

    fn mul(self, s: f32) -> Self {
        Q::new([self.x() * s, self.y() * s, self.z() * s, self.w() * s])
    }
}

// ----------------------------------------------------------------------------
// f32 * Q -> Q
impl Mul<Q> for f32 {
    type Output = Q;

    fn mul(self, q: Q) -> Q {
        q * self
    }
}

// ----------------------------------------------------------------------------
// f32 * Q (ref)
impl Mul<&Q> for f32 {
    type Output = Q;

    fn mul(self, q: &Q) -> Q {
        *q * self
    }
}

// ----------------------------------------------------------------------------
// Q * Q -> Q (Hamilton product)
impl Mul for Q {
    type Output = Q;

    fn mul(self, rhs: Q) -> Q {
        let ax = self.x();
        let ay = self.y();
        let az = self.z();
        let aw = self.w();

        let bx = rhs.x();
        let by = rhs.y();
        let bz = rhs.z();
        let bw = rhs.w();

        Q::new([
            aw * bx + ax * bw + ay * bz - az * by,
            aw * by + ay * bw + az * bx - ax * bz,
            aw * bz + az * bw + ax * by - ay * bx,
            aw * bw - ax * bx - ay * by - az * bz,
        ])
    }
}

// ----------------------------------------------------------------------------
impl Neg for Q {
    type Output = Self;

    fn neg(self) -> Self {
        Q::new([-self.x(), -self.y(), -self.z(), -self.w()])
    }
}

// ----------------------------------------------------------------------------
impl AddAssign for Q {
    fn add_assign(&mut self, rhs: Self) {
        self.m[0] += rhs.x();
        self.m[1] += rhs.y();
        self.m[2] += rhs.z();
        self.m[3] += rhs.w();
    }
}

// ----------------------------------------------------------------------------
impl SubAssign for Q {
    fn sub_assign(&mut self, rhs: Self) {
        self.m[0] -= rhs.x();
        self.m[1] -= rhs.y();
        self.m[2] -= rhs.z();
        self.m[3] -= rhs.w();
    }
}

// ----------------------------------------------------------------------------
impl MulAssign<f32> for Q {
    fn mul_assign(&mut self, s: f32) {
        self.m[0] *= s;
        self.m[1] *= s;
        self.m[2] *= s;
        self.m[3] *= s;
    }
}

// ----------------------------------------------------------------------------
impl Q {
    pub const fn new(m: [f32; 4]) -> Self {
        Q { m }
    }

    pub const fn identity() -> Self {
        Q::new([0.0, 0.0, 0.0, 1.0])
    }

    // ------------------------------------------------------------------------
    pub const fn x(&self) -> f32 {
        self.m[0]
    }
    pub const fn y(&self) -> f32 {
        self.m[1]
    }
    pub const fn z(&self) -> f32 {
        self.m[2]
    }
    pub const fn w(&self) -> f32 {
        self.m[3]
    }

    // ------------------------------------------------------------------------
    pub const fn dot(a: &Self, b: &Self) -> f32 {
        a.x() * b.x() + a.y() * b.y() + a.z() * b.z() + a.w() * b.w()
    }

    // ------------------------------------------------------------------------
    pub const fn length2(&self) -> f32 {
        Self::dot(self, self)
    }

    // ------------------------------------------------------------------------
    pub fn length(&self) -> f32 {
        self.length2().sqrt()
    }

    // ------------------------------------------------------------------------
    pub fn norm(&self) -> Self {
        let l2 = self.length2();
        if l2 < f32::EPSILON {
            Q::identity()
        } else {
            let inv = 1.0 / l2.sqrt();
            *self * inv
        }
    }

    // ------------------------------------------------------------------------
    pub const fn conjugate(&self) -> Self {
        Q::new([-self.x(), -self.y(), -self.z(), self.w()])
    }

    // ------------------------------------------------------------------------
    pub fn inverse(&self) -> Self {
        let l2 = self.length2();
        if l2 < f32::EPSILON {
            Q::identity()
        } else {
            self.conjugate() * (1.0 / l2)
        }
    }

    // ----------------------------------------------------------------------------
    // NLERP: normalized linear interpolation
    pub fn nlerp(q0: Self, q1: Self, t: f32) -> Self {
        let dot = Q::dot(&q0, &q1);
        let q1 = if dot < 0.0 { -q1 } else { q1 };
        (q0 * (1.0 - t) + q1 * t).norm()
    }

    // ----------------------------------------------------------------------------
    // SLERP: spherical linear interpolation
    pub fn slerp(a: Self, b: Self, t: f32) -> Self {
        let mut b = b;
        let mut c = Q::dot(&a, &b);

        // Take shortest path
        if c < 0.0 {
            b = -b;
            c = -c;
        }

        // If nearly parallel, fall back to nlerp
        if c > 0.9995 {
            return Q::nlerp(a, b, t);
        }

        let th = c.acos();
        let s = th.sin();

        let w0 = ((1.0 - t) * th).sin() / s;
        let w1 = (t * th).sin() / s;

        a * w0 + b * w1
    }

    // ----------------------------------------------------------------------------
    // Convert to a 3×3 rotation matrix (column-major)
    pub fn as_mat3(&self) -> [[f32; 3]; 3] {
        let x2 = self.x() + self.x();
        let y2 = self.y() + self.y();
        let z2 = self.z() + self.z();

        let xx = self.x() * x2;
        let yy = self.y() * y2;
        let zz = self.z() * z2;

        let xy = self.x() * y2;
        let xz = self.x() * z2;
        let yz = self.y() * z2;

        let wx = self.w() * x2;
        let wy = self.w() * y2;
        let wz = self.w() * z2;

        [
            [1.0 - (yy + zz), xy + wz, xz - wy],
            [xy - wz, 1.0 - (xx + zz), yz + wx],
            [xz + wy, yz - wx, 1.0 - (xx + yy)],
        ]
    }

    // ------------------------------------------------------------------------
    // Rotate a vector
    pub fn rotate(&self, v: &V3) -> V3 {
        let qv = Q::new([v.x0(), v.x1(), v.x2(), 0.0]);
        let r = *self * qv * self.inverse();
        V3::new([r.x(), r.y(), r.z()])
    }

    // ------------------------------------------------------------------------
    pub fn from_axis_angle(axis: &V3, angle: f32) -> Self {
        let half = angle * 0.5;
        let s = half.sin();
        let c = half.cos();
        Q::new([axis.x0() * s, axis.x1() * s, axis.x2() * s, c])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_float_eq;
    use std::f32::consts::PI;

    #[test]
    fn test_axis_angle() {
        let axis = V3::new([1.0, 1.0, 1.0]).norm();
        let q = Q::from_axis_angle(&axis, PI);
        assert_float_eq!(q.length(), 1.0);
    }

    #[test]
    fn test_rotate_vector_axis_angle() {
        let axis = V3::new([0.0, 1.0, 0.0]);
        let v = V3::new([1.0, 1.0, 1.0]);
        let q = Q::from_axis_angle(&axis, PI);
        let r = q.rotate(&v);
        assert_eq!(r, V3::new([-1.0, 1.0, -1.0]));
    }

    /// Rotation on axis parallel to vector direction should have no effect
    #[test]
    fn test_rotate_vector_axis_angle_same_axis() {
        let v = V3::new([1.0, 1.0, 1.0]);
        let axis = V3::new([1.0, 1.0, 1.0]).norm();
        let q = Q::from_axis_angle(&axis, 31.41);
        let r = q.rotate(&v);
        assert_eq!(r, v);
    }

    // #[test]
    // fn test_rotation_from_to() {
    //     let a = V3::new([1.0, 1.0, 1.0]);
    //     let b = V3::new([-1.0, -1.0, -1.0]);
    //     let q = super::rotation_from_to(a, b);
    //     let a_prime = q.rotate(a);
    //     assert_eq!(a_prime, a);
    // }
}
