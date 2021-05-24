use core::f32::EPSILON;

// TODO:
// Find a good default for this.
pub(crate) const DEFAULT_EPSILON: f32 = 1e-10;

#[inline(always)]
pub fn eq_with_epsilon(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() <= epsilon + EPSILON
}

#[inline(always)]
pub fn gt_with_epsilon(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b) > 0.0 && !eq_with_epsilon(a, b, epsilon)
}

#[inline(always)]
pub fn gte_with_epsilon(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b) > 0.0 || eq_with_epsilon(a, b, epsilon)
}
#[inline(always)]
pub fn lt_with_epsilon(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b) < 0.0 && !eq_with_epsilon(a, b, epsilon)
}

#[inline(always)]
pub fn lte_with_epsilon(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b) < 0.0 || eq_with_epsilon(a, b, epsilon)
}

#[inline(always)]
pub fn eq(a: f32, b: f32) -> bool {
    eq_with_epsilon(a, b, DEFAULT_EPSILON)
}

#[inline(always)]
pub fn gt(a: f32, b: f32) -> bool {
    gt_with_epsilon(a, b, DEFAULT_EPSILON)
}

#[inline(always)]
pub fn gte(a: f32, b: f32) -> bool {
    gte_with_epsilon(a, b, DEFAULT_EPSILON)
}

#[inline(always)]
pub fn lt(a: f32, b: f32) -> bool {
    lt_with_epsilon(a, b, DEFAULT_EPSILON)
}

#[inline(always)]
pub fn lte(a: f32, b: f32) -> bool {
    lte_with_epsilon(a, b, DEFAULT_EPSILON)
}
