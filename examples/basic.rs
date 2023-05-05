use rust_graphics::{
    app::App,
    color::{COLOR_BLACK, COLOR_CYAN, COLOR_GREEN},
    draw_command::{DrawCommand, Fill, Stroke},
    events::app_events::AppEvent,
    rect::Rect,
    run_app, run_draw_command, vec::Vec2,
};

struct Editor {
    rect: Rect,
    mouse_pos: Vec2,
}

impl Editor {
    fn new() -> Self {
        Self {
            rect: Rect::new_from_xy(24., 24., 100., 100.),
            mouse_pos: Vec2::zero(),
        }
    }
}

impl App for Editor {
    fn on_start(&mut self) {}

    fn on_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::KeyDown(_, _mods) => {}
            AppEvent::WindowResize(x, y) => {
                self.rect = Rect::new(24., 24., x as f32 - 24., y as f32 - 24.)
            }
            AppEvent::MouseMove { x, y } => {
                self.mouse_pos = Vec2::new(x as f32, y as f32);
            }
            _ => {}
        }
    }

    fn on_draw(&mut self) {
        run_draw_command(&DrawCommand::Rect {
            left: self.rect.left,
            top: self.rect.top,
            width: self.rect.width(),
            height: self.rect.height(),
            fill: Some(Fill { color: if self.rect.contains(self.mouse_pos) { COLOR_CYAN } else { COLOR_GREEN } }),
            stroke: Some(Stroke {
                width: 2.,
                color: COLOR_BLACK,
            }),
        })
    }
}

fn main() {
    run_app(Editor::new())
}
