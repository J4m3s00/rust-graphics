use rust_graphics::{run_app, app::App};

struct Editor;

impl App for Editor {
    fn on_start(&mut self) {
        println!("Hello, world!");
    }

}

fn main() {
    run_app(Editor)
}