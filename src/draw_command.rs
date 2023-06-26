use crate::{color::Color, font::Font, vec::Vec2};

#[derive(Clone, Copy)]
pub struct Fill {
    pub color: Color,
}

#[derive(Clone, Copy)]
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
    Circle {
        center: Vec2,
        radius: f32,
        fill: Option<Fill>,
        stroke: Option<Stroke>,
    },
    Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        stroke: Stroke,
    },
    Text {
        font: Font,
        text: String,
        position: Vec2,
        color: Color,
    },
}
