use bytemuck::{Pod, Zeroable};

use crate::{
    maths::{Mat4, Normed, Quat},
    scalar_maths,
    vec_maths,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(value: f32) -> Self {
        Self::new(value, value, value)
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
        Self::new(1.0, 0.0, 0.0)
    }

    pub const fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub const fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub const fn positive_infinity() -> Self {
        Self::splat(f32::INFINITY)
    }

    pub const fn negative_infinity() -> Self {
        Self::splat(f32::NEG_INFINITY)
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn rotate(&self, quat: &Quat) -> Self {
        // The below code is an optimised form of:
        // (quat * Quat::new(0.0, *self) * quat.inverse()).v
        let cross = quat.v.cross(self);
        quat.v + cross * (2.0 * quat.w) + quat.v.cross(&cross) * 2.0
    }

    pub fn rotate_by(&mut self, quat: &Quat) {
        *self = self.rotate(quat);
    }

    pub fn transform(&self, matrix: &Mat4) -> Self {
        let x = self.x * matrix[0] + self.y * matrix[4] + self.z * matrix[8] + matrix[12];
        let y = self.x * matrix[1] + self.y * matrix[5] + self.z * matrix[9] + matrix[13];
        let z = self.x * matrix[2] + self.y * matrix[6] + self.z * matrix[10] + matrix[14];

        Self { x, y, z }
    }

    pub fn transform_by(&mut self, matrix: &Mat4) {
        *self = self.transform(matrix);
    }
}

impl Normed for Vec3 {
    fn norm_squared(&self) -> f32 {
        self.dot(self)
    }
}

vec_maths!(
    Vec3{ x y z } (Add add) (AddAssign add_assign) +=,
    Vec3{ x y z } (Sub sub) (SubAssign sub_assign) -=,
);

scalar_maths!(
    Vec3{ x y z } (Mul mul) (MulAssign mul_assign) *=,
    Vec3{ x y z } (Div div) (DivAssign div_assign) /=,
);

impl From<[f32; 3]> for Vec3 {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Self { x, y, z }
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
}
