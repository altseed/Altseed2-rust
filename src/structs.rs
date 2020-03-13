use std::os::raw::*;

#[repr(C)]
pub(crate) struct Vector2I {
    pub(crate) x: c_int,
    pub(crate) y: c_int,
}

#[repr(C)]
pub(crate) struct Vector2F {
    pub(crate) x: c_float,
    pub(crate) y: c_float,
}

#[repr(C)]
pub(crate) struct Vector3I {
    pub(crate) x: c_int,
    pub(crate) y: c_int,
    pub(crate) z: c_int,
}

#[repr(C)]
pub(crate) struct Vector3F {
    pub(crate) x: c_float,
    pub(crate) y: c_float,
    pub(crate) z: c_float,
}

#[repr(C)]
pub(crate) struct Vector4I {
    pub(crate) x: c_int,
    pub(crate) y: c_int,
    pub(crate) z: c_int,
    pub(crate) w: c_int,
}

#[repr(C)]
pub(crate) struct Vector4F {
    pub(crate) x: c_float,
    pub(crate) y: c_float,
    pub(crate) z: c_float,
    pub(crate) w: c_float,
}

#[repr(C)]
pub(crate) struct RectI {
    pub(crate) x: c_int,
    pub(crate) y: c_int,
    pub(crate) width: c_int,
    pub(crate) height: c_int,
}

#[repr(C)]
pub(crate) struct RectF {
    pub(crate) x: c_float,
    pub(crate) y: c_float,
    pub(crate) width: c_float,
    pub(crate) height: c_float,
}

#[repr(C)]
pub(crate) struct Matrix44I {
    pub(crate) values: [[c_int; 4]; 4],
}

#[repr(C)]
pub(crate) struct Matrix44F {
    pub(crate) values: [[c_float; 4]; 4],
}

#[repr(C)]
pub(crate) struct Vertex {
    pub(crate) pos: Vector3F,
    pub(crate) col: Color,
    pub(crate) u_v1: Vector2F,
    pub(crate) u_v2: Vector2F,
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

pub mod rect;
pub mod vertex;
