use std::borrow::Borrow;

use bytemuck::{Pod, Zeroable};

use crate::core::maths::{fuzzy, Angle, Mat4, Normed, Unit};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn lerp(start: &Self, end: &Self, t: f32) -> Unit<Self> {
        let mid = (1.0 - t) * start + t * end;
        let mid = mid / mid.norm();

        Unit::new(mid)
    }

    pub const fn splat(value: f32) -> Self {
        Self::new(value, value)
    }

    pub const fn zero() -> Self {
        Self::splat(0.0)
    }

    pub const fn positive_one() -> Self {
        Self::splat(1.0)
    }

    pub const fn negative_one() -> Self {
        Self::splat(-1.0)
    }

    pub const fn unit_x() -> Self {
        Self::new(1.0, 0.0)
    }

    pub const fn unit_y() -> Self {
        Self::new(0.0, 1.0)
    }

    pub const fn positive_infinity() -> Self {
        Self::splat(f32::INFINITY)
    }

    pub const fn negative_infinity() -> Self {
        Self::splat(f32::NEG_INFINITY)
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        let rhs = rhs.borrow();

        self.x * rhs.x + self.y * rhs.y
    }

    pub fn reciprocal(&self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
        }
    }

    pub fn left_perpendicular(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    pub fn left_perpendicular_dot(&self, rhs: &Self) -> f32 {
        self.left_perpendicular().dot(rhs)
    }

    pub fn right_perpendicular(&self) -> Self {
        Self::new(self.y, -self.x)
    }

    pub fn right_perpendicular_dot(&self, rhs: &Self) -> f32 {
        self.right_perpendicular().dot(rhs)
    }

    pub fn rotate(&self, angle: Angle) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    pub fn rotate_by(&mut self, angle: Angle) {
        *self = self.rotate(angle)
    }

    pub fn transform(&self, matrix: &Mat4) -> Self {
        let matrix = matrix.borrow();

        let x = self.x * matrix[0] + self.y * matrix[4] + matrix[12];
        let y = self.x * matrix[1] + self.y * matrix[5] + matrix[13];

        Self { x, y }
    }

    pub fn transform_by(&mut self, matrix: &Mat4) {
        *self = self.transform(matrix);
    }
}

impl Normed for Vec2 {
    fn norm_squared(&self) -> f32 {
        self.dot(self)
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from([x, y]: [f32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl<V> From<V> for Unit<Vec2>
where
    V: Borrow<Vec2>,
{
    fn from(vec: V) -> Self {
        Self::new(*vec.borrow())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::maths::Vec3;

    use super::*;

    #[test]
    fn dot() {
        assert_eq!(Vec2::unit_x().dot(&Vec2::unit_x()), 1.0);
        assert_eq!(Vec2::unit_x().dot(&Vec2::unit_y()), 0.0);
        assert_eq!(Vec2::unit_y().dot(&Vec2::unit_x()), 0.0);
        assert_eq!(Vec2::unit_x().dot(&-Vec2::unit_x()), -1.0);
    }

    #[test]
    fn reciprocal_of_zero_is_infinity() {
        let Vec2 { x, y } = Vec2::zero().reciprocal();

        assert_eq!(x, f32::INFINITY);
        assert_eq!(y, f32::INFINITY);
    }

    #[test]
    fn reciprocal() {
        let Vec2 { x, y } = Vec2::new(3.0, 4.0).reciprocal();

        assert_eq!(x, (3.0f32).recip());
        assert_eq!(y, (4.0f32).recip());
    }

    #[test]
    fn left_perpendicular() {
        let Vec2 { x, y } = Vec2::unit_x().left_perpendicular();

        assert_eq!(x, -0.0);
        assert_eq!(y, 1.0);
    }

    #[test]
    fn left_perpendicular_dot() {
        assert_eq!(Vec2::unit_x().left_perpendicular_dot(&Vec2::unit_x()), 0.0);
        assert_eq!(Vec2::unit_x().left_perpendicular_dot(&Vec2::unit_y()), 1.0);
        assert_eq!(Vec2::unit_y().left_perpendicular_dot(&Vec2::unit_x()), -1.0);
        assert_eq!(Vec2::unit_x().left_perpendicular_dot(&-Vec2::unit_x()), 0.0);
    }

    #[test]
    fn right_perpendicular() {
        let Vec2 { x, y } = Vec2::unit_x().right_perpendicular();

        assert_eq!(x, 0.0);
        assert_eq!(y, -1.0);
    }

    #[test]
    fn right_perpendicular_dot() {
        assert_eq!(Vec2::unit_x().right_perpendicular_dot(&Vec2::unit_x()), 0.0);
        assert_eq!(
            Vec2::unit_x().right_perpendicular_dot(&Vec2::unit_y()),
            -1.0
        );
        assert_eq!(Vec2::unit_y().right_perpendicular_dot(&Vec2::unit_x()), 1.0);
        assert_eq!(
            Vec2::unit_x().right_perpendicular_dot(&-Vec2::unit_x()),
            0.0
        );
    }

    #[test]
    fn rotate() {
        let Vec2 { x, y } = Vec2::unit_x().rotate(Angle::radians(std::f32::consts::PI));

        assert!(fuzzy::eq(x, -1.0));
        assert!(fuzzy::eq(y, -0.0));
    }

    #[test]
    fn transform() {
        let Vec2 { x, y } = Vec2::unit_x().transform(&Mat4::translation(&Vec3::new(1.0, 2.0, 0.0)));

        assert_eq!(x, 2.0);
        assert_eq!(y, 2.0);
    }
}
