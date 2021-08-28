use crate::core::maths::{Unit, Vec2};

pub struct Ray2 {
    perpendicular: Vec2,
    reciprocal:    Vec2,
    origin:        Vec2,
    direction:     Unit<Vec2>,
}

impl Ray2 {
    pub fn new(origin: Vec2, mut direction: Unit<Vec2>) -> Self {
        direction.normalise_fast();

        let perpendicular = direction.left_perpendicular();

        // Each ±0 in the direction will become a ±Infinity.
        let reciprocal = direction.reciprocal();

        Self {
            perpendicular,
            reciprocal,
            origin,
            direction,
        }
    }

    // Source:
    // https://tavianator.com/2011/ray_box.html
    // https://tavianator.com/2015/ray_box_nan.html
    // pub fn intersectsAABB()
}

//     private prependicular: Vector2D;
//     private reciprocal: Vector2D;

//     constructor(
//         public origin: Vector2D,
//         public direction: Unit<Vector2D>,
//     ) {
//         const unit = this.direction.inner();

//         // Each ±0 in the direction will become a ±Infinity.
//         this.reciprocal = unit.reciprocal();
//         this.prependicular = unit.perp();
//     }

//     // Source:
//     // https://tavianator.com/2011/ray_box.html
//     // https://tavianator.com/2015/ray_box_nan.html
//     public intersectsAABB(box: AxisAligned2dBoundingBox): boolean {
//         const {
//             reciprocal: {
//                 x: rx,
//                 z: rz,
//             },
//             origin: {
//                 x: ox,
//                 z: oz,
//             },
//         } = this;

//         const min = box.getMin();
//         const max = box.getMax();

//         const tx1 = (min.x - ox) * rx;
//         const tx2 = (max.x - ox) * rx;
//         const tz1 = (min.z - oz) * rz;
//         const tz2 = (max.z - oz) * rz;

//         const largestMin = Math.max(
//             Math.min(tx1, tx2),
//             Math.min(tz1, tz2),
//         );

//         const smallestMax = Math.min(
//             Math.max(tx1, tx2),
//             Math.max(tz1, tz2),
//         );

//         return smallestMax >= Math.max(largestMin, 0.0);
//     }
// }
