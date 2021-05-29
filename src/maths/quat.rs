use bytemuck::{Pod, Zeroable};

use crate::{
    maths::{Angle, Normed, Unit, Vec3},
    scalar_maths,
    vec_maths,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Quat {
    pub v: Vec3,
    pub w: f32,
}

impl Quat {
    pub const fn identity() -> Self {
        Self {
            w: 1.0,
            v: Vec3::zero(),
        }
    }

    pub const fn new(w: f32, v: Vec3) -> Self {
        Self { w, v }
    }

    pub fn from_axis_angle(axis: Unit<Vec3>, angle: Angle) -> Self {
        let (half_sin, half_cos) = (angle * 0.5).sin_cos();

        Self {
            w: half_sin,
            v: *axis * half_cos,
        }
    }

    // pub fn from_pitch_yaw_roll(pitch: f32, yaw: f32, roll: f32) -> Self {}

    pub fn conjugate(&self) -> Self {
        Self {
            v: self.v * -1.0,
            w: self.w,
        }
    }

    pub fn inverse(&self) -> Self {
        self.conjugate() * self.norm_squared().recip()
    }
}

impl Normed for Quat {
    fn norm_squared(&self) -> f32 {
        let v = self.v.dot(&self.v);
        let w = self.w * self.w;

        v + w
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign for Quat {
    fn mul_assign(&mut self, rhs: Self) {
        let scalar = self.w * rhs.w - self.v.dot(&rhs.v);
        let imaginary = self.v * rhs.w + rhs.v * self.w + self.v.cross(&rhs.v);

        self.w = scalar;
        self.v = imaginary;
    }
}

impl Mul<&Self> for Quat {
    type Output = Self;

    fn mul(mut self, rhs: &Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<&Self> for Quat {
    fn mul_assign(&mut self, rhs: &Self) {
        let scalar = self.w * rhs.w - self.v.dot(&rhs.v);
        let imaginary = rhs.v * self.w + self.v * rhs.w + self.v.cross(&rhs.v);

        self.w = scalar;
        self.v = imaginary;
    }
}

vec_maths!(
    Quat{ v w } (Add add) (AddAssign add_assign) +=,
    Quat{ v w } (Sub sub) (SubAssign sub_assign) -=,
);

scalar_maths!(
    Quat{ v w } (Mul mul) (MulAssign mul_assign) *=,
    Quat{ v w } (Div div) (DivAssign div_assign) /=,
);

impl From<[f32; 4]> for Quat {
    fn from([x, y, z, w]: [f32; 4]) -> Self {
        Self {
            v: Vec3 { x, y, z },
            w,
        }
    }
}

impl From<(f32, f32, f32, f32)> for Quat {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Self {
            v: Vec3 { x, y, z },
            w,
        }
    }
}
