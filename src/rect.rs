use crate::vec::Vec2;

#[derive(Default, Clone, Copy, Debug)]
pub struct Rect {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub origin_x: f32,
    pub origin_y: f32,
}

impl Rect {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        assert!(left < right, "left must be less than right");
        assert!(top < bottom, "less must be bottom than top");
        Self {
            top,
            left,
            right,
            bottom,
            origin_x: (right - left) / 2.0,
            origin_y: (top - bottom) / 2.0,
        }
    }

    pub fn new_from_xy(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            top: y,
            left: x,
            right: x + width,
            bottom: y + height,
            origin_x: width / 2.0,
            origin_y: height / 2.0,
        }
    }

    pub fn top_left(&self) -> Vec2 {
        Vec2::new(self.left, self.top)
    }

    pub fn set_left(mut self, left: f32) -> Self {
        assert!(left < self.right, "left must be less than right");
        self.left = left;
        self.origin_x = self.width() / 2.0;
        self
    }

    pub fn set_right(mut self, right: f32) -> Self {
        assert!(self.left < right, "left must be less than right");
        self.right = right;
        self.origin_x = self.width() / 2.0;
        self
    }

    pub fn set_top(mut self, top: f32) -> Self {
        assert!(self.bottom > top, "bottom must be less than top");
        self.top = top;
        self.origin_y = self.height() / 2.0;
        self
    }

    pub fn set_bottom(mut self, bottom: f32) -> Self {
        assert!(bottom > self.top, "bottom must be less than top");
        self.bottom = bottom;
        self.origin_y = self.height() / 2.0;
        self
    }

    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(self.left + self.width() / 2., self.top + self.height() / 2.)
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.left
            && point.x <= self.right
            && point.y >= self.top
            && point.y <= self.bottom
    }
}
