use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Scale2 {
    pub w: f32,
    pub h: f32,
}

impl Scale2 {
    pub const fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }

    pub const fn identity() -> Self {
        Self { w: 1.0, h: 1.0 }
    }
}
