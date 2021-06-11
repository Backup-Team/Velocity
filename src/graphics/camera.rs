use crate::maths::{Angle, Mat4, Point3, Quat, Vec3};

pub struct Camera {
    pub position: Point3,
    pub rotation: Quat,

    projection: Mat4,
}

impl Camera {
    pub fn perspective(aspect_ratio: f32, field_of_view: Angle, near: f32, far: f32) -> Self {
        CameraBuilder::new(Mat4::perspective(aspect_ratio, field_of_view, near, far)).build()
    }

    pub fn orthographic_2d(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        CameraBuilder::new(Mat4::orthographic(left, right, top, bottom, -1.0, 1.0)).build()
    }

    pub fn orthographic_3d(
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        near: f32,
        far: f32,
    ) -> Self {
        CameraBuilder::new(Mat4::orthographic(left, right, top, bottom, near, far)).build()
    }

    pub fn view_projection(&self) -> Mat4 {
        let forward = Vec3::unit_z().rotate(&self.rotation);
        let up = Vec3::unit_y().rotate(&self.rotation);
        let right = Vec3::unit_x().rotate(&self.rotation);

        Mat4::view(forward, up, right, self.position)
    }
}

pub struct CameraBuilder {
    position:   Option<Point3>,
    rotation:   Option<Quat>,
    projection: Mat4,
}

impl CameraBuilder {
    pub fn new(projection: Mat4) -> Self {
        Self {
            position: None,
            rotation: None,
            projection,
        }
    }

    pub fn with_position(&mut self, position: Point3) -> &mut Self {
        self.position.replace(position);
        self
    }

    pub fn with_rotation(&mut self, rotation: Quat) -> &mut Self {
        self.rotation.replace(rotation);
        self
    }

    pub fn build(self) -> Camera {
        Camera {
            position:   self.position.unwrap_or_else(Point3::identity),
            rotation:   self.rotation.unwrap_or_else(Quat::identity),
            projection: self.projection,
        }
    }
}
