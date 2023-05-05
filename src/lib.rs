use app::App;
use bindings::{
    c_clean_up_editor, c_draw_circle, c_draw_line, c_draw_rect, c_draw_text,
    c_get_current_font_size, c_poll_events, c_post_update_application, c_pre_update_application,
    c_start_application, AppEventType_AppEventType_KeyDown, AppEventType_AppEventType_KeyUp,
    AppEventType_AppEventType_MouseDown, AppEventType_AppEventType_MouseMove,
    AppEventType_AppEventType_MouseUp, AppEventType_AppEventType_Quit,
    AppEventType_AppEventType_WindowResize, InitApp,
};
use draw_command::DrawCommand;

pub mod app;
mod bindings;
pub mod circle;
pub mod color;
pub mod draw_command;
pub mod events;
pub mod rect;
pub mod text;
pub mod vec;

#[derive(Debug)]
pub enum AppEvent {
    None,
    Quit,

    WindowResize(i32, i32),

    MouseDown { key: i32, x: i32, y: i32 },
    MouseMove { x: i32, y: i32 },
    MouseUp { key: i32, x: i32, y: i32 },

    KeyDown(i32, u16),
    KeyUp(i32, u16),
    //TextInput(String),
}

impl From<bindings::AppEvent> for AppEvent {
    fn from(value: bindings::AppEvent) -> Self {
        match value.type_ {
            AppEventType_AppEventType_Quit => AppEvent::Quit,
            AppEventType_AppEventType_WindowResize => AppEvent::WindowResize(value.x, value.y),
            AppEventType_AppEventType_KeyDown => AppEvent::KeyDown(value.key, value.code as u16),
            AppEventType_AppEventType_KeyUp => AppEvent::KeyUp(value.key, value.code as u16),
            AppEventType_AppEventType_MouseDown => AppEvent::MouseDown {
                key: value.key,
                x: value.x,
                y: value.y,
            },
            AppEventType_AppEventType_MouseUp => AppEvent::MouseUp {
                key: value.key,
                x: value.x,
                y: value.y,
            },
            AppEventType_AppEventType_MouseMove => AppEvent::MouseMove {
                x: value.x,
                y: value.y,
            },
            _ => AppEvent::None,
        }
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
    'app: loop {
        app.on_update();

        'event: loop {
            let event = unsafe { c_poll_events() };
            if event == std::ptr::null_mut() {
                break 'event;
            }

            let event = AppEvent::from(unsafe { *event });
            match event {
                AppEvent::None => {}
                AppEvent::Quit => {
                    app.on_event(event);
                    break 'app;
                }
                _ => {
                    app.on_event(event);
                }
            }
        }

        unsafe { c_pre_update_application() };
        app.on_draw();
        unsafe { c_post_update_application() };
    }
    app.on_stop();
    quit_editor();
}
