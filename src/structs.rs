use crate::math::*;
use std::os::raw::c_uchar;

#[repr(C)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vertex {
    pub pos: Vector3<f32>,
    pub col: Color,
    pub u_v1: Vector2<f32>,
    pub u_v2: Vector2<f32>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Color {
    pub r: c_uchar,
    pub g: c_uchar,
    pub b: c_uchar,
    pub a: c_uchar,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn new3(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
}
