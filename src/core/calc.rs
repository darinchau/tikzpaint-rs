mod hiya;

pub const EPS: f64 = 1e-10;

#[inline(always)]
pub fn abs(s: f64) -> f64 {
    s.abs()
}

#[inline(always)]
pub fn eq(a: &f64, b: &f64) -> bool {
    abs(a - b) <= EPS
}

#[inline(always)]
pub fn is_zero(a: f64) -> bool {
    abs(a) < EPS
}