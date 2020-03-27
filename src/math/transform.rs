use super::{vector::Vector2, Matrix44};

/// 変形行列を保持するための構造体
#[derive(Debug)]
pub struct Transform {
    pos: Vector2<f32>,
    scale: Vector2<f32>,
    angle: f32,
    center: Vector2<f32>,

    updated_parent: bool,
    parent: Matrix44<f32>,

    updated_local: bool,
    local: Matrix44<f32>,

    matrix: Matrix44<f32>,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            pos: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
            angle: 0.0,
            center: Vector2::new(0.0, 0.0),
            updated_parent: false,
            parent: Matrix44::identity(),
            updated_local: false,
            local: Matrix44::identity(),
            matrix: Matrix44::identity(),
        }
    }

    // TODO
    fn calc_local(&self) -> Matrix44<f32> {
        Matrix44::translation(self.center.x, self.center.y, 0.0)
            * Matrix44::translation(self.pos.x, self.pos.y, 0.0)
            * Matrix44::rotation_z(self.angle)
            * Matrix44::scale(self.scale.x, self.scale.y, 1.0)
            * Matrix44::translation(-self.center.x, -self.center.y, 0.0)
    }

    pub fn update(&mut self) -> Option<&Matrix44<f32>> {
        match (self.updated_local, self.updated_parent) {
            (false, false) => return None,
            (true, true) => {
                self.local = self.calc_local();
                self.matrix = &self.parent * &self.local;
                self.updated_local = false;
                self.updated_parent = false;
            }
            (true, false) => {
                self.local = self.calc_local();
                self.matrix = &self.parent * &self.local;
                self.updated_local = false;
            }
            (false, true) => {
                self.matrix = &self.parent * &self.local;
                self.updated_parent = false;
            }
        }

        Some(&self.matrix)
    }
}

pub trait HasTransform {
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;

    /// 変更があった際は再計算して、現在の行列を取得します。
    fn matrix(&mut self) -> &Matrix44<f32> {
        let t = self.transform_mut();
        t.update();
        &t.matrix
    }

    /// 親の行列
    fn parent(&self) -> &Matrix44<f32> {
        &self.transform().parent
    }

    /// 親の行列
    fn parent_mut(&mut self) -> &mut Matrix44<f32> {
        let t = self.transform_mut();
        t.updated_parent = true;
        &mut t.parent
    }

    /// Position (位置)
    fn pos(&self) -> Vector2<f32> {
        self.transform().pos
    }

    /// Position (位置)
    fn pos_mut(&mut self) -> &mut Vector2<f32> {
        let t = self.transform_mut();
        t.updated_local = true;
        &mut t.pos
    }

    /// Scale (拡大率)
    fn scale(&self) -> Vector2<f32> {
        self.transform().scale
    }

    /// Scale (拡大率)
    fn scale_mut(&mut self) -> &mut Vector2<f32> {
        let t = self.transform_mut();
        t.updated_local = true;
        &mut t.scale
    }

    /// Angle (角度)
    fn angle(&self) -> f32 {
        self.transform().angle
    }

    /// Angle (角度)
    fn angle_mut(&mut self) -> &mut f32 {
        let t = self.transform_mut();
        t.updated_local = true;
        &mut t.angle
    }

    /// Center Position (中心位置)
    fn center(&self) -> Vector2<f32> {
        self.transform().center
    }

    /// Center Position (中心位置)
    fn center_mut(&mut self) -> &mut Vector2<f32> {
        let t = self.transform_mut();
        t.updated_local = true;
        &mut t.center
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
