use crate::events::app_events::AppEvent;


pub trait App {
    fn on_start(&mut self) {}
    fn on_update(&mut self) {}
    fn on_event(&mut self, _event: AppEvent) {}
    fn on_stop(&mut self) {}
    fn on_draw(&mut self) {}
}