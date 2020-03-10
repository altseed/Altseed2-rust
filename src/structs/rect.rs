#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl From<super::RectI> for Rect<i32> {
    fn from(item: super::RectI) -> Self {
        Self {
            x: item.x,
            y: item.y,
            width: item.width,
            height: item.height,
        }
    }
}
impl Into<super::RectI> for Rect<i32> {
    fn into(self) -> super::RectI {
        super::RectI {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

impl From<super::RectF> for Rect<f32> {
    fn from(item: super::RectF) -> Self {
        Self {
            x: item.x,
            y: item.y,
            width: item.width,
            height: item.height,
        }
    }
}
impl Into<super::RectF> for Rect<f32> {
    fn into(self) -> super::RectF {
        super::RectF {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}
