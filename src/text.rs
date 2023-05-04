use crate::vec::Vec2;

pub struct TextStyle {
    pub font_size: f32,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self { font_size: 24.0 }
    }
}

pub struct Text {
    pub position: Vec2,
    pub text: String,
    pub style: TextStyle,
}

impl Text {
    pub fn new(position: impl Into<Vec2>, text: impl Into<String>) -> Self {
        Self {
            position: position.into(),
            text: text.into(),
            style: TextStyle::default(),
        }
    }
}
