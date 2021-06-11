pub struct Scale3 {
    pub w: f32,
    pub h: f32,
    pub d: f32,
}

impl Scale3 {
    pub const fn new(w: f32, h: f32, d: f32) -> Self {
        Self { w, h, d }
    }

    pub const fn identity() -> Self {
        Self {
            w: 1.0,
            h: 1.0,
            d: 1.0,
        }
    }
}
