use rust_graphics::{
    app::App,
    color::{COLOR_BLACK, COLOR_CYAN, COLOR_GREEN},
    draw_command::{DrawCommand, Fill, Stroke},
    events::app_events::AppEvent,
    font::Font,
    rect::Rect,
    run_app, run_draw_command,
    vec::Vec2,
};

struct Editor {
    font: Option<Font>,
    rect: Rect,
    mouse_pos: Vec2,
    mouse_down: bool,
}

impl Editor {
    fn new() -> Self {
        Self {
            font: None,
            rect: Rect::new_from_xy(24., 24., 100., 100.),
            mouse_pos: Vec2::zero(),
            mouse_down: false,
        }
    }
}

impl App for Editor {
    fn on_start(&mut self) {
        self.font = Some(Font::from_file("Roboto.ttf", 24));
    }

    fn on_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::KeyDown(_, _mods) => {}
            AppEvent::WindowResize(x, y) => {
                self.rect = Rect::new(24., 24., x as f32 - 24., y as f32 - 24.)
            }
            AppEvent::MouseMove { x, y } => {
                self.mouse_pos = Vec2::new(x as f32, y as f32);
            }
            AppEvent::MouseDown { .. } => {
                self.mouse_down = true;
            }
            AppEvent::MouseUp { .. } => {
                self.mouse_down = false;
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
            fill: Some(Fill {
                color: if self.rect.contains(self.mouse_pos) {
                    if self.mouse_down {
                        COLOR_BLACK
                    } else {
                        COLOR_CYAN
                    }
                } else {
                    COLOR_GREEN
                },
            }),
            stroke: Some(Stroke {
                width: 2.,
                color: COLOR_BLACK,
            }),
        });
        if let Some(font) = &self.font {
            run_draw_command(&DrawCommand::Text {
                font: font.clone(),
                text: "Hello World!gg".into(),
                position: self.rect.center(),
                color: COLOR_BLACK,
            });
        }
        run_draw_command(&DrawCommand::Line {
            x1: self.rect.left,
            y1: self.rect.center().y,
            x2: self.rect.right,
            y2: self.rect.center().y,
            stroke: Stroke {
                width: 1.,
                color: COLOR_BLACK,
            },
        });
    }
}

fn main() {
    run_app(Editor::new())
}
