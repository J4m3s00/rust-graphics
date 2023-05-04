use crate::vec::Vec2;

pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: impl Into<Vec2>, radius: f32) -> Self {
        Self {
            center: center.into(),
            radius,
        }
    }
}
