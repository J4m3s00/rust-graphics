use crate::{
    bindings::{
        self, AppEventType_AppEventType_KeyDown, AppEventType_AppEventType_KeyUp,
        AppEventType_AppEventType_MouseDown, AppEventType_AppEventType_MouseMove,
        AppEventType_AppEventType_MouseUp, AppEventType_AppEventType_Quit,
        AppEventType_AppEventType_WindowResize,
    },
    keycodes::KeyCode,
};

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
/// TODO: NEED TO MAKE MATCHING ON MODS BETTER.
/// WHEN CHECKING FOR SHORTCUT ALL OTHER MODS SHOULD BE FALSE, NOT JUST THE ONE BEING CHECKED.
/// THE CAPSLOCK CAN BE IGNORED THOUGH.

#[derive(Default, Debug)]
pub struct KeyMods {
    pub l_shift: bool,
    pub r_shift: bool,
    pub l_control: bool,
    pub r_control: bool,
    pub l_alt: bool, // Option on macOS
    pub r_alt: bool,
    pub l_system: bool, // Command on macOS, Windows on Windows
    pub r_system: bool,

    pub caps_lock: bool,
}

impl From<u16> for KeyMods {
    fn from(value: u16) -> Self {
        Self {
            l_shift: value & 0x0001 != 0,
            r_shift: value & 0x0002 != 0,
            l_control: value & 0x0040 != 0,
            r_control: value & 0x0080 != 0,
            l_alt: value & 0x0100 != 0,
            r_alt: value & 0x0200 != 0,
            l_system: value & 0x0400 != 0,
            r_system: value & 0x0800 != 0,
            caps_lock: value & 0x2000 != 0,
        }
    }
}

#[derive(Debug)]
pub enum AppEvent {
    None,
    Quit,

    WindowResize(i32, i32),

    MouseDown { key: i32, x: i32, y: i32 },
    MouseMove { x: i32, y: i32 },
    MouseUp { key: i32, x: i32, y: i32 },

    KeyDown(KeyCode, KeyMods),
    KeyUp(KeyCode, KeyMods),
    //TextInput(String),
}

impl From<bindings::AppEvent> for AppEvent {
    fn from(value: bindings::AppEvent) -> Self {
        #[allow(non_upper_case_globals)]
        match value.type_ {
            AppEventType_AppEventType_Quit => AppEvent::Quit,
            AppEventType_AppEventType_WindowResize => AppEvent::WindowResize(value.x, value.y),
            AppEventType_AppEventType_KeyDown => AppEvent::KeyDown(
                KeyCode::from(value.key as u32),
                KeyMods::from(value.code as u16),
            ),
            AppEventType_AppEventType_KeyUp => AppEvent::KeyUp(
                KeyCode::from(value.key as u32),
                KeyMods::from(value.code as u16),
            ),
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
