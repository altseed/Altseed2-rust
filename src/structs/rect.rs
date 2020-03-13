use crate::prelude::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn new_vec(pos: Vector2<T>, size: Vector2<T>) -> Self {
        Rect {
            x: pos.x,
            y: pos.y,
            width: size.x,
            height: size.y,
        }
    }
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
