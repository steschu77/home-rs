#[inline]
pub fn float_eq_ulps(lhs: f32, rhs: f32) -> bool {
    if lhs.is_sign_positive() != rhs.is_sign_positive() {
        lhs == rhs // +0.0 == -0.0
    } else {
        let bits0 = lhs.to_bits();
        let bits1 = rhs.to_bits();
        let ulps = bits0.abs_diff(bits1);
        ulps <= 4
    }
}

#[inline]
pub fn float_eq_rel(lhs: f32, rhs: f32) -> bool {
    let diff = (lhs - rhs).abs();
    diff <= 1.0e-6 || float_eq_ulps(lhs, rhs)
}

#[macro_export]
macro_rules! assert_float_eq {
    ($lhs:expr, $rhs:expr) => {
        assert!(float_eq_rel($lhs, $rhs), "{:?} != {:?}", $lhs, $rhs);
    };
}
