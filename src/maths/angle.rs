use std::{
    f32::consts::{FRAC_PI_2, PI, TAU},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use bytemuck::{Pod, Zeroable};

const DEG_TO_RAD: f32 = TAU / 360.0;
const RAD_TO_DEG: f32 = 360.0 / TAU;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Angle(f32);

impl Angle {
    pub const fn zero() -> Self {
        Self(0.0)
    }

    pub const fn half_pi() -> Self {
        Self(FRAC_PI_2)
    }

    pub const fn pi() -> Self {
        Self(PI)
    }

    // TODO:
    // Make this a const when floating point arithmatic is allowed
    pub fn three_pi() -> Self {
        Self(PI + FRAC_PI_2)
    }

    // TODO:
    // Make this a const when floating point arithmatic is allowed
    pub fn degrees(degrees: f32) -> Self {
        Self((degrees * DEG_TO_RAD) % TAU)
    }

    // TODO:
    // Make this a const when floating point arithmatic is allowed
    pub fn radians(radians: f32) -> Self {
        Self(radians % TAU)
    }

    // TODO:
    // Make this a const when floating point arithmatic is allowed
    pub fn into_degress(self) -> f32 {
        self.0 * RAD_TO_DEG
    }

    pub const fn into_radians(self) -> f32 {
        self.0
    }

    pub fn sin_cos(&self) -> (f32, f32) {
        (f32::sin(self.0), f32::cos(self.0))
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(mut self, rhs: Angle) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + rhs.0) % TAU;
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(mut self, rhs: Angle) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = (self.0 - rhs.0) % TAU;
    }
}

impl Mul<f32> for Angle {
    type Output = Angle;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f32> for Angle {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 = (self.0 * rhs) % TAU;
    }
}

impl Div<f32> for Angle {
    type Output = Angle;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<f32> for Angle {
    fn div_assign(&mut self, rhs: f32) {
        self.0 = (self.0 / rhs) % TAU;
    }
}
