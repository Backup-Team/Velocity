use bytemuck::{Pod, Zeroable};

use crate::maths::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Bounds3 {
    centre: Vec3,
    half_w: f32,
    half_h: f32,
    half_d: f32,
}

impl Bounds3 {
    pub const fn new(centre: Vec3, half_w: f32, half_h: f32, half_d: f32) -> Self {
        Self {
            centre,
            half_w,
            half_h,
            half_d,
        }
    }

    #[rustfmt::skip]
    pub fn intersects(&self, rhs: &Bounds3) -> bool {
           f32::abs(self.centre.x - rhs.centre.x) <= self.half_w + rhs.half_w
        && f32::abs(self.centre.y - rhs.centre.y) <= self.half_h + rhs.half_h
        && f32::abs(self.centre.z - rhs.centre.z) <= self.half_d + rhs.half_d
    }

    #[inline(always)]
    pub fn min(&self) -> Vec3 {
        self.centre - Vec3::new(self.half_w, self.half_h, self.half_d)
    }

    #[inline(always)]
    pub fn max(&self) -> Vec3 {
        self.centre + Vec3::new(self.half_w, self.half_h, self.half_d)
    }
}
