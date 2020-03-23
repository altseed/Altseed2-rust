use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::auto_generated_core_binding::{Graphics, Renderer};
use crate::math::{Matrix44, Vector2};
use crate::node::{
    list::{SortVec, SortedItem},
    CameraNode, HasTransform, Node, Polygon, Sprite, Text, Transform,
};
use crate::{engine::Engine, error::AltseedResult};

use crate as altseed2;
use crate::{create_node, define_node};

pub trait Drawn: HasTransform {
    fn into_node(self) -> Rc<RefCell<DrawnNode>>;
}

pub(crate) trait DrawnInternal: Drawn {
    fn on_drawn(&mut self, renderer: &mut Renderer);

    fn set_transform(&mut self, transform: Matrix44<f32>);

    fn relative_transform(&mut self, ancestors: Option<&Matrix44<f32>>) -> Option<Matrix44<f32>> {
        if self.transform_mut().update(ancestors) {
            let t = self.transform().get().clone();
            self.set_transform(t.clone());
            Some(t)
        } else {
            ancestors.map(Clone::clone)
        }
    }
}

#[derive(Debug)]
pub enum DrawnKind {
    Sprite(Sprite),
    Text(Text),
    Polygon(Polygon),
}

impl HasTransform for DrawnKind {
    fn transform(&self) -> &Transform {
        match self {
            Self::Sprite(x) => x.transform(),
            Self::Text(x) => x.transform(),
            Self::Polygon(x) => x.transform(),
        }
    }

    fn transform_mut(&mut self) -> &mut Transform {
        match self {
            Self::Sprite(x) => x.transform_mut(),
            Self::Text(x) => x.transform_mut(),
            Self::Polygon(x) => x.transform_mut(),
        }
    }
}

impl Drawn for DrawnKind {
    fn into_node(self) -> Rc<RefCell<DrawnNode>> {
        DrawnNode::new(self)
    }
}

impl DrawnInternal for DrawnKind {
    fn on_drawn(&mut self, renderer: &mut Renderer) {
        match self {
            Self::Sprite(x) => x.on_drawn(renderer),
            Self::Text(x) => x.on_drawn(renderer),
            Self::Polygon(x) => x.on_drawn(renderer),
        }
    }

    fn set_transform(&mut self, transform: Matrix44<f32>) {
        match self {
            Self::Sprite(x) => x.set_transform(transform),
            Self::Text(x) => x.set_transform(transform),
            Self::Polygon(x) => x.set_transform(transform),
        }
    }
}

impl From<Sprite> for DrawnKind {
    fn from(item: Sprite) -> Self {
        DrawnKind::Sprite(item)
    }
}

impl From<Text> for DrawnKind {
    fn from(item: Text) -> Self {
        DrawnKind::Text(item)
    }
}

impl From<Polygon> for DrawnKind {
    fn from(item: Polygon) -> Self {
        DrawnKind::Polygon(item)
    }
}

define_node! {
    /// 描画対象を保持する[Node](../node/trait.Node.html)を表します。
    pub struct DrawnNode {
        kind: DrawnKind,
        is_drawn: bool,
        z_order: i32,
        last_z_order: Option<i32>,
        camera_group: u32,
        last_camera_group: Option<u32>,
        weak: Option<Weak<RefCell<Self>>>,
    }
}

impl SortedItem<i32> for DrawnNode {
    fn sorted_key(&self) -> i32 {
        self.get_z_order()
    }

    fn is_key_updated(&self) -> bool {
        Some(self.z_order) != self.last_z_order
    }
}

impl Node for DrawnNode {
    fn on_connected_root(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        engine.add_drawn_node(self.weak.clone().unwrap(), self.z_order);
        Ok(())
    }

    fn on_removed(&mut self, _: &mut Engine) -> AltseedResult<()> {
        self.kind.transform_mut().updated();
        Ok(())
    }

    fn on_disconnected_root(&mut self, _: &mut Engine) -> AltseedResult<()> {
        self.kind.transform_mut().updated();
        Ok(())
    }
}

impl DrawnNode {
    pub(crate) fn is_camera_group_updated(&self) -> bool {
        Some(self.camera_group) != self.last_camera_group
    }

    /// 描画処理の前に行う
    pub(crate) fn before_drawn(&mut self, camera_nodes: &SortVec<u32, CameraNode>) {
        if self.is_camera_group_updated() {
            // EngineからCameraNodeを引っ張って来て追加する
            for node in camera_nodes.iter() {
                let rc = node.upgrade().expect("Already filtered");
                // 更新されていたらCameraNodeの方で処理されるのでskip
                if rc.borrow().is_key_updated() {
                    continue;
                }

                let target_group = rc.borrow().get_group();
                if self.camera_group & target_group == target_group {
                    rc.borrow_mut()
                        .add_drawn_node(self.weak.clone().unwrap(), self.z_order)
                }
            }
        }
    }

