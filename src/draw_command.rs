use crate::{
    color::{Color},
    font::Font,
    path_builder::Path,
    vec::Vec2,
};

#[derive(Clone, Copy, Debug)]
pub struct Fill {
    pub color: Color,
}

impl Fill {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Stroke {
    pub width: f32,
    pub color: Color,
}

impl Stroke {
    pub fn new(color: Color, width: f32) -> Self {
        Self { color, width }
    }
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
        stroke: Option<Stroke>,
    },
    Path(Path),
}
