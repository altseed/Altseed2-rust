use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::error::*;
use crate::node::{Node, NodeState};
use std::collections::VecDeque;

#[derive(Debug)]
pub(crate) struct NodeVec {
    items: VecDeque<Rc<RefCell<dyn Node>>>,
    add_queue: VecDeque<Rc<RefCell<dyn Node>>>,
}

impl NodeVec {
    pub(crate) fn new() -> Self {
        NodeVec {
            items: VecDeque::new(),
            add_queue: VecDeque::new(),
        }
    }

    pub(crate) fn items(&self) -> &VecDeque<Rc<RefCell<dyn Node>>> {
        &self.items
    }

    pub(crate) fn items_mut(&mut self) -> &mut VecDeque<Rc<RefCell<dyn Node>>> {
        &mut self.items
    }

    pub(crate) fn add(&mut self, item: Rc<RefCell<dyn Node>>) -> AltseedResult<()> {
        let state = item.borrow().node_base().state;
        match state {
            NodeState::Free => {
                self.add_queue.push_back(item.clone());
                item.borrow_mut().node_base_mut().state = NodeState::WaitingAdded;
                Ok(())
            }
            state => Err(AltseedError::InvalidNodeState(
                "On adding child".to_owned(),
                std::any::type_name_of_val(&item.borrow()).to_owned(),
                state,
            )),
        }
    }

    pub(crate) fn add_waiting_nodes(&mut self, owner: Weak<RefCell<dyn Node>>) {
        for item in self.add_queue.iter() {
            self.items.push_back(item.clone());
            item.borrow_mut().node_base_mut().owner = Some(owner.clone());
        }
        self.add_queue.clear();
    }

    pub(crate) fn clear(&self) {
        for item in self.add_queue.iter() {
            item.borrow_mut().node_base_mut().state = NodeState::Free;
        }

        for item in self.items.iter() {
            item.borrow_mut().node_base_mut().state = NodeState::WaitingRemoved;
        }
    }
}

use std::{cmp::Ord, marker::PhantomData, slice::Iter};

pub(crate) trait SortedItem<T: Ord> {
    fn sorted_key(&self) -> T;
}

#[derive(Debug)]
pub(crate) struct SortVec<K: Ord, T: Node + SortedItem<K>> {
    vec: Vec<Weak<RefCell<T>>>,
    updated: bool,
    phantom: PhantomData<K>,
}

impl<K: Ord, T: Node + SortedItem<K>> SortVec<K, T> {
    pub fn new() -> Self {
        SortVec {
            vec: Vec::new(),
            updated: false,
            phantom: PhantomData,
        }
    }

    pub fn iter(&mut self) -> Iter<'_, Weak<RefCell<T>>> {
        self.vec.iter()
    }

    pub fn add(&mut self, item: Weak<RefCell<T>>) {
        self.vec.push(item);
        self.updated = true;
    }

    pub fn update(&mut self) {
        // 生存していないNodeは取り除く
        self.vec.retain(|x| match x.upgrade() {
            None => false,
            Some(x) => {
                let state = x.borrow().node_base().state;
                state == NodeState::Registered || state == NodeState::AncestorRemoved
            },
        });

        // 更新があったらソート
        if self.updated {
            self.vec
                .sort_by_key(|x| x.upgrade().unwrap().borrow().sorted_key());
            self.updated = false;
        }
    }
}
