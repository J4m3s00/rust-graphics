use rust_graphics::{
    app::App,
    color::COLOR_BLACK,
    draw_command::{DrawCommand, Stroke},
    init_app,
    path_builder::{Path, PathBuilder},
    run_draw_command,
};

struct Editor {
    path: Path,
}

impl App for Editor {
    fn init() -> Self {
        Self {
            path: PathBuilder::new()
                .stroke(Some(Stroke::new(COLOR_BLACK, 5.0)))
                //.fill(Some(Fill::new(COLOR_MAGENTA)))
                .move_to((400.0, 400.0))
                .cubic_to((400.0, 200.0), (500.0, 200.0), (500.0, 400.0))
                .move_to((100, 100))
                .line_to((200, 100))
                .line_to((200, 200))
                .build(),
        }
    }

    fn on_draw(&mut self) {
        run_draw_command(&DrawCommand::Path(self.path.clone()));
    }
}

fn main() -> Result<(), ()> {
    init_app::<Editor>()?.run();
    Ok(())
}
