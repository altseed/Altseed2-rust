use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use downcast_rs::*;

use crate::engine::Engine;
use crate::error::*;

#[macro_use]
pub mod boiler_plate;

mod list;
mod transform;

#[macro_use]
mod drawn;

mod camera;
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

pub use drawn::*;
pub use sprite::Sprite;
pub use text::Text;
pub use transform::*;
