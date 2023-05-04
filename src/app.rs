pub trait App {
    fn on_start(&mut self) {}
    fn on_update(&mut self) {}
    fn on_stop(&mut self) {}
    fn on_draw(&mut self) {}
}