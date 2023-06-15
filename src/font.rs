use crate::bindings::{
    c_font_get_line_height, c_font_get_line_top, c_font_get_text_height, c_font_get_text_width,
    c_load_font,
};

#[derive(Clone, Copy, Debug)]
pub struct Font {
    handle: u32,
    size: i32,
}

impl Font {
    pub fn from_file(path: impl AsRef<str>, size: i32) -> Self {
        let c_msg = std::ffi::CString::new(path.as_ref())
            .unwrap_or(std::ffi::CString::new("ERROR Converting string").unwrap());
        let handle = unsafe { c_load_font(c_msg.as_ptr(), size) };

        Self { handle, size }
    }

    pub fn get_text_width(&self, text: impl AsRef<str>) -> i32 {
        let c_msg = std::ffi::CString::new(text.as_ref())
            .unwrap_or(std::ffi::CString::new("ERROR Converting string").unwrap());
        unsafe { c_font_get_text_width(self.handle, c_msg.as_ptr()) }
    }

    pub fn get_text_height(&self, text: impl AsRef<str>) -> i32 {
        let c_msg = std::ffi::CString::new(text.as_ref())
            .unwrap_or(std::ffi::CString::new("ERROR Converting string").unwrap());
        unsafe { c_font_get_text_height(self.handle, c_msg.as_ptr()) }
    }

    pub fn get_text_size(&self, text: impl AsRef<str>) -> (i32, i32) {
        (
            self.get_text_width(text.as_ref()),
            self.get_text_height(text.as_ref()),
        )
    }

    pub fn get_line_height(&self) -> i32 {
        unsafe { c_font_get_line_height(self.handle) }
    }

    pub fn get_line_top(&self) -> i32 {
        unsafe { c_font_get_line_top(self.handle) }
    }

    pub fn handle(&self) -> u32 {
        self.handle
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}
