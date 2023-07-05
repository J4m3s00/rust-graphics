use crate::{
    bindings::{
        c_path_begin, c_path_cubic_bezier_curve_to, c_path_ellips_arc_to, c_path_end,
        c_path_line_to, c_path_quadr_bezier_curve_to,
    },
    color::COLOR_WHITE,
    draw_command::{Fill, Stroke},
    vec::Vec2,
};

#[derive(Clone, PartialEq)]
enum PathElem {
    MoveTo(Vec2),
    LineTo(Vec2),
    QuadTo(Vec2, Vec2),
    CubicTo(Vec2, Vec2, Vec2),
    ArcTo {
        to: Vec2,
        radius: Vec2,
        angle: f32,
        large_arc_flag: bool,
        sweep_flag: bool,
    },
    Close,
}

#[derive(Clone, Default)]
pub struct Path {
    elems: Vec<PathElem>,
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
    pub closed: bool,
}

impl Path {
    unsafe fn begin(&self) {
        c_path_begin(
            self.stroke.is_some() as i32,
            self.fill.is_some() as i32,
            self.stroke.map(|s| s.width).unwrap_or(0.),
            self.stroke
                .map(|s| s.color.as_int())
                .unwrap_or(COLOR_WHITE.as_int()),
            self.fill
                .map(|f| f.color.as_int())
                .unwrap_or(COLOR_WHITE.as_int()),
        );
    }

    pub unsafe fn execute(&self) {
        let mut justed_closed = false;
        for elem in &self.elems {
            let closed = justed_closed;
            justed_closed = false;

            if closed {
                self.begin();
            }

            match elem {
                PathElem::MoveTo(to) => {
                    if !closed {
                        c_path_end(false as i32);
                        self.begin();
                    }
                    c_path_line_to(to.x, to.y);
                } //c_path_move_to(to.x, to.y
                PathElem::LineTo(to) => {
                    c_path_line_to(to.x, to.y);
                }
                PathElem::QuadTo(ctrl, to) => {
                    c_path_quadr_bezier_curve_to(ctrl.x, ctrl.y, to.x, to.y, 22);
                }
                PathElem::CubicTo(ctrl1, ctrl2, to) => {
                    c_path_cubic_bezier_curve_to(
                        ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y, 22,
                    );
                }
                PathElem::ArcTo {
                    to,
                    radius,
                    angle,
                    large_arc_flag,
                    sweep_flag,
                } => c_path_ellips_arc_to(
                    to.x,
                    to.y,
                    *angle,
                    radius.x,
                    radius.y,
                    *large_arc_flag as i32,
                    *sweep_flag as i32,
                    22,
                ),
                PathElem::Close => {
                    justed_closed = true;
                    c_path_end(true as i32);
                }
            }
        }
        unsafe {
            c_path_end(0);
        }
    }
}

pub struct PathBuilder {
    pub path: Path,
}

impl PathBuilder {
    pub fn new() -> Self {
        Self {
            path: Path::default(),
        }
    }

    pub fn move_to(&mut self, to: impl Into<Vec2>) {
        let to = to.into();
        self.path.elems.push(PathElem::MoveTo(to.into()));
    }

    pub fn line_to(&mut self, to: impl Into<Vec2>) {
        self.path.elems.push(PathElem::LineTo(to.into()));
    }

    pub fn quad_to(&mut self, ctrl: impl Into<Vec2>, to: impl Into<Vec2>) {
        self.path
            .elems
            .push(PathElem::QuadTo(ctrl.into(), to.into()));
    }

    pub fn cubic_to(
        &mut self,
        ctrl1: impl Into<Vec2>,
        ctrl2: impl Into<Vec2>,
        to: impl Into<Vec2>,
    ) {
        self.path
            .elems
            .push(PathElem::CubicTo(ctrl1.into(), ctrl2.into(), to.into()));
    }

    pub fn arc_to(
        &mut self,
        to: impl Into<Vec2>,
        radius: impl Into<Vec2>,
        angle: f32,
        large_arc_flag: bool,
        sweep_flag: bool,
    ) {
        self.path.elems.push(PathElem::ArcTo {
            to: to.into(),
            radius: radius.into(),
            angle,
            large_arc_flag,
            sweep_flag,
        });
    }

    pub fn close(&mut self) {
        self.path.elems.push(PathElem::Close);
        self.path.closed = true;
    }

    pub fn fill(&mut self, fill: Option<Fill>) {
        self.path.fill = fill;
    }

    pub fn stroke(&mut self, stroke: Option<Stroke>) {
        self.path.stroke = stroke;
    }

    pub fn build(self) -> Path {
        if self.path.elems.is_empty() {
            panic!("PathBuilder: path is empty");
        }
        if self.path.fill.is_none() && self.path.stroke.is_none() {
            panic!("PathBuilder: path has no fill and no stroke");
        }
        self.path
    }
}
