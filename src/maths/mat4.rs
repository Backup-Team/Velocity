use std::ops::{Index, Mul, MulAssign};

use crate::maths::{Angle, Point3, Quat, Scale3, Vec3};

const MATRIX_WIDTH: usize = 4;
const MATRIX_SIZE: usize = MATRIX_WIDTH * MATRIX_WIDTH;

pub struct Mat4(pub(in crate::maths) [f32; MATRIX_SIZE]);

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
    pub fn translation(translation: &Point3) -> Self {
        let Point3 { x, y, z } = translation;

        Self([
            1.0, 0.0, 0.0, *x,
            0.0, 1.0, 0.0, *y,
            0.0, 0.0, 1.0, *z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn rotation(rotation: &Quat) -> Self {
        let Vec3 { x, y, z } = Vec3::unit_x().rotate(&rotation);

        Self([
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn scale(scale: &Scale3) -> Self {
        let Scale3 { w, h, d } = scale;

        Self([
            *w , 0.0, 0.0, 0.0,
            0.0, *h , 0.0, 0.0,
            0.0, 0.0, *d , 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn perspective(aspect_ratio: f32, field_of_view: Angle, near: f32, far: f32) -> Self {
        let tan_half_fov = field_of_view.into_radians().tan() * 0.5;

        let x_scale = (tan_half_fov * aspect_ratio).recip();
        let y_scale = tan_half_fov.recip();
        let z_scale = (-near - far) / (near - far);

        let z_translation = 2.0 * near * far / (near - far);

        Self([
            x_scale, 0.0    , 0.0    , 0.0,
            0.0    , y_scale, 0.0    , 0.0,
            0.0    , 0.0    , z_scale, z_translation,
            0.0    , 0.0    , 1.0    , 0.0,
        ])
    }

    #[rustfmt::skip]
    pub fn orthographic(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
        let r_l = right - left;
        let t_b = top - bottom;
        let f_n = far - near;

        let x_scale =  2.0 / r_l;
        let y_scale =  2.0 / t_b;
        let z_scale = -2.0 / f_n;

        let x_translation = -((right + left)   / r_l);
        let y_translation = -((top   + bottom) / t_b);
        let z_translation = -((far   + near)   / f_n);

        Self([
            x_scale, 0.0    , 0.0    , x_translation,
            0.0    , y_scale, 0.0    , y_translation,
            0.0    , 0.0    , z_scale, z_translation,
            0.0    , 0.0    , 0.0    , 1.0,
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
             right   .x,  right   .y,  right   .z, 0.0,
             up      .x,  up      .y,  up      .z, 0.0,
             forward .x,  forward .y,  forward .z, 0.0,
            -position.x, -position.y, -position.z, 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn determinant(&self) -> f32 {
        // The determinant of a matrix is the sum of the products of the elements of any one row or
        // column and their cofactors. Therefore if all elements in any row or column are 0, then
        // the determinant is 0. This also means that the fastest way to calculate the determinant
        // is to use the row or column with the most 0s in it. Unfortunately we don't have the
        // luxury of time required to figure that out.

        let Self([
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33,
        ]) = *self;

        let c0 = m00 * Vec3::new(m11, m12, m13).cross(&Vec3::new(m21, m22, m23)).dot(&Vec3::new(m31, m32, m33));
        let c1 = m01 * Vec3::new(m10, m12, m13).cross(&Vec3::new(m20, m22, m23)).dot(&Vec3::new(m30, m32, m33));
        let c2 = m02 * Vec3::new(m10, m11, m13).cross(&Vec3::new(m20, m21, m23)).dot(&Vec3::new(m30, m31, m33));
        let c3 = m03 * Vec3::new(m10, m11, m12).cross(&Vec3::new(m20, m21, m22)).dot(&Vec3::new(m30, m31, m32));

        c0 - c1 + c2 - c3
    }

    pub fn inverse(&self) -> Mat4 {
        Self::identity()
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Index<usize> for Mat4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
