use rust_graphics::{
    app::App,
    color::COLOR_BLACK,
    draw_command::{DrawCommand, Stroke},
    init_app,
    path_builder::{Path, PathBuilder},
    rect::Rect,
    run_draw_command,
};

struct Editor {
    path: Path,
}

impl App for Editor {
    fn init() -> Self {
        let mut builder = PathBuilder::new();
        builder.stroke(Some(Stroke::new(COLOR_BLACK, 5.0)));
        //builder.fill(Some(Fill::new(COLOR_MAGENTA)));
        builder.move_to((400.0, 400.0));
        builder.cubic_to((400.0, 200.0), (500.0, 200.0), (500.0, 400.0));
        builder.move_to((100, 100));
        builder.line_to((200, 100));
        builder.line_to((200, 200));

        Self {
            path: builder.build(),
        }
    }

    fn on_draw(&mut self) {
        let rect = Rect::new_from_xy(200., 200., 500., 500.);

        run_draw_command(&DrawCommand::Rect {
            left: rect.left,
            top: rect.top,
            width: rect.width(),
            height: rect.height(),
            fill: None,
            stroke: Some(Stroke::new(COLOR_BLACK, 2.0)),
        });

        run_draw_command(&DrawCommand::Path(
            self.path.clone(),
            (600., 500.).into(),
            rect.clone(),
        ));
    }
}

fn main() -> Result<(), ()> {
    init_app::<Editor>()?.run();
    Ok(())
}