    pub(crate) fn on_drawn(&mut self, _: &mut Graphics, renderer: &mut Renderer) {
        if !self.get_is_drawn() {
            return;
        }

        self.kind.on_drawn(renderer);
    }

    pub(crate) fn after_drawn(&mut self) {
        self.last_z_order = Some(self.z_order);
        self.last_camera_group = Some(self.camera_group);
    }

    /// 新しい[DrawnNode](struct.DrawnNode.html)を作成します。
    pub fn new<T: Into<DrawnKind>>(kind: T) -> Rc<RefCell<Self>> {
        let rc = Rc::new(RefCell::new(create_node!(DrawnNode {
            kind: kind.into(),
            is_drawn: true,
            z_order: 0,
            last_z_order: None,
            camera_group: 0,
            last_camera_group: None,
            weak: None,
        })));

        rc.borrow_mut().weak = Some(Rc::downgrade(&rc));

        rc
    }

    /// 描画される種類
    pub fn kind(&self) -> &DrawnKind {
        &self.kind
    }

    /// 描画される種類
    pub fn kind_mut(&mut self) -> &mut DrawnKind {
        &mut self.kind
    }

    /// 描画順を取得します。
    pub fn get_z_order(&self) -> i32 {
        self.z_order
    }

    /// 描画順を設定します。
    pub fn set_z_order(&mut self, z_order: i32) -> &mut Self {
        self.z_order = z_order;
        self
    }

    /// 描画されるかどうかを取得します。
    pub fn get_is_drawn(&self) -> bool {
        self.is_drawn
    }

    /// 描画されるかどうかを設定します。
    pub fn set_is_drawn(&mut self, is_drawn: bool) -> &mut Self {
        self.is_drawn = is_drawn;
        self
    }

    /// カメラグループを取得します。
    pub fn get_camera_group(&self) -> u32 {
        self.camera_group
    }

    /// カメラグループを設定します。
    pub fn set_camera_group(&mut self, camera_group: u32) -> &mut Self {
        self.camera_group = camera_group;
        self
    }

    /// 位置を取得します。
    pub fn get_pos(&self) -> Vector2<f32> {
        self.kind.transform().pos()
    }

    /// 位置を設定します。
    pub fn set_pos(&mut self, pos: Vector2<f32>) -> &mut Self {
        *self.kind.transform_mut().pos_mut() = pos;
        self
    }

    /// 中心位置を取得します。
    pub fn get_center(&self) -> Vector2<f32> {
        self.kind.transform().center()
    }

    /// 中心位置を設定します。
    pub fn set_center(&mut self, center: Vector2<f32>) -> &mut Self {
        *self.kind.transform_mut().center_mut() = center;
        self
    }

    /// 拡大率を取得します。
    pub fn get_scale(&self) -> Vector2<f32> {
        self.kind.transform().scale()
    }

    /// 中心位置を設定します。
    pub fn set_scale(&mut self, scale: Vector2<f32>) -> &mut Self {
        *self.kind.transform_mut().scale_mut() = scale;
        self
    }

    /// 角度を取得します。
    pub fn get_angle(&self) -> f32 {
        self.kind.transform().angle()
    }

    /// 角度を設定します。
    pub fn set_angle(&mut self, angle: f32) -> &mut Self {
        *self.kind.transform_mut().angle_mut() = angle;
        self
    }
}

macro_rules! impl_drawn {
    ($name: ident) => {
        impl crate::node::HasTransform for $name {
            fn transform(&self) -> &crate::node::Transform {
                &self.trans
            }

            fn transform_mut(&mut self) -> &mut crate::node::Transform {
                &mut self.trans
            }
        }

        impl crate::node::Drawn for $name {
            fn into_node(self) -> Rc<RefCell<crate::node::DrawnNode>> {
                crate::node::DrawnNode::new(self)
            }
        }

        impl $name {
            /// 位置を設定します。
            pub fn with_pos(mut self, pos: crate::math::Vector2<f32>) -> Self {
                *self.pos_mut() = pos;
                self
            }

            /// 中心位置を設定します。
            pub fn with_center(mut self, center: crate::math::Vector2<f32>) -> Self {
                *self.center_mut() = center;
                self
            }

            /// 拡大率を設定します。
            pub fn with_scale(mut self, scale: crate::math::Vector2<f32>) -> Self {
                *self.scale_mut() = scale;
                self
            }

            /// 角度を設定します。
            pub fn with_angle(mut self, angle: f32) -> Self {
                *self.angle_mut() = angle;
                self
            }
        }
    };
}
