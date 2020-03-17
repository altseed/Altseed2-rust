use std::{cell::RefCell, rc::Rc};

use crate::auto_generated_core_binding::{Graphics, Material, Renderer};
use crate::math::Vector2;
use crate::node::*;

use crate as altseed2;
use crate::define_node;

pub trait Drawn {
    fn transform(&self) -> &Transform;

    fn transform_mut(&mut self) -> &mut Transform;

    /// 描画順
    fn z_order(&self) -> i32;

    /// 描画順への可変参照
    fn z_order_mut(&mut self) -> &mut i32;

    /// マテリアルを取得します。
    fn get_material(&mut self) -> Rc<RefCell<Material>>;

    /// マテリアルを設定します。
    fn set_material(&mut self, mat: Rc<RefCell<Material>>);
}

#[derive(Debug, Clone)]
pub enum DrawnKind {
    Sprite(Rc<RefCell<Sprite>>),
    Text(Rc<RefCell<Text>>),
    Polygon(Rc<RefCell<Polygon>>),
}

impl DrawnKind {
    pub fn get_drawn(&self) -> Rc<RefCell<dyn Drawn>> {
        match self.clone() {
            DrawnKind::Sprite(x) => x,
            DrawnKind::Text(x) => x,
            DrawnKind::Polygon(x) => x,
        }
    }
}

impl From<Rc<RefCell<Sprite>>> for DrawnKind {
    fn from(item: Rc<RefCell<Sprite>>) -> Self {
        DrawnKind::Sprite(item)
    }
}

impl From<Rc<RefCell<Text>>> for DrawnKind {
    fn from(item: Rc<RefCell<Text>>) -> Self {
        DrawnKind::Text(item)
    }
}

impl From<Rc<RefCell<Polygon>>> for DrawnKind {
    fn from(item: Rc<RefCell<Polygon>>) -> Self {
        DrawnKind::Polygon(item)
    }
}

define_node! {
    pub struct DrawnNode {
        kind: DrawnKind,
    }
}

impl Node for DrawnNode {}

impl DrawnNode {
    pub fn new<T: Into<DrawnKind>>(kind: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(DrawnNode {
            node_base: BaseNode::default(),
            kind: kind.into(),
        }))
    }

    pub fn kind(&self) -> &DrawnKind {
        &self.kind
    }

    pub fn kind_mut(&mut self) -> &mut DrawnKind {
        &mut self.kind
    }

    pub(crate) fn on_drawn(&mut self, _: &mut Graphics, renderer: &mut Renderer) {
        match self.kind.clone() {
            DrawnKind::Sprite(x) => {
                let mut x = x.borrow_mut();
                x.update_transform();
                renderer.draw_sprite(x.instance());
            }
            DrawnKind::Text(x) => {
                let mut x = x.borrow_mut();
                x.update_transform();
                renderer.draw_text(x.instance());
            }
            DrawnKind::Polygon(x) => {
                let mut x = x.borrow_mut();
                x.update_transform();
                renderer.draw_polygon(x.instance());
            }
        }
    }
}

impl DrawnNode {
    pub fn get_z_order(&self) -> i32 {
        self.kind.get_drawn().borrow().z_order()
    }

    pub fn set_z_order(&mut self, z_order: i32) -> &mut Self {
        *self.kind.get_drawn().borrow_mut().z_order_mut() = z_order;
        self
    }

    pub fn get_material(&mut self) -> Rc<RefCell<Material>> {
        self.kind.get_drawn().borrow_mut().get_material()
    }

    pub fn set_material(&mut self, mat: Rc<RefCell<Material>>) -> &mut Self {
        self.kind.get_drawn().borrow_mut().set_material(mat);
        self
    }

    pub fn get_pos(&self) -> Vector2<f32> {
        self.kind.get_drawn().borrow().transform().pos()
    }

    pub fn set_pos(&mut self, pos: Vector2<f32>) -> &mut Self {
        *self.kind.get_drawn().borrow_mut().transform_mut().pos_mut() = pos;
        self
    }

    pub fn get_center(&self) -> Vector2<f32> {
        self.kind.get_drawn().borrow().transform().center()
    }

    pub fn set_center(&mut self, center: Vector2<f32>) -> &mut Self {
        *self
            .kind
            .get_drawn()
            .borrow_mut()
            .transform_mut()
            .center_mut() = center;
        self
    }

    pub fn get_scale(&self) -> Vector2<f32> {
        self.kind.get_drawn().borrow().transform().scale()
    }

    pub fn set_scale(&mut self, scale: Vector2<f32>) -> &mut Self {
        *self
            .kind
            .get_drawn()
            .borrow_mut()
            .transform_mut()
            .scale_mut() = scale;
        self
    }

    pub fn get_angle(&self) -> f32 {
        self.kind.get_drawn().borrow().transform().angle()
    }

    pub fn set_angle(&mut self, angle: f32) -> &mut Self {
        *self
            .kind
            .get_drawn()
            .borrow_mut()
            .transform_mut()
            .angle_mut() = angle;
        self
    }
}

macro_rules! define_drawn {
    ($(#[$meta_s:meta])*
    pub struct $name: ident {
        $(
            $(#[$meta_v:meta])*
            $variant: ident : $ty: ty,
        )*
    }) => {
        $(#[$meta_s])*
        #[derive(Debug)]
        pub struct $name{
            trans: crate::node::Transform,
            z_order: i32,
            $(
                $(#[$meta_v])*
                $variant: $ty,
            )*
        }

        impl $name {
            pub(crate) fn update_transform(&mut self) {
                if self.trans.is_updated() {
                    self.trans.update();
                    let t = self.trans.get();
                    self.instance.set_transform(t);
                }
            }
        }

        impl Drawn for $name {
            fn transform(&self) -> &crate::node::Transform {
                &self.trans
            }

            fn transform_mut(&mut self) -> &mut crate::node::Transform {
                &mut self.trans
            }

            fn z_order(&self) -> i32 {
                self.z_order
            }

            fn z_order_mut(&mut self) -> &mut i32 {
                &mut self.z_order
            }

            fn get_material(&mut self) -> Rc<RefCell<crate::prelude::Material>> {
                self.instance.get_material().unwrap()
            }

            fn set_material(
                &mut self,
                mat: Rc<RefCell<crate::prelude::Material>>) {
                self.instance.set_material(mat);
            }
        }

        impl $name {
            pub fn get_pos(&self) -> Vector2<f32> {
                self.transform().pos().clone()
            }

            pub fn set_pos(&mut self, pos: Vector2<f32>) -> &mut Self {
                *self.transform_mut().pos_mut() = pos;
                self
            }

            pub fn get_center(&self) -> Vector2<f32> {
                self.transform().center().clone()
            }

            pub fn set_center(&mut self, center: Vector2<f32>) -> &mut Self {
                *self.transform_mut().center_mut() = center;
                self
            }

            pub fn get_scale(&self) -> Vector2<f32> {
                self.transform().scale().clone()
            }

            pub fn set_scale(&mut self, scale: Vector2<f32>) -> &mut Self {
                *self.transform_mut().scale_mut() = scale;
                self
            }

            pub fn get_angle(&self) -> f32 {
                self.transform().angle().clone()
            }

            pub fn set_angle(&mut self, angle: f32) -> &mut Self {
                *self.transform_mut().angle_mut() = angle;
                self
            }
        }
    };
}
