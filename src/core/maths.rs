mod angle;
mod mat4;
mod point2;
mod point3;
mod quat;
mod scale2;
mod scale3;
mod unit;
mod vec2;
mod vec3;

// TODO:
//
// mod line2;
// mod line3;
// mod segment2;
// mod segment3;
// mod radial_vec2;
// mod radial_vec3;
// mod spherical_vec3;
//
// Quat from_yaw_pitch_roll
// impl PartialEq using fuzzy

pub mod fuzzy;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub use crate::core::maths::{
    angle::*,
    mat4::*,
    point2::*,
    point3::*,
    quat::*,
    scale2::*,
    scale3::*,
    unit::*,
    vec2::*,
    vec3::*,
};

// macro_rules! permute {
//     ($a:ty, $b:ty) => {

//     };
// }

macro_rules! scalar {
    (standard $ref:ty => $struct:ty { $( $member:ident )* } ($trait:ident $fn:ident $op:tt)) => {
        impl $trait<f32> for $ref {
            type Output = $struct;

            fn $fn(self, rhs: f32) -> Self::Output {
                Self::Output {
                    $(
                        $member: self.$member $op rhs,
                    )*
                }
            }
        }

        impl $trait<$ref> for f32 {
            type Output = $struct;

            fn $fn(self, rhs: $ref) -> Self::Output {
                Self::Output {
                    $(
                        $member: rhs.$member $op self,
                    )*
                }
            }
        }
    };

    (assign $ref:ty => $struct:ty { $( $member:ident )* } ($trait:ident $fn:ident $op:tt)) => {
        impl $trait<f32> for $ref {
            fn $fn(&mut self, rhs: f32) {
                $(
                    self.$member $op rhs;
                )*
            }
        }
    };



    ($($struct:ty, $unit:ty => { $( $member:ident )* },)*) => {
        $(
            scalar!(standard      $struct => $struct { $( $member )* } (Mul mul *));
            scalar!(standard &    $struct => $struct { $( $member )* } (Mul mul *));
            scalar!(standard &mut $struct => $struct { $( $member )* } (Mul mul *));
            scalar!(standard      $struct => $struct { $( $member )* } (Div div /));
            scalar!(standard &    $struct => $struct { $( $member )* } (Div div /));
            scalar!(standard &mut $struct => $struct { $( $member )* } (Div div /));

            scalar!(standard      $unit => $struct { $( $member )* } (Mul mul *));
            scalar!(standard &    $unit => $struct { $( $member )* } (Mul mul *));
            scalar!(standard &mut $unit => $struct { $( $member )* } (Mul mul *));
            scalar!(standard      $unit => $struct { $( $member )* } (Div div /));
            scalar!(standard &    $unit => $struct { $( $member )* } (Div div /));
            scalar!(standard &mut $unit => $struct { $( $member )* } (Div div /));

            scalar!(assign      $struct => $struct { $( $member )* } (MulAssign mul_assign *=));
            scalar!(assign &mut $struct => $struct { $( $member )* } (MulAssign mul_assign *=));
            scalar!(assign      $struct => $struct { $( $member )* } (DivAssign div_assign /=));
            scalar!(assign &mut $struct => $struct { $( $member )* } (DivAssign div_assign /=));
        )*
    };


}

scalar!(
    Vec2, Unit<Vec2> => { x y },
    Vec3, Unit<Vec3> => { x y z },
    Quat, Unit<Quat> => { v w },
);

