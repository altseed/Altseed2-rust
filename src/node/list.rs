use std::{
    cell::{Ref, RefCell},
    rc::{Rc, Weak},
};

use crate::engine::Engine;
use crate::error::*;
use crate::node::{Node, NodeState};
use std::collections::VecDeque;

#[derive(Debug)]
pub(crate) struct NodeVec {
    items: RefCell<VecDeque<Rc<RefCell<dyn Node>>>>,
    add_queue: RefCell<VecDeque<Rc<RefCell<dyn Node>>>>,
}

impl NodeVec {
    pub(crate) fn new() -> Self {
        NodeVec {
            items: RefCell::new(VecDeque::new()),
            add_queue: RefCell::new(VecDeque::new()),
        }
    }

    pub(crate) fn items(&self) -> Ref<'_, VecDeque<Rc<RefCell<dyn Node>>>> {
        self.items.borrow()
    }

    pub(crate) fn add(&self, item: Rc<RefCell<dyn Node>>) -> AltseedResult<()> {
        let state = item.borrow().node_base().state;
        match state {
            NodeState::Free => {
                self.add_queue.borrow_mut().push_back(item.clone());
                item.borrow_mut().node_base_mut().state = NodeState::WaitingAdded;
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
        {
            let mut added = self.add_queue.borrow_mut();
            for item in added.iter() {
                items.push_back(item.clone());
                item.borrow_mut().node_base_mut().owner = Some(owner.clone());
            }
            added.clear();
        }

        let mut tmp = VecDeque::new();

        while let Some(item) = items.pop_front() {
            let mut x = item.borrow_mut();
            match x.state() {
                NodeState::WaitingAdded => {
                    tmp.push_back(item.clone());
                    x.on_added(engine)?;
                    x.node_base_mut().state = NodeState::Registered;
                }
                NodeState::Registered => {
                    tmp.push_back(item.clone());
                }
                NodeState::WaitingRemoved => {
                    x.on_removed(engine)?;
                    x.node_base_mut().state = NodeState::Free;
                }
                _ => (),
            }
        }

        std::mem::swap(&mut tmp, &mut items);

        Ok(())
    }

    pub(crate) fn clear(&mut self) {
        self.add_queue.borrow_mut().clear();
        self.items.borrow_mut().clear();
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
            Some(x) => x.borrow().node_base().state == NodeState::Registered,
        });

        // 更新があったらソート
        if self.updated {
            self.vec
                .sort_by_key(|x| x.upgrade().unwrap().borrow().sorted_key());
            self.updated = false;
        }
    }
}
