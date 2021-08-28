use std::{
    borrow::Borrow,
    ops::{Mul, MulAssign},
};

use bytemuck::{Pod, Zeroable};

use crate::core::maths::{Angle, Normed, Unit, Vec3};

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

    pub fn lerp(start: &Self, end: &Self, t: f32) -> Unit<Self> {
        let mid = (1.0 - t) * start + t * end;
        let mid = mid / mid.norm();

        Unit::new(mid)
    }

    pub fn from_axis_angle(axis: &Unit<Vec3>, angle: Angle) -> Self {
        let (half_sin, half_cos) = (angle * 0.5).sin_cos();

        Self {
            w: half_cos,
            v: half_sin * axis,
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
        // OPTIMISE:
        // Since all 3D rotations are represented with unit length quaternions we can assume that:
        // norm_squared() is 1 -> 1.recip() is 1/1 which is 1 -> conjugate() * 1 is conjugate().
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

macro_rules! quaternion {
    (standard $lhs:ty, $rhs:ty) => {
        impl Mul<$rhs> for $lhs {
            type Output = Quat;

            fn mul(self, rhs: $rhs) -> Self::Output {
                let scalar = self.w * rhs.w - self.v.dot(&rhs.v);
                let imaginary = self.w * rhs.v + rhs.w * self.v + self.v.cross(&rhs.v);

                Self::Output {
                    w: scalar,
                    v: imaginary,
                }
            }
        }
    };

    (assign $lhs:ty, $rhs:ty) => {
        impl MulAssign<$rhs> for $lhs {
            fn mul_assign(&mut self, rhs: $rhs) {
                let scalar = self.w * rhs.w - self.v.dot(&rhs.v);
                let imaginary = self.w * rhs.v + rhs.w * self.v + self.v.cross(&rhs.v);

                self.w = scalar;
                self.v = imaginary;
            }
        }
    };

    () => {
        quaternion!(standard      Quat,      Quat);
        quaternion!(standard      Quat, &    Quat);
        quaternion!(standard      Quat, &mut Quat);
        quaternion!(standard &    Quat,      Quat);
        quaternion!(standard &    Quat, &    Quat);
        quaternion!(standard &    Quat, &mut Quat);
        quaternion!(standard &mut Quat,      Quat);
        quaternion!(standard &mut Quat, &    Quat);
        quaternion!(standard &mut Quat, &mut Quat);

        quaternion!(assign      Quat,      Quat);
        quaternion!(assign      Quat, &    Quat);
        quaternion!(assign      Quat, &mut Quat);
        quaternion!(assign &mut Quat,      Quat);
        quaternion!(assign &mut Quat, &    Quat);
        quaternion!(assign &mut Quat, &mut Quat);
    };
}

quaternion!();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        assert_eq!(Quat::new(1.0, Vec3::zero()), Quat::identity());
    }

    #[test]
    fn lerp() {}

    #[test]
    fn from_axis_angle() {}

    #[test]
    fn conjugate() {}

    #[test]
    fn inverse() {}
}
