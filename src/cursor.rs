use crate::bindings;

#[derive(Debug, Clone, Copy)]
pub enum SystemCursor {
    Arrow,
    IBeam,
    Crosshair,
    Hand,
    HResize,
    VResize,
}

impl From<bindings::Cursor> for SystemCursor {
    fn from(cursor: bindings::Cursor) -> Self {
        match cursor {
            bindings::Cursor_Arrow => SystemCursor::Arrow,
            bindings::Cursor_IBeam => SystemCursor::IBeam,
            bindings::Cursor_Crosshair => SystemCursor::Crosshair,
            bindings::Cursor_Hand => SystemCursor::Hand,
            bindings::Cursor_HResize => SystemCursor::HResize,
            bindings::Cursor_VResize => SystemCursor::VResize,
            _ => SystemCursor::Arrow,
        }
    }
}

impl Into<bindings::Cursor> for SystemCursor {
    fn into(self) -> bindings::Cursor {
        match self {
            SystemCursor::Arrow => bindings::Cursor_Arrow,
            SystemCursor::IBeam => bindings::Cursor_IBeam,
            SystemCursor::Crosshair => bindings::Cursor_Crosshair,
            SystemCursor::Hand => bindings::Cursor_Hand,
            SystemCursor::HResize => bindings::Cursor_HResize,
            SystemCursor::VResize => bindings::Cursor_VResize,
        }
    }
}
