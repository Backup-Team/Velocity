use std::ops::{Add, AddAssign};

use bytemuck::{Pod, Zeroable};

use crate::core::maths::Vec2;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl Point2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn splat(value: f32) -> Self {
        Self::new(value, value)
    }

    pub const fn identity() -> Self {
        Self::new(0.0, 0.0)
    }

    pub const fn vector_from_origin(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn vector_from(&self, rhs: &Self) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }

    pub fn vector_to(&self, rhs: &Self) -> Vec2 {
        Vec2::new(rhs.x - self.x, rhs.y - self.y)
    }
}

impl Add<Vec2> for Point2 {
    type Output = Point2;

    fn add(mut self, rhs: Vec2) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<Vec2> for Point2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<[f32; 2]> for Point2 {
    fn from([x, y]: [f32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for Point2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}
