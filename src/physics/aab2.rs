use bytemuck::{Pod, Zeroable};

use crate::core::maths::{Point2, Scale2, Vec2};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct AxisAlignedBox3 {
    centre: Vec2,
    scale:  Scale2,
}

impl AxisAlignedBox3 {
    pub const fn new(centre: Vec2, scale: Scale2) -> Self {
        Self { centre, scale }
    }

    #[rustfmt::skip]
    pub fn intersects(&self, rhs: &AxisAlignedBox3) -> bool {
           f32::abs(self.centre.x - rhs.centre.x) <= self.scale.w + rhs.scale.w
        && f32::abs(self.centre.y - rhs.centre.y) <= self.scale.h + rhs.scale.h
    }

    #[rustfmt::skip]
    pub fn contains_point(&self, rhs: &Point2) -> bool {
           f32::abs(self.centre.x - rhs.x) <= self.scale.w
        && f32::abs(self.centre.y - rhs.y) <= self.scale.h
    }

    #[inline(always)]
    pub fn min(&self) -> Vec2 {
        self.centre - Vec2::new(self.scale.w, self.scale.h)
    }

    #[inline(always)]
    pub fn max(&self) -> Vec2 {
        self.centre + Vec2::new(self.scale.w, self.scale.h)
    }
}
