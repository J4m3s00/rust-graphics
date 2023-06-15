use crate::{events::app_events::AppEvent, run_app};

pub trait App {
    fn init() -> Self;
    fn on_start(&mut self) {}
    fn on_update(&mut self) {}
    fn on_event(&mut self, _event: AppEvent) {}
    fn on_stop(&mut self) {}
    fn on_draw(&mut self) {}

    fn run(self)
    where
        Self: Sized,
    {
        run_app(self);
    }
}
