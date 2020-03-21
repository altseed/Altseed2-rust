use crate::{math::Matrix44, prelude::*};

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

    /// Position (位置)
    pub fn pos(&self) -> Vector2<f32> {
        self.pos
    }

    /// Position (位置)
    pub fn pos_mut(&mut self) -> &mut Vector2<f32> {
        self.updated = true;
        &mut self.pos
    }

    /// Scale (拡大率)
    pub fn scale(&self) -> Vector2<f32> {
        self.scale
    }

    /// Scale (拡大率)
    pub fn scale_mut(&mut self) -> &mut Vector2<f32> {
        self.updated = true;
        &mut self.scale
    }

    /// Angle (角度)
    pub fn angle(&self) -> f32 {
        self.angle
    }

    /// Angle (角度)
    pub fn angle_mut(&mut self) -> &mut f32 {
        self.updated = true;
        &mut self.angle
    }

    /// Center Position (中央位置)
    pub fn center(&self) -> Vector2<f32> {
        self.center
    }

    /// Center Position (中央位置)
    pub fn center_mut(&mut self) -> &mut Vector2<f32> {
        self.updated = true;
        &mut self.center
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
