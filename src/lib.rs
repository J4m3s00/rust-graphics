use app::App;
use bindings::{
    c_clean_up_editor, c_draw_circle, c_draw_circle_outline, c_draw_line, c_draw_rect, c_draw_text,
    c_poll_events, c_post_update_application, c_pre_update_application, c_start_application,
    InitApp,
};
use draw_command::DrawCommand;
use events::app_events::AppEvent;

pub mod app;
pub(crate) mod bindings;
pub mod circle;
pub mod color;
pub mod draw_command;
pub mod events;
pub mod font;
pub mod rect;
pub mod text;
pub mod vec;

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
                0.,
                0.,
                0.0,
                0.0,
                fill.is_some() as i32,
                stroke.is_some() as i32,
                fill.as_ref().map(|f| f.color.as_int()).unwrap_or(0),
                stroke.as_ref().map(|s| s.color.as_int()).unwrap_or(0),
                stroke.as_ref().map(|s| s.width).unwrap_or(0.0),
            )
        },
        DrawCommand::Circle {
            center,
            radius,
            fill,
            stroke,
        } => unsafe {
            if let Some(fill) = fill {
                c_draw_circle(center.x, center.y, *radius, fill.color.as_int());
            }
            if let Some(stroke) = stroke {
                c_draw_circle_outline(
                    center.x,
                    center.y,
                    *radius,
                    stroke.width,
                    stroke.color.as_int(),
                );
            }
        },
        DrawCommand::Line {
            x1,
            y1,
            x2,
            y2,
            stroke,
        } => unsafe {
            c_draw_line(*x1, *y1, *x2, *y2, stroke.width, stroke.color.as_int());
        },
        DrawCommand::Text {
            font,
            text,
            position,
            color,
        } => unsafe {
            let c_msg = std::ffi::CString::new(text.as_str())
                .unwrap_or(std::ffi::CString::new("ERROR Converting string").unwrap());
            c_draw_text(
                font.handle(),
                position.x,
                position.y,
                c_msg.as_ptr(),
                color.as_int(),
            );
        },
    }
}

pub fn init_app<A: App>() -> Result<A, ()> {
    let c_msg = match std::ffi::CString::new("Hello World") {
        Ok(s) => s,
        Err(_e) => return Err(()),
    };
    let init_app = InitApp {
        title: c_msg.as_ptr(),
    };
    if unsafe { c_start_application(&init_app) } != 0 {
        println!("Error starting application");
        return Err(());
    }
    Ok(A::init())
}

pub fn run_app<A: App>(mut app: A) {
    'app: loop {
        app.on_update();

        'event: loop {
            let event = unsafe { c_poll_events() };
            if event == std::ptr::null_mut() {
                break 'event;
            }

            let event = AppEvent::from(unsafe { *event });
            match event {
                AppEvent::None => {
                    break 'event;
                }
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
