mod angle;
mod bounds3;
mod point3;
mod quat;
mod unit;
mod vec2;
mod vec3;

pub mod fuzzy;

// TODO:
// Point2
// Mat4x4
// Bounds2
// Quat from_yaw_pitch_roll
// impl PartialEq using fuzzy

pub use crate::maths::{angle::*, bounds3::*, point3::*, quat::*, unit::*, vec2::*, vec3::*};

// TODO:
// Figure out how to mark macros as pub(crate)
// Might need to make a workspace lib and re-export them
#[macro_export]
macro_rules! vec_maths {
    ($($struct:ident { $( $member:ident )* } ( $trait:ident $fn:ident ) ( $assign_trait:ident $assign_fn:ident ) $op:tt,)*) => {
        $(
            use std::ops::{$trait, $assign_trait};

            impl $trait for $struct {
                type Output = $struct;

                fn $fn(mut self, rhs: $struct) -> Self::Output {
                    self $op rhs;
                    self
                }
            }

            impl $assign_trait for $struct {
                fn $assign_fn(&mut self, rhs: $struct) {
                    $(self.$member $op rhs.$member;)*
                }
            }

            impl $trait<&Self> for $struct {
                type Output = $struct;

                fn $fn(mut self, rhs: &Self) -> Self::Output {
                    self $op rhs;
                    self
                }
            }

            impl $assign_trait<&Self> for $struct {
                fn $assign_fn(&mut self, rhs: &Self) {
                    $(self.$member $op rhs.$member;)*
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! scalar_maths {
    ($($struct:ident { $( $member:ident )* } ( $trait:ident $fn:ident ) ( $assign_trait:ident $assign_fn:ident ) $op:tt,)*) => {
        $(
            use std::ops::{$trait, $assign_trait};

            impl $trait<f32> for $struct {
                type Output = $struct;

                fn $fn(mut self, rhs: f32) -> Self::Output {
                    self $op rhs;
                    self
                }
            }

            impl $assign_trait<f32> for $struct {
                fn $assign_fn(&mut self, rhs: f32) {
                    $(self.$member $op rhs;)*
                }
            }
        )*
    };
}
