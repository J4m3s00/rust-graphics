use crate::{color::Color, font::Font, path_builder::Path, run_draw_command, vec::Vec2};

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
    Path {
        path: Path,
        path_size: Vec2,
        render_position: Vec2,
        render_size: Vec2,
        stroke_override: Option<Stroke>,
    },
}

impl DrawCommand {
    pub fn run(&self) {
        run_draw_command(self);
    }

    pub fn rect_fill(left: f32, top: f32, width: f32, height: f32, fill: Fill) -> Self {
        Self::Rect {
            left,
            top,
            width,
            height,
            fill: Some(fill),
            stroke: None,
        }
    }

    pub fn rect_outline(left: f32, top: f32, width: f32, height: f32, stroke: Stroke) -> Self {
        Self::Rect {
            left,
            top,
            width,
            height,
            fill: None,
            stroke: Some(stroke),
        }
    }

    pub fn rect(left: f32, top: f32, width: f32, height: f32, fill: Fill, stroke: Stroke) -> Self {
        Self::Rect {
            left,
            top,
            width,
            height,
            fill: Some(fill),
            stroke: Some(stroke),
        }
    }

    pub fn circle_fill(center: Vec2, radius: f32, fill: Fill) -> Self {
        Self::Circle {
            center,
            radius,
            fill: Some(fill),
            stroke: None,
        }
    }

    pub fn circle_outline(center: Vec2, radius: f32, stroke: Stroke) -> Self {
        Self::Circle {
            center,
            radius,
            fill: None,
            stroke: Some(stroke),
        }
    }

    pub fn circle(center: Vec2, radius: f32, fill: Fill, stroke: Stroke) -> Self {
        Self::Circle {
            center,
            radius,
            fill: Some(fill),
            stroke: Some(stroke),
        }
    }

    pub fn line(p1: Vec2, p2: Vec2, stroke: Stroke) -> Self {
        Self::Line {
            x1: p1.x,
            y1: p1.y,
            x2: p2.x,
            y2: p2.y,
            stroke,
        }
    }

    pub fn text(font: Font, text: String, position: Vec2, color: Color) -> Self {
        Self::Text {
            font,
            text,
            position,
            color,
            stroke: None,
        }
    }

    pub fn text_stroke(
        font: Font,
        text: String,
        position: Vec2,
        color: Color,
        stroke: Stroke,
    ) -> Self {
        Self::Text {
            font,
            text,
            position,
            color,
            stroke: Some(stroke),
        }
    }

    pub fn path(path: Path, path_size: Vec2, render_position: Vec2, render_size: Vec2) -> Self {
        Self::Path {
            path,
            path_size,
            render_position,
            render_size,
            stroke_override: None,
        }
    }

    pub fn path_stroke(
        path: Path,
        path_size: Vec2,
        render_position: Vec2,
        render_size: Vec2,
        stroke: Stroke,
    ) -> Self {
        Self::Path {
            path,
            path_size,
            render_position,
            render_size,
            stroke_override: Some(stroke),
        }
    }
}
