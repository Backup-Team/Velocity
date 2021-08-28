use crate::core::maths::{Mat4, Quat, Scale3, Vec3};

pub struct Transform {
    pub translation: Vec3,
    pub rotation:    Quat,
    pub scale:       Scale3,
}

impl Transform {
    pub fn transformation(&self) -> Mat4 {
        let Transform {
            translation,
            rotation,
            scale,
        } = self;

        Mat4::translation(translation) * Mat4::scale(scale) * Mat4::rotation(rotation)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::zero(),
            rotation:    Quat::identity(),
            scale:       Scale3::identity(),
        }
    }
}
