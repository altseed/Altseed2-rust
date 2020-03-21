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
            _ => Err(AltseedError::InvalidNodeState(
                "On adding child".to_owned(),
                state,
                format!("{:?}", &item.borrow()),
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

use std::{cmp::Ord, slice::Iter};

pub(crate) trait SortedItem<T: Ord + Copy> {
    fn sorted_key(&self) -> T;
    fn is_key_updated(&self) -> bool;
}

#[derive(Debug)]
pub(crate) struct SortVec<K: Ord + Copy, T: Node + SortedItem<K>> {
    vec: Vec<Weak<RefCell<T>>>,
    do_sort: bool,
    last_key: Option<K>,
}

impl<K: Ord + Copy, T: Node + SortedItem<K>> SortVec<K, T> {
    pub fn new() -> Self {
        SortVec {
            vec: Vec::new(),
            do_sort: false,
            last_key: None,
        }
    }

    pub fn clear(&mut self) {
        self.vec.clear();
        self.do_sort = false;
        self.last_key = None;
    }

    pub fn iter(&self) -> Iter<'_, Weak<RefCell<T>>> {
        self.vec.iter()
    }

    pub fn add(&mut self, item: Weak<RefCell<T>>, key: K) {
        self.vec.push(item);

        // 順序が乱れたらソートするフラグを立てる
        match self.last_key {
            Some(x) if x > key => {
                self.do_sort = true;
            }
            _ => (),
        }

        self.last_key = Some(key);
    }

    // pub fn set_do_sort(&mut self) {
    //     self.do_sort = true;
    // }

    pub fn update_with<F: Fn(&T) -> bool>(&mut self, f: F) {
        // Rootに接続されていないNodeは取り除く
        self.vec.retain(|x| match x.upgrade() {
            None => false,
            Some(x) => f(&x.borrow()),
        });

        // 更新があったらソート
        if self.do_sort {
            self.vec
                .sort_by_key(|x| x.upgrade().unwrap().borrow().sorted_key());
            self.do_sort = false;
        }
    }

    pub fn update(&mut self) {
        let mut tmp = Vec::new();
        std::mem::swap(&mut tmp, &mut self.vec);

        let mut key_updated = false;

        // Rootに接続されていないNodeは取り除く
        for x in tmp.into_iter() {
            match x.upgrade() {
                None => (),
                Some(rc) => {
                    if !key_updated && rc.borrow().is_key_updated() {
                        key_updated = true;
                    }

                    if rc.borrow().node_base().state == NodeState::Registered {
                        self.vec.push(x);
                    }
                }
            }
        }

        // 更新があったらソート
        if self.do_sort || key_updated {
            self.vec
                .sort_by_key(|x| x.upgrade().unwrap().borrow().sorted_key());
            self.do_sort = false;
        }
    }
}
