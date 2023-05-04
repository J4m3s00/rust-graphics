use app::App;
use bindings::{c_pre_update_application, c_post_update_application, c_clean_up_editor, c_draw_rect, c_draw_circle, c_draw_line, c_draw_text, c_get_current_font_size, InitApp, c_start_application, AppEventType_AppEventType_Quit};
use draw_command::DrawCommand;

pub mod app;
pub mod draw_command;
pub mod rect;
pub mod vec;
pub mod text;
pub mod color;
pub mod circle;
mod bindings;


pub enum AppEvent {
    None,
    Quit,
}

pub fn update_editor<T>(mut render_func: T) -> AppEvent
where
    T: FnMut(),
{
    let event = unsafe { *c_pre_update_application() };
    render_func();
    unsafe { c_post_update_application() };
    match event.type_ {
        AppEventType_AppEventType_Quit => AppEvent::Quit,
        _ => AppEvent::None,
    }
}

pub fn quit_editor() {
    unsafe { c_clean_up_editor() };
}

pub fn run_draw_command(command: &DrawCommand) {
    match command {
        DrawCommand::Rect {
            left,
            top,
            width,
            height,
            fill,
            stroke,
        } => unsafe {
            c_draw_rect(
                *left,
                *top,
                *width,
                *height,
                width / 2.,
                height / 2.,
                0.0,
                0.0,
                fill.is_some() as i32,
                stroke.is_some() as i32,
                fill.as_ref().map(|f| f.color.as_int()).unwrap_or(0),
                stroke.as_ref().map(|s| s.color.as_int()).unwrap_or(0),
                stroke.as_ref().map(|s| s.width).unwrap_or(0.0),
            )
        },
        DrawCommand::Circle(circle) => unsafe {
            c_draw_circle(circle.center.x, circle.center.y, circle.radius);
        },
        DrawCommand::Line { x1, y1, x2, y2 } => unsafe {
            c_draw_line(*x1, *y1, *x2, *y2);
        },
        DrawCommand::Text(text) => unsafe {
            let c_msg = std::ffi::CString::new(text.text.as_str())
                .unwrap_or(std::ffi::CString::new("ERROR Converting string").unwrap());
            c_draw_text(
                text.position.x,
                text.position.y - text.style.font_size,
                c_msg.as_ptr(),
            );
        },
    }
}

pub fn get_current_font_size() -> f32 {
    unsafe { c_get_current_font_size() }
}

pub fn run_app<A: App>(mut app: A) {
    let c_msg = match std::ffi::CString::new("Hello World") {
        Ok(s) => s,
        Err(_e) => return,
    };
    let init_app = InitApp {
        title: c_msg.as_ptr(),
    };
    if unsafe { c_start_application(&init_app) } != 0 {
        println!("Error starting application");
        return;
    }

    app.on_start();
    loop {
        app.on_update();
        match update_editor(|| app.on_update()) {
            AppEvent::Quit => break,
            AppEvent::None => {}
        }
    }
    app.on_stop();
    quit_editor();
}
