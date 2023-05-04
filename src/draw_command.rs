use crate::{circle::Circle, color::Color, rect::Rect, text::Text};

pub struct Fill {
    pub color: Color,
}

pub struct Stroke {
    pub width: f32,
    pub color: Color,
}

pub enum DrawCommand {
    Rect {
        left: f32,
        top: f32,
        width: f32,
        height: f32,
        fill: Option<Fill>,
        stroke: Option<Stroke>,
    },
    Circle(Circle),
    Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    },
    Text(Text),
}
