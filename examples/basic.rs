use rust_graphics::{app::App, events::app_events::KeyMods, run_app, AppEvent, run_draw_command, draw_command::{DrawCommand, Stroke}, color::{Color, COLOR_BLACK}};

struct Editor;

impl App for Editor {
    fn on_start(&mut self) {}

    fn on_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::KeyDown(_, mods) => {
                let mods = KeyMods::from(mods);
                println!("KeyDown: {:?}", mods);
            }
            _ => {}
        }
    }

    fn on_draw(&mut self) {
        run_draw_command(&DrawCommand::Rect { left: 0., top: 0., width: 100., height: 100., fill: None, stroke: Some(Stroke { width: 2., color: COLOR_BLACK }) })
    }
}

fn main() {
    run_app(Editor)
}