macro_rules! vector {
    (standard $lhs:ty, $rhs:ty => $struct:ty { $( $member:ident )* } ($trait:ident $fn:ident $op:tt)) => {
        impl $trait<$rhs> for $lhs {
            type Output = $struct;

            fn $fn(self, rhs: $rhs) -> Self::Output {
                Self::Output {
                    $(
                        $member: self.$member $op rhs.$member,
                    )*
                }
            }
        }
    };

    (assign $lhs:ty, $rhs:ty => $struct:ty { $( $member:ident )* } ($trait:ident $fn:ident $op:tt)) => {
        impl $trait<$rhs> for $lhs {
            fn $fn(&mut self, rhs: $rhs) {
                $(
                    self.$member $op rhs.$member;
                )*
            }
        }
    };

    ($lhs:ty, $rhs:ty => $struct:ty { $( $member:ident )* } ($trait:ident $fn:ident $op:tt) ($trait_assign:ident $fn_assign:ident $op_assign:tt)) => {
        // TODO:
        // Write a macro to help writing this macro...

        vector!(standard      $lhs,      $rhs => $struct { $( $member )* } ($trait $fn $op));
        vector!(standard      $lhs, &    $rhs => $struct { $( $member )* } ($trait $fn $op));
        vector!(standard      $lhs, &mut $rhs => $struct { $( $member )* } ($trait $fn $op));
        vector!(standard &    $lhs,      $rhs => $struct { $( $member )* } ($trait $fn $op));
        vector!(standard &    $lhs, &    $rhs => $struct { $( $member )* } ($trait $fn $op));
        vector!(standard &    $lhs, &mut $rhs => $struct { $( $member )* } ($trait $fn $op));
        vector!(standard &mut $lhs,      $rhs => $struct { $( $member )* } ($trait $fn $op));
        vector!(standard &mut $lhs, &    $rhs => $struct { $( $member )* } ($trait $fn $op));
        vector!(standard &mut $lhs, &mut $rhs => $struct { $( $member )* } ($trait $fn $op));

        vector!(assign      $lhs,      $rhs => $struct { $( $member )* } ($trait_assign $fn_assign $op_assign));
        vector!(assign      $lhs, &    $rhs => $struct { $( $member )* } ($trait_assign $fn_assign $op_assign));
        vector!(assign      $lhs, &mut $rhs => $struct { $( $member )* } ($trait_assign $fn_assign $op_assign));
        vector!(assign &mut $lhs,      $rhs => $struct { $( $member )* } ($trait_assign $fn_assign $op_assign));
        vector!(assign &mut $lhs, &    $rhs => $struct { $( $member )* } ($trait_assign $fn_assign $op_assign));
        vector!(assign &mut $lhs, &mut $rhs => $struct { $( $member )* } ($trait_assign $fn_assign $op_assign));
    };

    ($($struct:ty, $unit:ty => { $( $member:ident )* },)*) => {
        $(
            vector!($struct, $struct => $struct { $( $member )* } (Add add +) (AddAssign add_assign +=));
            vector!($unit  , $unit   => $struct { $( $member )* } (Add add +) (AddAssign add_assign +=));
            vector!($struct, $unit   => $struct { $( $member )* } (Add add +) (AddAssign add_assign +=));
            vector!($unit  , $struct => $struct { $( $member )* } (Add add +) (AddAssign add_assign +=));

            vector!($struct, $struct => $struct { $( $member )* } (Sub sub -) (SubAssign sub_assign -=));
            vector!($unit  , $unit   => $struct { $( $member )* } (Sub sub -) (SubAssign sub_assign -=));
            vector!($struct, $unit   => $struct { $( $member )* } (Sub sub -) (SubAssign sub_assign -=));
            vector!($unit  , $struct => $struct { $( $member )* } (Sub sub -) (SubAssign sub_assign -=));
        )*
    };
}

vector!(
    Vec2, Unit<Vec2> => { x y },
    Vec3, Unit<Vec3> => { x y z },
    Quat, Unit<Quat> => { v w },
);

macro_rules! negate {
    ($ref:ty => $struct:ty { $( $member:ident )* }) => {
        impl Neg for $ref {
            type Output = $struct;

            fn neg(self) -> Self::Output {
                Self::Output {
                    $(
                        $member: -self.$member,
                    )*
                }
            }
        }
    };

    ($($struct:ty { $( $member:ident )* },)*) => {
        $(
            negate!(     $struct => $struct { $( $member )* } );
            negate!(&    $struct => $struct { $( $member )* } );
            negate!(&mut $struct => $struct { $( $member )* } );
        )*
    }
}

negate!(
    Vec2 { x y },
    Vec3 { x y z },
);

macro_rules! equal {
    ($lhs:ty, $rhs:ty => { $( $member:ident => $eq:expr $( ; $ref:tt )?,)* }) => {
        impl PartialEq<$rhs> for $lhs {
            fn eq(&self, rhs: &$rhs) -> bool {
                let mut equal = false;

                $(
                    equal = equal && $eq($($ref)?self.$member, $($ref)?rhs.$member);
                )*

                equal
            }
        }
    };

    ($($struct:ty => { $( $member:ident => $eq:expr $( ; $ref:tt )?,)* },)*) => {
        $(
            equal!($struct, $struct => { $( $member => $eq $(; $ref)?, )* });
        )*
    };

    ($($struct:ty, $unit:ty => { $( $member:ident => $eq:expr $( ; $ref:tt )?,)* },)*) => {
        $(
            equal!($struct, $struct => { $( $member => $eq $(; $ref)?, )* });
            equal!($struct, $unit   => { $( $member => $eq $(; $ref)?, )* });
            equal!($unit  , $struct => { $( $member => $eq $(; $ref)?, )* });
            equal!($unit  , $unit   => { $( $member => $eq $(; $ref)?, )* });
        )*
    };
}

equal!(
    Vec2, Unit<Vec2> => {
        x => fuzzy::eq,
        y => fuzzy::eq,
    },

    Vec3, Unit<Vec3> => {
        x => fuzzy::eq,
        y => fuzzy::eq,
        z => fuzzy::eq,
    },

    Quat, Unit<Quat> => {
        v => Vec3 ::eq; &,
        w => fuzzy::eq,
    },
);

equal!(
    Point2 => {
        x => fuzzy::eq,
        y => fuzzy::eq,
    },

    Point3 => {
        x => fuzzy::eq,
        y => fuzzy::eq,
        z => fuzzy::eq,
    },
);
