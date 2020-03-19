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
pub mod transform;

#[macro_use]
pub mod drawn;

pub mod camera;
pub mod polygon;
pub mod root;
pub mod sprite;
pub mod text;

use list::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NodeState {
    Free,
    Registered,
    WaitingAdded,
    WaitingRemoved,
    AncestorRemoved,
}

#[derive(Debug)]
pub struct BaseNode {
    pub(crate) state: NodeState,
    pub(crate) children: NodeVec,
    pub(crate) owner: Option<Weak<RefCell<dyn Node>>>,
}

impl Default for BaseNode {
    fn default() -> Self {
        BaseNode {
            state: NodeState::Free,
            children: NodeVec::new(),
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

use std::collections::VecDeque;

pub trait HasBaseNode: std::fmt::Debug {
    fn node_base(&self) -> &BaseNode;
    fn node_base_mut(&mut self) -> &mut BaseNode;

    /// 現在のノードの状態を取得します。
    fn state(&self) -> NodeState {
        self.node_base().state
    }

    /// 親ノードを取得します。
    fn owner(&self) -> Option<Rc<RefCell<dyn Node>>> {
        match &self.node_base().owner {
            Some(x) => x.upgrade(),
            _ => None,
        }
    }

    /// 子ノードの一覧を取得します。
    fn children(&self) -> &VecDeque<Rc<RefCell<dyn Node>>> {
        self.node_base().children.items()
    }

    /// 子ノードを追加するフラグを立てます。EngineのUpdate時に更新されます。
    fn add_child(&mut self, child: Rc<RefCell<dyn Node>>) -> AltseedResult<()> {
        self.node_base_mut().children.add(child)
    }

    /// 子ノードを削除するフラグを立てます。EngineのUpdate時に更新されます。
    fn remove_child(&mut self, child: &mut dyn Node) -> AltseedResult<()> {
        if let Some(owner) = child.owner() {
            if self.node_base_mut() as *const BaseNode
                != owner.borrow().node_base() as *const BaseNode
            {
                return Err(AltseedError::RemovedInvalidNode(
                    std::any::type_name_of_val(self).to_owned(),
                    std::any::type_name_of_val(&child).to_owned(),
                    std::any::type_name_of_val(&owner).to_owned(),
                ));
            }
        }

        match child.node_base().state {
            NodeState::Registered | NodeState::AncestorRemoved => {
                child.node_base_mut().state = NodeState::WaitingRemoved;

                let mut descendants = VecDeque::new();

                for gc in child.children().iter() {
                    if gc.borrow().node_base().state == NodeState::Registered {
                        descendants.push_back(gc);
                    }
                }

                while let Some(c) = descendants.pop_front() {
                    let mut n = c.borrow_mut();
                    let mut base = n.node_base_mut();
                    base.state = NodeState::AncestorRemoved;

                    for gc in n.children().iter() {
                        if gc.borrow().node_base().state == NodeState::Registered {
                            descendants.push_back(c);
                        }
                    }
                }

                Ok(())
            }
            state => Err(AltseedError::InvalidNodeState(
                std::any::type_name_of_val(&child).to_owned(),
                state,
                NodeState::Registered,
            )),
        }
    }

    /// 全ての子ノードを削除するフラグを立てます。実際の更新はフレームの終わりに実行されます。
    fn clear_children(&mut self) {
        self.node_base().children.clear()
    }
}

pub(crate) fn update_node_recursive(
    node: &Rc<RefCell<dyn Node>>,
    engine: &mut Engine,
    ancestors: Option<&crate::math::Matrix44<f32>>,
) -> AltseedResult<()> {
    // 伝播させるTransformの用意
    let t = node
        .borrow_mut()
        .downcast_mut::<DrawnNode>()
        .map(|d| d.relative_transform(ancestors));

    let ancestors = match &t {
        None => ancestors,
        Some(m) => m.as_ref(),
    };

    let mut items = VecDeque::new();
    let mut tmp = VecDeque::new();
    {
        let mut x = node.borrow_mut();
        x.node_base_mut()
            .children
            .add_waiting_nodes(Rc::downgrade(&node));
        std::mem::swap(&mut items, &mut x.node_base_mut().children.items_mut());
    }

    // 子ノードの`on_hoge`呼び出し時に親ノードがborrowされてると都合が悪いのでこうなった
    while let Some(item) = items.pop_front() {
        let s = item.borrow().state().clone();
        match s {
            NodeState::WaitingAdded => {
                {
                    let mut x = item.borrow_mut();
                    // 追加後
                    x.on_added(engine)?;
                    x.node_base_mut().state = NodeState::Registered;
                }
                // 再帰
                update_node_recursive(&item, engine, ancestors)?;

                tmp.push_back(item);
            }
            NodeState::Registered => {
                // 更新前
                item.borrow_mut().on_updating(engine)?;
                // 再帰
                update_node_recursive(&item, engine, ancestors)?;
                // 更新後
                item.borrow_mut().on_updated(engine)?;

                tmp.push_back(item);
            }
            NodeState::AncestorRemoved => {
                item.borrow_mut().node_base_mut().state = NodeState::Registered;
                // 更新前
                item.borrow_mut().on_updating(engine)?;
                // 再帰
                update_node_recursive(&item, engine, ancestors)?;
                // 更新後
                item.borrow_mut().on_updated(engine)?;
                tmp.push_back(item);
            }
            NodeState::WaitingRemoved => {
                {
                    let mut x = item.borrow_mut();
                    // 削除後
                    x.on_removed(engine)?;
                    x.node_base_mut().state = NodeState::Free;
                }

                // 再帰
                update_node_recursive(&item, engine, ancestors)?;
            }
            _ => (),
        }
    }

    std::mem::swap(
        &mut tmp,
        &mut node.borrow_mut().node_base_mut().children.items_mut(),
    );

    Ok(())
}

#[allow(unused_variables)]
pub trait Node: HasBaseNode + Downcast {
    fn on_added(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        Ok(())
    }

    fn on_updating(&mut self, engine: &mut Engine) -> AltseedResult<()> {
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
pub use root::RootNode;
pub use sprite::Sprite;
pub use text::Text;
pub use transform::*;
