use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use downcast_rs::*;

use crate::engine::Engine;
use crate::error::*;

#[macro_use]
pub mod boiler_plate;

mod camera;
mod list;
mod sprite;
mod text;
mod transform;

pub use sprite::SpriteNode;
pub use text::TextNode;
pub use transform::Transform;

use list::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NodeState {
    Free,
    Registered,
    WaitingAdded,
    WaitingRemoved,
}

#[derive(Debug)]
pub struct NodeBase {
    pub(crate) state: NodeState,
    pub(crate) children: NodeList,
    pub(crate) owner: Option<Weak<RefCell<dyn Node>>>,
}

impl Default for NodeBase {
    fn default() -> Self {
        NodeBase {
            state: NodeState::Free,
            children: NodeList::new(),
            owner: None,
        }
    }
}

impl NodeBase {
    pub fn new() -> Rc<RefCell<NodeBase>> {
        Rc::new(RefCell::new(Self::default()))
    }
}

impl HasNodeBase for NodeBase {
    fn node_base(&self) -> &NodeBase {
        self
    }

    fn node_base_mut(&mut self) -> &mut NodeBase {
        self
    }
}

impl Node for NodeBase {}

pub trait HasNodeBase: std::fmt::Debug {
    fn node_base(&self) -> &NodeBase;
    fn node_base_mut(&mut self) -> &mut NodeBase;

    fn state(&self) -> NodeState {
        self.node_base().state
    }

    fn owner(&self) -> Option<Rc<RefCell<dyn Node>>> {
        match &self.node_base().owner {
            Some(x) => x.upgrade(),
            _ => None,
        }
    }

    /// mutability concealed
    fn add_child(&self, child: Rc<RefCell<dyn Node>>) -> AltseedResult<()> {
        self.node_base().children.add(child)
    }

    /// mutability concealed
    fn remove_child(&self, child: Rc<RefCell<dyn Node>>) -> AltseedResult<()> {
        if let Some(owner) = child.borrow().owner() {
            if self.node_base() as *const NodeBase != owner.borrow().node_base() as *const NodeBase
            {
                return Err(AltseedError::RemovedInvalidNode(
                    std::any::type_name_of_val(self).to_owned(),
                    std::any::type_name_of_val(&child.borrow()).to_owned(),
                    std::any::type_name_of_val(&owner.borrow()).to_owned(),
                ));
            }
        }

        self.node_base().children.remove(child)
    }
}

pub(crate) fn update_node_base(
    node: &Rc<RefCell<dyn Node>>,
    engine: &mut Engine,
) -> AltseedResult<()> {
    node.borrow()
        .node_base()
        .children
        .update(Rc::downgrade(node), engine)?;

    for child in node.borrow().node_base().children.items.borrow().iter() {
        update_node_base(&child, engine)?;
        child.borrow_mut().on_updated(engine)?;
    }

    Ok(())
}

#[allow(unused_variables)]
pub trait Node: HasNodeBase + Downcast {
    fn on_added(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        Ok(())
    }
    fn on_updated(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        Ok(())
    }
    fn on_removed(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        Ok(())
    }
}

impl_downcast!(Node);

use crate::auto_generated_core_binding::{Graphics, Renderer};

impl dyn Node {
    pub(crate) fn is_drawn_node(&self) -> bool {
        self.is::<SpriteNode>() || self.is::<TextNode>()
    }

    pub(crate) unsafe fn downcast_z_order(&self) -> i32 {
        if let Some(x) = self.downcast_ref::<SpriteNode>() {
            x.get_z_order()
        } else if let Some(x) = self.downcast_ref::<TextNode>() {
            x.get_z_order()
        } else {
            panic!("Never call downcast_z_order to a non-DrawnNode");
        }
    }

    pub(crate) unsafe fn downcast_on_drawn(
        &mut self,
        graphics: &mut Graphics,
        renderer: &mut Renderer,
    ) {
        if let Some(x) = self.downcast_mut::<SpriteNode>() {
            x.on_drawn(graphics, renderer);
        } else if let Some(x) = self.downcast_mut::<TextNode>() {
            x.on_drawn(graphics, renderer);
        } else {
            panic!("Never call downcast_on_drawn to a non-DrawnNode");
        }
    }
}

pub trait DrawnNode: Node {
    fn transform(&self) -> &crate::prelude::Transform;
    fn transform_mut(&mut self) -> &mut crate::prelude::Transform;
    /// 描画順を取得します。
    fn get_z_order(&self) -> i32;
    /// 描画順を設定します。
    fn set_z_order(&mut self, z_order: i32) -> &mut dyn DrawnNode;
    /// マテリアルを取得します。
    fn get_material(&mut self) -> Rc<RefCell<crate::prelude::Material>>;
    /// マテリアルを設定します。
    fn set_material(&mut self, mat: Rc<RefCell<crate::prelude::Material>>) -> &mut dyn DrawnNode;
}

// pub(crate) trait HasOnDrawn {
//     fn on_drawn(&mut self, graphics: &mut Graphics, renderer: &mut Renderer);
// }
