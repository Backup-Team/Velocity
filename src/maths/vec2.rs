use bytemuck::{Pod, Zeroable};

use crate::{
    maths::{Angle, Mat4, Normed},
    negate,
    scalar_maths,
    vec_maths,
};

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
        self.x * rhs.x + self.y * rhs.y
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

vec_maths!(
    Vec2{ x y } (Add add) (AddAssign add_assign) +=,
    Vec2{ x y } (Sub sub) (SubAssign sub_assign) -=,
);

scalar_maths!(
    Vec2{ x y } (Mul mul) (MulAssign mul_assign) *=,
    Vec2{ x y } (Div div) (DivAssign div_assign) /=,
);

negate!(Vec2{ x y },);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot() {
        assert_eq!(Vec2::unit_x().dot(&Vec2::unit_x()), 1.0);
        assert_eq!(Vec2::unit_x().dot(&Vec2::unit_y()), 0.0);
        assert_eq!(Vec2::unit_y().dot(&Vec2::unit_x()), 0.0);
        assert_eq!(Vec2::unit_x().dot(&-Vec2::unit_x()), -1.0);
    }
}
