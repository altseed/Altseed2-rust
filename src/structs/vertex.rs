use super::*;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vertex {
    pub pos: Vector3<f32>,
    pub col: Color,
    pub u_v1: Vector2<f32>,
    pub u_v2: Vector2<f32>,
}

impl From<super::Vertex> for Vertex {
    fn from(item: super::Vertex) -> Self {
        Self {
            pos: item.pos.into(),
            col: item.col,
            u_v1: item.u_v1.into(),
            u_v2: item.u_v2.into(),
        }
    }
}

impl Into<super::Vertex> for Vertex {
    fn into(self) -> super::Vertex {
        super::Vertex {
            pos: self.pos.into(),
            col: self.col,
            u_v1: self.u_v1.into(),
            u_v2: self.u_v2.into(),
        }
    }
}
