#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub static COLOR_BLACK: Color = Color {r: 0, g: 0, b: 0, a: 255};
pub static COLOR_WHITE: Color = Color {r: 255, g: 255, b: 255, a: 255};
pub static COLOR_RED: Color = Color {r: 235, g: 87, b: 87, a: 255};
pub static COLOR_GREEN: Color = Color {r: 87, g: 235, b: 87, a: 255};
pub static COLOR_BLUE: Color = Color {r: 87, g: 87, b: 235, a: 255};
pub static COLOR_YELLOW: Color = Color {r: 235, g: 235, b: 87, a: 255};
pub static COLOR_MAGENTA: Color = Color {r: 235, g: 87, b: 235, a: 255};
pub static COLOR_CYAN: Color = Color {r: 87, g: 235, b: 235, a: 255};
pub static COLOR_TRANSPARENT: Color = Color {r: 0, g: 0, b: 0, a: 0};

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn as_int(&self) -> u32 {
        (self.a as u32) << 24 | (self.b as u32) << 16 | (self.g as u32) << 8 | (self.r as u32)
    }
}

impl<S> From<S> for Color
where
    S: Into<String>,
{
    fn from(str: S) -> Self {
        let str = str.into();
        if str.len() < 5 {
            panic!("Invalid color string: {}", str);
        }
        let mut chars = str.chars();
        let r = chars.next().unwrap().to_digit(16).unwrap() * 16
            + chars.next().unwrap().to_digit(16).unwrap();
        let g = chars.next().unwrap().to_digit(16).unwrap() * 16
            + chars.next().unwrap().to_digit(16).unwrap();
        let b = chars.next().unwrap().to_digit(16).unwrap() * 16
            + chars.next().unwrap_or('0').to_digit(16).unwrap();
        let a = if str.len() > 6 {
            chars.next().unwrap().to_digit(16).unwrap() * 16
                + chars.next().unwrap().to_digit(16).unwrap()
        } else {
            255
        };
        Self {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: a as u8,
        }
    }
}
