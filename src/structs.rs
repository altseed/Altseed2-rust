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
pub(crate) struct Color {
    pub(crate) r: c_uchar,
    pub(crate) g: c_uchar,
    pub(crate) b: c_uchar,
    pub(crate) a: c_uchar,
}

pub mod color;
pub mod rect;
pub mod vertex;
