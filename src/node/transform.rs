use crate::prelude::*;

#[derive(Debug)]
pub struct Transform {
    pos: Vector2<f32>,
    scale: Vector2<f32>,
    angle: f32,
    center: Vector2<f32>,

    updated: bool,
    transform: Matrix44<f32>,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            pos: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
            angle: 0.0,
            center: Vector2::new(0.0, 0.0),
            updated: false,
            transform: Matrix44::identity(),
        }
    }
}

impl Transform {
    pub fn pos(&self) -> Vector2<f32> {
        self.pos
    }

    pub fn pos_mut(&mut self) -> &mut Vector2<f32> {
        self.updated = true;
        &mut self.pos
    }

    pub fn scale(&self) -> Vector2<f32> {
        self.scale
    }

    pub fn scale_mut(&mut self) -> &mut Vector2<f32> {
        self.updated = true;
        &mut self.scale
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn angle_mut(&mut self) -> &mut f32 {
        self.updated = true;
        &mut self.angle
    }

    pub fn center(&self) -> Vector2<f32> {
        self.center
    }

    pub fn center_mut(&mut self) -> &mut Vector2<f32> {
        self.updated = true;
        &mut self.center
    }

    pub(crate) fn update(&mut self) {
        let mut t = Matrix44::identity();
        t *= Matrix44::translation(self.center.x, self.center.y, 0.0);
        t *= Matrix44::translation(self.pos.x, self.pos.y, 0.0);
        t *= Matrix44::rotation_z(self.angle);
        t *= Matrix44::scale(self.scale.x, self.scale.y, 1.0);
        t *= Matrix44::translation(-self.center.x, -self.center.y, 0.0);

        self.transform = t;
        self.updated = false;
    }

    pub(crate) fn is_updated(&self) -> bool {
        self.updated
    }

    pub(crate) fn get(&self) -> Matrix44<f32> {
        self.transform.clone()
    }
}
