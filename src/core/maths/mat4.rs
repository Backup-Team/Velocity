use std::{
    borrow::Borrow,
    ops::{Index, Mul, MulAssign},
};

use crate::core::maths::{Angle, Point3, Quat, Scale3, Vec3};

const MATRIX_WIDTH: usize = 4;
const MATRIX_SIZE: usize = MATRIX_WIDTH * MATRIX_WIDTH;

#[repr(C)]
pub struct Mat4(pub(in crate::core::maths) [f32; MATRIX_SIZE]);

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
    pub fn translation(translation: &Vec3) -> Self {
        let Vec3 { x, y, z } = translation.borrow();

        Self([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            *x , *y , *z , 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn rotation(rotation: &Quat) -> Self {
        let Quat { v: Vec3 { x, y, z }, w} = *rotation.borrow();

        let two_x = 2.0 * x;
        let two_y = 2.0 * y;
        let two_z = 2.0 * z;

        let two_x_squared = two_x.powf(2.0);
        let two_y_squared = two_y.powf(2.0);
        let two_z_squared = two_z.powf(2.0);

        let scale_x = 1.0 - two_y_squared - two_z_squared;
        let scale_y = 1.0 - two_x_squared - two_z_squared;
        let scale_z = 1.0 - two_x_squared - two_y_squared;

        let two_x_y = two_x * y;
        let two_x_z = two_x * y;
        let two_y_z = two_y * z;
        let two_x_w = two_x * w;
        let two_y_w = two_y * w;
        let two_z_w = two_z * w;

        let m01 = two_x_y + two_z_w;
        let m02 = two_x_z - two_y_w;
        let m10 = two_x_y - two_z_w;
        let m12 = two_y_z + two_x_w;
        let m20 = two_x_z + two_y_w;
        let m21 = two_y_z - two_x_w;

        Self([
            scale_x, m01    , m02    , 0.0,
            m10    , scale_y, m12    , 0.0,
            m20    , m21    , scale_z, 0.0,
            0.0    , 0.0    , 0.0    , 1.0,
        ])
    }

    #[rustfmt::skip]
    pub fn scale(scale: &Scale3) -> Self {
        let Scale3 { w, h, d } = scale.borrow();

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
            x_scale, 0.0    , 0.0          , 0.0,
            0.0    , y_scale, 0.0          , 0.0,
            0.0    , 0.0    , z_scale      , 1.0,
            0.0    , 0.0    , z_translation, 0.0,
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
            x_scale      , 0.0          , 0.0          , 0.0,
            0.0          , y_scale      , 0.0          , 0.0,
            0.0          , 0.0          , z_scale      , 0.0,
            x_translation, y_translation, z_translation, 1.0,
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

    fn row(&self, index: usize) -> &[f32] {
        &[
            self.0[index + 0],
            self.0[index + 4],
            self.0[index + 8],
            self.0[index + 12],
        ]
    }

    #[inline(always)]
    pub fn col(&self, index: usize) -> &[f32] {
        &self.0[index..index + 4]
    }

    // fn row(&self, index: usize) -> (f32, f32, f32, f32) {
    //     (
    //         self.0[index + 0],
    //         self.0[index + 4],
    //         self.0[index + 8],
    //         self.0[index + 12],
    //     )
    // }

    #[inline(always)]
    pub fn row_0(&self) -> &[f32] {
        self.row(0)
    }

    // #[inline(always)]
    // pub fn row_0(&self) -> (f32, f32, f32, f32) {
    //     self.row(0)
    // }

    // #[inline(always)]
    // pub fn row_1(&self) -> (f32, f32, f32, f32) {
    //     self.row(1)
    // }

    // #[inline(always)]
    // pub fn row_2(&self) -> (f32, f32, f32, f32) {
    //     self.row(2)
    // }

    // #[inline(always)]
    // pub fn row_3(&self) -> (f32, f32, f32, f32) {
    //     self.row(3)
    // }

    #[inline(always)]
    pub fn col_0(&self, index: usize) -> &[f32] {
        &self.0[0..4]
    }

    #[inline(always)]
    pub fn col_1(&self, index: usize) -> &[f32] {
        &self.0[4..8]
    }

    #[inline(always)]
    pub fn col_2(&self, index: usize) -> &[f32] {
        &self.0[8..12]
    }

    #[inline(always)]
    pub fn col_3(&self, index: usize) -> &[f32] {
        &self.0[12..16]
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

    #[rustfmt::skip]
    pub fn inverse(&self) -> Self {
        let Self([
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33,
        ]) = *self;

        // Upper 2D matrix determinants.
        let b00 = m00 * m11 - m01 * m10;
        let b01 = m00 * m12 - m02 * m10;
        let b02 = m00 * m13 - m03 * m10;
        let b03 = m01 * m12 - m02 * m11;
        let b04 = m01 * m13 - m03 * m11;
        let b05 = m02 * m13 - m03 * m12;

        // Lower 2D matrix determinants.
        let b06 = m20 * m31 - m21 * m30;
        let b07 = m20 * m32 - m22 * m30;
        let b08 = m20 * m33 - m23 * m30;
        let b09 = m21 * m32 - m22 * m31;
        let b10 = m21 * m33 - m23 * m31;
        let b11 = m22 * m33 - m23 * m32;

        let determinant = b00 * b11
                        - b01 * b10
                        + b02 * b09
                        + b03 * b08
                        - b04 * b07
                        + b05 * b06;

        let reciprocal = determinant.recip();

        let data = [
            (m11 * b11 - m12 * b10 + m13 * b09) * reciprocal,
            (m02 * b10 - m01 * b11 - m03 * b09) * reciprocal,
            (m31 * b05 - m32 * b04 + m33 * b03) * reciprocal,
            (m22 * b04 - m21 * b05 - m23 * b03) * reciprocal,
            (m12 * b08 - m10 * b11 - m13 * b07) * reciprocal,
            (m00 * b11 - m02 * b08 + m03 * b07) * reciprocal,
            (m32 * b02 - m30 * b05 - m33 * b01) * reciprocal,
            (m20 * b05 - m22 * b02 + m23 * b01) * reciprocal,
            (m10 * b10 - m11 * b08 + m13 * b06) * reciprocal,
            (m01 * b08 - m00 * b10 - m03 * b06) * reciprocal,
            (m30 * b04 - m31 * b02 + m33 * b00) * reciprocal,
            (m21 * b02 - m20 * b04 - m23 * b00) * reciprocal,
            (m11 * b07 - m10 * b09 - m12 * b06) * reciprocal,
            (m00 * b09 - m01 * b07 + m02 * b06) * reciprocal,
            (m31 * b01 - m30 * b03 - m32 * b00) * reciprocal,
            (m20 * b03 - m21 * b01 + m22 * b00) * reciprocal,
        ];

        Self(data)
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= &rhs;
        self
    }
}

impl<M> MulAssign<M> for Mat4
where
    M: Borrow<Mat4>,
{
    #[rustfmt::skip]
    fn mul_assign(&mut self, rhs: M) {
        let rhs = rhs.borrow();

        for i in 0..MATRIX_WIDTH {
            let row = self.row(i);

            for j in 0..MATRIX_WIDTH {
                let col = rhs.col(j);

                self.0[(j * MATRIX_WIDTH) + i]
                    = col[0] * row[0]
                    + col[1] * row[1]
                    + col[2] * row[2]
                    + col[3] * row[3];
            }


            // let m0i = self.0[i + 0];
            // let m1i = self.0[i + 4];
            // let m2i = self.0[i + 8];
            // let m3i = self.0[i + 12];

            // for j in (0..MATRIX_SIZE).step_by(4) {
            //     self.0[j + i] = rhs.0[j + 0] * m0i
            //                   + rhs.0[j + 1] * m1i
            //                   + rhs.0[j + 2] * m2i
            //                   + rhs.0[j + 3] * m3i;
            // }
        }
    }
}

impl Index<usize> for Mat4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
