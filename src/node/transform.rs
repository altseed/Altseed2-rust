use crate::{math::Matrix44, prelude::*};
use std::fmt;

/// 変形行列を保持するための構造体
#[derive(Debug)]
pub struct Transform {
    pos: Vector2<f32>,
    scale: Vector2<f32>,
    angle: f32,
    center: Vector2<f32>,

    updated: bool,
    transform: Matrix44<f32>,
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[pos: ({}, {}), center: ({}, {}), scale: ({}, {}), angle: {}]",
            self.pos.x,
            self.pos.y,
            self.center.x,
            self.center.y,
            self.scale.x,
            self.scale.y,
            self.angle
        )
    }
}

impl Transform {
    pub(crate) fn new() -> Transform {
        Transform {
            pos: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
            angle: 0.0,
            center: Vector2::new(0.0, 0.0),
            updated: false,
            transform: Matrix44::identity().clone(),
        }
    }

    pub(crate) fn calculate(&self) -> Matrix44<f32> {
        Matrix44::translation(self.center.x, self.center.y, 0.0)
            * Matrix44::translation(self.pos.x, self.pos.y, 0.0)
            * Matrix44::rotation_z(self.angle)
            * Matrix44::scale(self.scale.x, self.scale.y, 1.0)
            * Matrix44::translation(-self.center.x, -self.center.y, 0.0)
    }

    pub(crate) fn update(&mut self, ancestors: Option<&crate::math::Matrix44<f32>>) -> bool {
        match (self.updated, ancestors) {
            (_, Some(p)) => {
                self.transform = p * &self.calculate();
                self.updated = false;
                true
            }
            (true, None) => {
                if self.updated {
                    self.transform = self.calculate();
                    self.updated = false;
                }
                true
            }
            (false, None) => false,
        }
    }

    pub(crate) fn get(&self) -> &Matrix44<f32> {
        &self.transform
    }

    pub(crate) fn updated(&mut self) {
        self.updated = true;
    }
}

pub trait HasTransform {
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;

    /// Position (位置)
    fn pos(&self) -> Vector2<f32> {
        self.transform().pos
    }

    /// Position (位置)
    fn pos_mut(&mut self) -> &mut Vector2<f32> {
        self.transform_mut().updated = true;
        &mut self.transform_mut().pos
    }

    /// Scale (拡大率)
    fn scale(&self) -> Vector2<f32> {
        self.transform().scale
    }

    /// Scale (拡大率)
    fn scale_mut(&mut self) -> &mut Vector2<f32> {
        self.transform_mut().updated = true;
        &mut self.transform_mut().scale
    }

    /// Angle (角度)
    fn angle(&self) -> f32 {
        self.transform().angle
    }

    /// Angle (角度)
    fn angle_mut(&mut self) -> &mut f32 {
        self.transform_mut().updated = true;
        &mut self.transform_mut().angle
    }

    /// Center Position (中央位置)
    fn center(&self) -> Vector2<f32> {
        self.transform().center
    }

    /// Center Position (中央位置)
    fn center_mut(&mut self) -> &mut Vector2<f32> {
        self.transform_mut().updated = true;
        &mut self.transform_mut().center
    }
}

impl HasTransform for Transform {
    fn transform(&self) -> &Transform {
        self
    }

    fn transform_mut(&mut self) -> &mut Transform {
        self
    }
}
