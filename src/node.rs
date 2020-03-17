use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use downcast_rs::*;

use crate::engine::Engine;
use crate::error::*;

#[macro_use]
pub mod boiler_plate;

pub(crate) mod list;
mod transform;

#[macro_use]
mod drawn;

mod camera;
mod polygon;
mod sprite;
mod text;

use list::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NodeState {
    Free,
    Registered,
    WaitingAdded,
    WaitingRemoved,
}

#[derive(Debug)]
pub struct BaseNode {
    pub(crate) state: NodeState,
    pub(crate) children: NodeList,
    pub(crate) owner: Option<Weak<RefCell<dyn Node>>>,
}

impl Default for BaseNode {
    fn default() -> Self {
        BaseNode {
            state: NodeState::Free,
            children: NodeList::new(),
            owner: None,
        }
    }
}

impl BaseNode {
    pub fn new() -> Rc<RefCell<BaseNode>> {
        Rc::new(RefCell::new(Self::default()))
    }
}

impl HasBaseNode for BaseNode {
    fn node_base(&self) -> &BaseNode {
        self
    }

    fn node_base_mut(&mut self) -> &mut BaseNode {
        self
    }
}

impl Node for BaseNode {}

pub trait HasBaseNode: std::fmt::Debug {
    fn node_base(&self) -> &BaseNode;
    fn node_base_mut(&mut self) -> &mut BaseNode;

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
            if self.node_base() as *const BaseNode != owner.borrow().node_base() as *const BaseNode
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
pub trait Node: HasBaseNode + Downcast {
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

pub use camera::*;
pub use drawn::*;
pub use polygon::Polygon;
pub use sprite::Sprite;
pub use text::Text;
pub use transform::*;
