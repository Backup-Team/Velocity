use crate::maths::Vec3;

use super::Point3;

pub struct Mat4([f32; 16]);

pub struct Camera {
    pub position: Vec3,
    pub right:    Vec3,
    pub up:       Vec3,
    pub forward:  Vec3,
}

impl Mat4 {
    #[rustfmt::skip]
    pub fn identity() -> Self {
        Self([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn translation(Vec3 { x, y, z }: Vec3) -> Self {
        Self([
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn scale(Vec3 { x, y, z }: Vec3) -> Self {
        Self([
            x  , 0.0, 0.0, 0.0,
            0.0, y  , 0.0, 0.0,
            0.0, 0.0, z  , 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn perspective() -> Self {
        Self([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn orthographic(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
        let r_l = right - left;
        let t_b = top - bottom;
        let f_n = far - near;

        Self([
            2.0 / r_l, 0.0      , 0.0       , -((right + left) / r_l),
            0.0      , 2.0 / t_b, 0.0       , -((top + bottom) / t_b),
            0.0      , 0.0      , -2.0 / f_n, -((far + near) / f_n),
            0.0      , 0.0      , 0.0       , 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn look_at(camera: &Camera, target: &Vec3) -> Self {
        // let z_axis = < hector done goofed


        Self([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn view(forward: Vec3, up: Vec3, right: Vec3, position: Point3) -> Self {


        Self([
            right   .x, right   .y, right   .z, 0.0,
            up      .x, up      .y, up      .z, 0.0,
            forward .x, forward .y, forward .z, 0.0,
            position.x, position.y, position.z, 1.0,
        ])
    }
}
