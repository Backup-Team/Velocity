use std::ops::{Deref, DerefMut, DivAssign, MulAssign};

use bytemuck::{Pod, Zeroable};

pub trait Normed: MulAssign<f32> + DivAssign<f32> {
    fn norm_squared(&self) -> f32;

    fn norm(&self) -> f32 {
        f32::sqrt(self.norm_squared())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Unit<T>(T)
where
    T: Normed;

impl<T> Unit<T>
where
    T: Normed,
{
    /// Assumes that the value is already normalised.
    pub fn from_normalised(value: T) -> Self {
        Self(value)
    }

    pub fn from(value: T) -> Self {
        let mut unit = Self(value);

        unit.normalise();
        unit
    }

    /// Normalizes this vector again. This is useful when repeated computations
    /// might cause a drift in the norm because of float inaccuracies.
    ///
    /// Returns the norm before re-normalization. See `.renormalize_fast` for a faster alternative
    /// that may be slightly less accurate if `self` drifted significantly from having a unit
    /// length.
    pub fn normalise(&mut self) {
        self.0 /= self.0.norm();
    }

    /// Normalizes this vector again using a first-order Taylor approximation.
    /// This is useful when repeated computations might cause a drift in the norm
    /// because of float inaccuracies.
    pub fn nomalise_fast(&mut self) {
        self.0 *= 0.5 * (3.0 - self.0.norm_squared());
    }
}

impl<T> Deref for Unit<T>
where
    T: Normed,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Unit<T>
where
    T: Normed,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl<T: Normed + Pod> Pod for Unit<T> {}

unsafe impl<T: Normed + Zeroable> Zeroable for Unit<T> {}
