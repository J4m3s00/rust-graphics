use crate::{
    bindings::{
        c_path_begin, c_path_cubic_bezier_curve_to, c_path_ellips_arc_to, c_path_end,
        c_path_line_to, c_path_quadr_bezier_curve_to,
    },
    color::{Color, COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_RED, COLOR_WHITE},
    draw_command::{DrawCommand, Fill, Stroke},
    vec::Vec2,
};

#[derive(Clone, PartialEq, Debug)]
enum PathElem {
    MoveTo(Vec2),
    LineTo(Vec2),
    Vert(f32),
    Horiz(f32),
    /// ctrl1, to
    QuadTo(Vec2, Vec2),
    /// to
    SmoothQuad(Vec2),
    /// ctrl1, ctrl2, to
    CubicTo(Vec2, Vec2, Vec2),
    /// ctrl2, to
    SmoothCubic(Vec2, Vec2),
    ArcTo {
        to: Vec2,
        radius: Vec2,
        angle: f32,
        large_arc_flag: bool,
        sweep_flag: bool,
    },
    Close,
}

#[derive(Clone, Default, Debug)]
pub struct Path {
    /// bool is relative
    elems: Vec<(PathElem, bool)>,
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

    pub unsafe fn draw(&self, offset: Vec2, scale: Vec2) {
        let mut justed_closed = false;
        let mut last_pos = Vec2::default();
        let mut last_move_pos = Vec2::default();
        let mut last_controll_point = None;
        for elem in &self.elems {
            let closed = justed_closed;
            justed_closed = false;

            if closed {
                self.begin();
            }

            let get_pos = |pos: Vec2| -> Vec2 {
                let pos = pos * scale;
                if elem.1 {
                    pos + last_pos
                } else {
                    offset + pos
                }
            };

            let this_last_controll = last_controll_point.clone();
            last_controll_point = None;

            match elem {
                (PathElem::MoveTo(to), _) => {
                    if !closed {
                        c_path_end(false as i32);
                        self.begin();
                    }
                    let to = get_pos(to.clone());
                    c_path_line_to(to.x, to.y);
                    last_pos = to;
                    last_move_pos = to;
                } //c_path_move_to(to.x, to.y
                (PathElem::LineTo(to), _) => {
                    let to = get_pos(to.clone());
                    c_path_line_to(to.x, to.y);
                    last_pos = to;
                }
                (PathElem::Vert(y), _) => {
                    let to_y = get_pos(Vec2::new(0.0, *y)).y;
                    let to = Vec2::new(last_pos.x, to_y);
                    c_path_line_to(to.x, to.y);
                    last_pos = to;
                }
                (PathElem::Horiz(x), _) => {
                    let to_x = get_pos(Vec2::new(*x, 0.0)).x;
                    let to = Vec2::new(to_x, last_pos.y);
                    c_path_line_to(to.x, to.y);
                    last_pos = to;
                }
                (PathElem::QuadTo(ctrl, to), _) => {
                    let to = get_pos(to.clone());
                    let ctrl = get_pos(ctrl.clone());

                    c_path_quadr_bezier_curve_to(ctrl.x, ctrl.y, to.x, to.y, 22);
                    DrawCommand::circle_fill(ctrl, 3.0, Fill::new(COLOR_BLUE)).run();
                    //DrawCommand::line(last_pos, ctrl, Stroke::new(COLOR_BLACK, 2.0)).run();
                    //DrawCommand::line(ctrl, to, Stroke::new(COLOR_BLACK, 2.0)).run();

                    last_pos = to;
                    last_controll_point = Some(ctrl);
                }
                (PathElem::SmoothQuad(to), _) => {
                    let to = get_pos(to.clone());
                    let ctrl = this_last_controll.map_or(last_pos, |c| last_pos + (last_pos - c));

                    c_path_quadr_bezier_curve_to(ctrl.x, ctrl.y, to.x, to.y, 22);
                    DrawCommand::circle_fill(ctrl, 3.0, Fill::new(COLOR_BLUE)).run();
                    DrawCommand::circle_fill(to, 3.0, Fill::new(COLOR_GREEN)).run();

                    last_pos = to;
                    last_controll_point = Some(ctrl);
                }
                (PathElem::SmoothCubic(ctrl2, to), _) => {
                    let to = get_pos(to.clone());
                    let ctrl1 = this_last_controll.map_or(last_pos, |c| last_pos + (last_pos - c));
                    let ctrl2 = get_pos(ctrl2.clone());

                    c_path_cubic_bezier_curve_to(
                        ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y, 22,
                    );
                    last_controll_point = Some(ctrl2);
                    last_pos = to;
                }
                (PathElem::CubicTo(ctrl1, ctrl2, to), _) => {
                    let to = get_pos(to.clone());
                    let ctrl1 = get_pos(ctrl1.clone());
                    let ctrl2 = get_pos(ctrl2.clone());
                    c_path_cubic_bezier_curve_to(
                        ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y, 22,
                    );
                    last_controll_point = Some(ctrl2);
                    last_pos = to;
                }
                (
                    PathElem::ArcTo {
                        to,
                        radius,
                        angle,
                        large_arc_flag,
                        sweep_flag,
                    },
                    _,
                ) => {
                    let to = get_pos(to.clone());
                    let radius = get_pos(radius.clone());

                    c_path_ellips_arc_to(
                        to.x,
                        to.y,
                        *angle,
                        radius.x,
                        radius.y,
                        *large_arc_flag as i32,
                        *sweep_flag as i32,
                        22,
                    );
                    last_pos = to;
                }
                (PathElem::Close, _) => {
                    justed_closed = true;
                    c_path_end(true as i32);
                    last_pos = last_move_pos;
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
        self.path.elems.push((PathElem::MoveTo(to.into()), false));
    }

    pub fn move_to_rel(&mut self, to: impl Into<Vec2>) {
        let to = to.into();
        self.path.elems.push((PathElem::MoveTo(to.into()), true));
    }

    pub fn line_to(&mut self, to: impl Into<Vec2>) {
        self.path.elems.push((PathElem::LineTo(to.into()), false));
    }

    pub fn line_to_rel(&mut self, to: impl Into<Vec2>) {
        self.path.elems.push((PathElem::LineTo(to.into()), true));
    }

    pub fn vert(&mut self, y: f32) {
        self.path.elems.push((PathElem::Vert(y), false));
    }

    pub fn vert_rel(&mut self, y: f32) {
        self.path.elems.push((PathElem::Vert(y), true));
    }

    pub fn horiz(&mut self, x: f32) {
        self.path.elems.push((PathElem::Horiz(x), false));
    }

    pub fn horiz_rel(&mut self, x: f32) {
        self.path.elems.push((PathElem::Horiz(x), true));
    }

    pub fn quad_to(&mut self, ctrl: impl Into<Vec2>, to: impl Into<Vec2>) {
        self.path
            .elems
            .push((PathElem::QuadTo(ctrl.into(), to.into()), false));
    }

    pub fn quad_to_rel(&mut self, ctrl: impl Into<Vec2>, to: impl Into<Vec2>) {
        self.path
            .elems
            .push((PathElem::QuadTo(ctrl.into(), to.into()), true));
    }

    pub fn smooth_quad_to(&mut self, to: impl Into<Vec2>) {
        self.path
            .elems
            .push((PathElem::SmoothQuad(to.into()), false));
    }

    pub fn smooth_quad_to_rel(&mut self, to: impl Into<Vec2>) {
        self.path
            .elems
            .push((PathElem::SmoothQuad(to.into()), true));
    }

    pub fn cubic_to(
        &mut self,
        ctrl1: impl Into<Vec2>,
        ctrl2: impl Into<Vec2>,
        to: impl Into<Vec2>,
    ) {
        self.path.elems.push((
            PathElem::CubicTo(ctrl1.into(), ctrl2.into(), to.into()),
            false,
        ));
    }

    pub fn cubic_to_rel(
        &mut self,
        ctrl1: impl Into<Vec2>,
        ctrl2: impl Into<Vec2>,
        to: impl Into<Vec2>,
    ) {
        self.path.elems.push((
            PathElem::CubicTo(ctrl1.into(), ctrl2.into(), to.into()),
            true,
        ));
    }

    pub fn smooth_cubic_to(&mut self, ctrl2: impl Into<Vec2>, to: impl Into<Vec2>) {
        self.path
            .elems
            .push((PathElem::SmoothCubic(ctrl2.into(), to.into()), false));
    }

    pub fn smooth_cubic_to_rel(&mut self, ctrl2: impl Into<Vec2>, to: impl Into<Vec2>) {
        self.path
            .elems
            .push((PathElem::SmoothCubic(ctrl2.into(), to.into()), true));
    }

    pub fn arc_to(
        &mut self,
        to: impl Into<Vec2>,
        radius: impl Into<Vec2>,
        angle: f32,
        large_arc_flag: bool,
        sweep_flag: bool,
    ) {
        self.path.elems.push((
            PathElem::ArcTo {
                to: to.into(),
                radius: radius.into(),
                angle,
                large_arc_flag,
                sweep_flag,
            },
            false,
        ));
    }

    pub fn arc_to_rel(
        &mut self,
        to: impl Into<Vec2>,
        radius: impl Into<Vec2>,
        angle: f32,
        large_arc_flag: bool,
        sweep_flag: bool,
    ) {
        self.path.elems.push((
            PathElem::ArcTo {
                to: to.into(),
                radius: radius.into(),
                angle,
                large_arc_flag,
                sweep_flag,
            },
            true,
        ));
    }

    pub fn close(&mut self) {
        self.path.elems.push((PathElem::Close, false));
        self.path.closed = true;
    }

    pub fn fill(&mut self, fill: Option<Fill>) {
        self.path.fill = fill;
    }

    pub fn stroke(&mut self, stroke: Option<Stroke>) {
        self.path.stroke = stroke;
    }

    pub fn build(&self) -> Path {
        if self.path.elems.is_empty() {
            panic!("PathBuilder: path is empty");
        }
        if self.path.fill.is_none() && self.path.stroke.is_none() {
            panic!("PathBuilder: path has no fill and no stroke");
        }
        self.path.clone()
    }
}
