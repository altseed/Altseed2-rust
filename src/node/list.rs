use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::engine::Engine;
use crate::error::*;
use crate::node::{Node, NodeState};

#[derive(Debug)]
pub(crate) struct NodeList {
    pub(crate) items: RefCell<Vec<Rc<RefCell<dyn Node>>>>,
    add_queue: RefCell<Vec<Rc<RefCell<dyn Node>>>>,
    remove_queue: RefCell<Vec<Rc<RefCell<dyn Node>>>>,
}

impl NodeList {
    pub(crate) fn new() -> Self {
        NodeList {
            items: RefCell::new(Vec::new()),
            add_queue: RefCell::new(Vec::new()),
            remove_queue: RefCell::new(Vec::new()),
        }
    }

    pub(crate) fn add(&self, item: Rc<RefCell<dyn Node>>) -> AltseedResult<()> {
        let state = item.borrow().node_base().state;
        match state {
            NodeState::Free => {
                self.add_queue.borrow_mut().push(item.clone());
                item.borrow_mut().node_base_mut().state = NodeState::WaitingAdded;
                Ok(())
            }
            state => Err(AltseedError::InvalidNodeState(
                std::any::type_name_of_val(&item.borrow()).to_owned(),
                state,
            )),
        }
    }

    pub(crate) fn remove(&self, item: Rc<RefCell<dyn Node>>) -> AltseedResult<()> {
        match item.borrow().node_base().state {
            NodeState::Registered => {
                self.remove_queue.borrow_mut().push(item.clone());
                item.borrow_mut().node_base_mut().state = NodeState::WaitingRemoved;
                Ok(())
            }
            state => Err(AltseedError::InvalidNodeState(
                std::any::type_name_of_val(&item.borrow()).to_owned(),
                state,
            )),
        }
    }

    pub(crate) fn update(
        &self,
        owner: Weak<RefCell<dyn Node>>,
        engine: &mut Engine,
    ) -> AltseedResult<()> {
        let mut items = self.items.borrow_mut();
        items.retain(|x| x.borrow().node_base().state == NodeState::Registered);

        for item in self.add_queue.borrow().iter() {
            {
                items.push(item.clone());

                let mut item = item.borrow_mut();
                let mut base = item.node_base_mut();
                base.state = NodeState::Registered;
                base.owner = Some(owner.clone());
                item.on_added(engine)?;
            }

            if item.borrow().is_drawn_node() {
                engine.drawn_nodes.borrow_mut().push(Rc::downgrade(&item));
                engine.sort_drawn_nodes = true;
            }
        }

        for item in self.remove_queue.borrow().iter() {
            let mut item = item.borrow_mut();
            item.on_removed(engine)?;
            let mut base = item.node_base_mut();
            base.state = NodeState::Free;
            base.owner = None;
        }

        self.remove_queue.borrow_mut().clear();
        self.add_queue.borrow_mut().clear();

        Ok(())
    }

    pub(crate) fn clear(&mut self) {
        self.items.borrow_mut().clear();
        self.remove_queue.borrow_mut().clear();
        self.add_queue.borrow_mut().clear();
    }

    // pub(crate) fn add_immediately(&mut self, node: Rc<RefCell<dyn Node>>) -> Result<(), String> {
    //     match node.borrow().get_state() {
    //         NodeState::Free => {
    //             self.items.borrow_mut().push(node.clone());
    //             Ok(())
    //         },
    //         state => {
    //             Err(format!("Invalid NodeState: {:?}", state))
    //         }
    //     }
    // }
}
