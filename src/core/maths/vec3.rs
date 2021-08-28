use std::borrow::Borrow;

use bytemuck::{Pod, Zeroable};

use crate::core::maths::{fuzzy, Mat4, Normed, Point3, Quat, Unit};

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

    pub fn lerp(start: &Self, end: &Self, t: f32) -> Unit<Self> {
        let mid = (1.0 - t) * start + t * end;
        let mid = mid / mid.norm();

        Unit::new(mid)
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

    pub const fn unit_x() -> Unit<Self> {
        Unit::from_normalised(Self::new(1.0, 0.0, 0.0))
    }

    pub const fn unit_y() -> Unit<Self> {
        Unit::from_normalised(Self::new(0.0, 1.0, 0.0))
    }

    pub const fn unit_z() -> Unit<Self> {
        Unit::from_normalised(Self::new(0.0, 0.0, 1.0))
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

    pub fn reciprocal(&self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
            z: self.z.recip(),
        }
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn rotate(&self, quat: &Quat) -> Self {
        (quat * Quat::new(0.0, *self) * quat.inverse()).v
    }

    pub fn rotate_by(&mut self, quat: &Quat) {
        *self = self.rotate(quat);
    }

    pub fn transform(&self, matrix: &Mat4) -> Self {
        let matrix = matrix.borrow();

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

impl<P> From<P> for Vec3
where
    P: Borrow<Point3>,
{
    fn from(point: P) -> Self {
        let Point3 { x, y, z } = *point.borrow();

        Self { x, y, z }
    }
}

impl<V> From<V> for Unit<Vec3>
where
    V: Borrow<Vec3>,
{
    fn from(vec: V) -> Self {
        Self::new(*vec.borrow())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::maths::Angle;

    use super::*;

    #[test]
    fn dot() {
        assert_eq!(Vec3::unit_x().dot(&Vec3::unit_x()), 1.0);
        assert_eq!(Vec3::unit_x().dot(&Vec3::unit_y()), 0.0);
        assert_eq!(Vec3::unit_y().dot(&Vec3::unit_x()), 0.0);
        assert_eq!(Vec3::unit_x().dot(&-Vec3::unit_x()), -1.0);
    }

    #[test]
    fn reciprocal_of_zero_is_infinity() {
        let Vec3 { x, y, z } = Vec3::zero().reciprocal();

        assert_eq!(x, f32::INFINITY);
        assert_eq!(y, f32::INFINITY);
        assert_eq!(z, f32::INFINITY);
    }

    #[test]
    fn reciprocal() {
        let Vec3 { x, y, z } = Vec3::new(3.0, 4.0, 5.0).reciprocal();

        assert_eq!(x, (3.0f32).recip());
        assert_eq!(y, (4.0f32).recip());
        assert_eq!(z, (5.0f32).recip());
    }

    #[test]
    fn cross() {
        assert!(Vec3::unit_x().cross(&Vec3::unit_y()) == Vec3::unit_z());
    }

    #[test]
    fn rotate() {
        assert_eq!(
            Vec3::unit_x().rotate(&Quat::from_axis_angle(
                &Vec3::unit_y(),
                Angle::radians(std::f32::consts::PI),
            )),
            Vec3::new(-1.0, 0.0, -0.0)
        );
    }

    #[test]
    fn transform() {
        let Vec3 { x, y, z } =
            Vec3::unit_x().transform(&Mat4::translation(&Vec3::new(1.0, 2.0, 0.0)));

        assert_eq!(x, 2.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 0.0);
    }
}
