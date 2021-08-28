use bytemuck::{Pod, Zeroable};

use crate::core::maths::{Point2, Point3, Scale3, Vec3};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct AxisAlignedBox3 {
    centre: Vec3,
    scale:  Scale3,
}

impl AxisAlignedBox3 {
    pub const fn new(centre: Vec3, scale: Scale3) -> Self {
        Self { centre, scale }
    }

    #[rustfmt::skip]
    pub fn intersects(&self, rhs: &AxisAlignedBox3) -> bool {
           f32::abs(self.centre.x - rhs.centre.x) <= self.scale.w + rhs.scale.w
        && f32::abs(self.centre.y - rhs.centre.y) <= self.scale.h + rhs.scale.h
        && f32::abs(self.centre.z - rhs.centre.z) <= self.scale.d + rhs.scale.d
    }

    #[rustfmt::skip]
    pub fn contains_point(&self, rhs: &Point3) -> bool {
           f32::abs(self.centre.x - rhs.x) <= self.scale.w
        && f32::abs(self.centre.y - rhs.y) <= self.scale.h
        && f32::abs(self.centre.z - rhs.z) <= self.scale.d
    }

    #[rustfmt::skip]
    pub fn contains_point_xy(&self, rhs: &Point2) -> bool {
           f32::abs(self.centre.x - rhs.x) <= self.scale.w
        && f32::abs(self.centre.y - rhs.y) <= self.scale.h
    }

    #[inline(always)]
    pub fn min(&self) -> Vec3 {
        self.centre - Vec3::new(self.scale.w, self.scale.h, self.scale.d)
    }

    #[inline(always)]
    pub fn max(&self) -> Vec3 {
        self.centre + Vec3::new(self.scale.w, self.scale.h, self.scale.d)
    }
}
