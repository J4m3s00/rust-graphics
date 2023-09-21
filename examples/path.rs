use rust_graphics::{
    app::App,
    color::{COLOR_BLACK, COLOR_RED},
    draw_command::{DrawCommand, Stroke},
    init_app,
    path_builder::{Path, PathBuilder},
    run_draw_command,
    vec::Vec2,
};

struct Editor {
    path: Path,
}

fn heart_builder() -> PathBuilder {
    let mut builder = PathBuilder::new();
    builder.stroke(Some(Stroke::new(COLOR_BLACK, 2.0)));
    //builder.fill(Some(Fill::new(COLOR_MAGENTA)));
    builder.move_to(Vec2 { x: 24.0, y: 41.95 });
    builder.line_to_rel(Vec2 { x: -2.05, y: -1.85 });
    builder.quad_to_rel(
        Vec2 { x: -5.3, y: -4.85 },
        Vec2 {
            x: -8.75,
            y: -8.375,
        },
    );
    builder.quad_to_rel(
        Vec2 {
            x: -3.45,
            y: -3.525,
        },
        Vec2 { x: -5.5, y: -6.3 },
    );
    builder.quad_to(Vec2 { x: 4.0, y: 18.15 }, Vec2 { x: 4.0, y: 15.85 });
    builder.quad_to_rel(
        Vec2 { x: 0.0, y: -4.5 },
        Vec2 {
            x: 3.025,
            y: -7.525,
        },
    );
    builder.quad_to(Vec2 { x: 10.05, y: 5.3 }, Vec2 { x: 14.5, y: 5.3 });
    builder.quad_to_rel(Vec2 { x: 2.85, y: 0.0 }, Vec2 { x: 5.275, y: 1.35 });
    builder.quad_to(Vec2 { x: 22.2, y: 8.0 }, Vec2 { x: 24.0, y: 10.55 });
    builder.quad_to_rel(Vec2 { x: 2.1, y: -2.7 }, Vec2 { x: 4.45, y: -3.975 });
    builder.smooth_quad_to(Vec2 { x: 33.5, y: 5.3 });
    //builder.quad_to((30.8, 5.3), (33.5, 5.3));
    builder.quad_to_rel(Vec2 { x: 4.45, y: 0.0 }, Vec2 { x: 7.475, y: 3.025 });
    builder.quad_to(Vec2 { x: 44.0, y: 11.35 }, Vec2 { x: 44.0, y: 15.85 });
    builder.quad_to_rel(Vec2 { x: 0.0, y: 2.3 }, Vec2 { x: -0.825, y: 4.55 });
    builder.quad_to_rel(Vec2 { x: -2.05, y: 2.775 }, Vec2 { x: -5.5, y: 6.3 });
    builder.close();
    builder
}

fn star_builder() -> PathBuilder {
    let mut builder = PathBuilder::new();
    builder.stroke(Some(Stroke::new(COLOR_BLACK, 2.0)));
    builder.move_to_rel((11.65, 44.0));
    builder.line_to_rel((4.65, -15.2));
    builder.line_to((4.0, 20.0));
    builder.horiz_rel(15.2);
    builder.line_to((24.0, 4.0));
    builder.line_to_rel((4.8, 16.0));
    builder.horiz(44.0);
    builder.line_to_rel((-12.3, 8.8));
    builder.line_to((36.35, 44.0));
    builder.line_to((24.0, 34.6));
    builder.close();

    builder
}

impl App for Editor {
    fn init() -> Self {
        //builder.move_to(Vec2 { x: 24.0, y: 38.0 });

        Self {
            path: star_builder().build(),
        }
    }

    fn on_draw(&mut self) {
        let position = Vec2::new(254., 34.);
        let size = Vec2::new(500.0, 500.);

        run_draw_command(&DrawCommand::Rect {
            left: position.x,
            top: position.y,
            width: size.x,
            height: size.y,
            fill: None,
            stroke: Some(Stroke::new(COLOR_BLACK, 2.0)),
        });

        DrawCommand::path_stroke(
            self.path.clone(),
            (48., 48.).into(),
            position,
            size,
            Stroke::new(COLOR_RED, 4.),
        )
        .run();
    }
}

fn main() -> Result<(), ()> {
    init_app::<Editor>()?.run();
    Ok(())
}
