use std::ops::{Add, AddAssign};

use bytemuck::{Pod, Zeroable};

use crate::maths::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(value: f32) -> Self {
        Self::new(value, value, value)
    }

    pub const fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub const fn vector_from_origin(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn vector_from(&self, rhs: &Self) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }

    pub fn vector_to(&self, rhs: &Self) -> Vec3 {
        Vec3::new(rhs.x - self.x, rhs.y - self.y, rhs.z - self.z)
    }
}

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(mut self, rhs: Vec3) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl From<[f32; 3]> for Point3 {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Self { x, y, z }
    }
}

impl From<(f32, f32, f32)> for Point3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
}
