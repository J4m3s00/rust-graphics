use rust_graphics::{
    app::App,
    color::{COLOR_BLACK, COLOR_CYAN},
    draw_command::{DrawCommand, Fill, Stroke},
    events::app_events::AppEvent,
    font::Font,
    init_app, run_draw_command,
    vec::Vec2,
};

struct Editor {
    font: Font,
    window_size: Vec2,
    mouse_pos: Vec2,
    mouse_down: bool,
}

impl App for Editor {
    fn init() -> Self {
        Self {
            font: Font::from_file("Roboto.ttf", 24),
            window_size: Vec2::new(100.0, 100.0),
            mouse_pos: Vec2::zero(),
            mouse_down: false,
        }
    }

    fn on_start(&mut self) {}

    fn on_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::KeyDown(_, _mods) => {}
            AppEvent::WindowResize(x, y) => {
                self.window_size.x = x as f32;
                self.window_size.y = y as f32;
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
            left: 24.0,
            top: 24.0,
            width: self.window_size.x - 48.0,
            height: self.window_size.y - 48.0,
            fill: Some(Fill { color: COLOR_CYAN }),
            stroke: Some(Stroke {
                width: 2.,
                color: COLOR_BLACK,
            }),
        });
        run_draw_command(&DrawCommand::Text {
            font: self.font.clone(),
            text: "Hello World!gg".into(),
            position: self.window_size / 2.0,
            color: COLOR_BLACK,
            stroke: None,
        });
        run_draw_command(&DrawCommand::Line {
            x1: 24.0,
            y1: self.window_size.y / 2.0,
            x2: self.window_size.x - 24.0,
            y2: self.window_size.y / 2.0,
            stroke: Stroke {
                width: 1.,
                color: COLOR_BLACK,
            },
        });
    }
}

fn main() -> Result<(), ()> {
    init_app::<Editor>()?.run();
    Ok(())
}
