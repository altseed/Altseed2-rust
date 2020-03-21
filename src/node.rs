//! オブジェクトシステムを実装するモジュールです。

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use downcast_rs::*;

use crate::engine::Engine;
use crate::error::*;

#[macro_use]
#[doc(hidden)]
mod boiler_plate;

pub(crate) mod list;
pub mod transform;

#[macro_use]
pub mod drawn;
pub mod camera;
pub mod polygon;
pub mod sprite;
pub mod text;

use list::*;

/// [Node](trait.Node.html)の登録状態を表します。
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NodeState {
    /// [Node](trait.Node.html)に追加されていません。
    Free,
    /// [Node](trait.Node.html)に追加されています。
    Registered,
    /// [Node](trait.Node.html)への追加待ちです。
    WaitingAdded,
    /// [Node](trait.Node.html)からの削除待ちです。
    WaitingRemoved,
    /// [Node](trait.Node.html)に追加されてますが、[RootNode](root/struct.RootNode.html)に接続されていません。
    Disconnected,
}

/// 空の[Node](trait.Node.html)です。[Node](trait.Node.html)の基本機能を提供するために[define_node!](../macro.define_node!.html)マクロで埋め込まれます。
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
    /// 新しい[BaseNode](struct.BaseNode.html)を作成します。
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

/// [BaseNode](struct.BaseNode.html)の基本的な機能を提供するトレイトです。
/// [define_node!](../macro.define_node!.html)マクロで自動的に実装されます。
pub trait HasBaseNode: std::fmt::Debug {
    fn node_base(&self) -> &BaseNode;
    fn node_base_mut(&mut self) -> &mut BaseNode;

    /// 現在の[Node](trait.Node.html)の状態を取得します。
    fn state(&self) -> NodeState {
        self.node_base().state
    }

    /// 親[Node](trait.Node.html)を取得します。
    fn owner(&self) -> Option<Rc<RefCell<dyn Node>>> {
        match &self.node_base().owner {
            Some(x) => x.upgrade(),
            _ => None,
        }
    }

    /// 子[Node](trait.Node.html)の一覧を取得します。
    fn children(&self) -> &VecDeque<Rc<RefCell<dyn Node>>> {
        self.node_base().children.items()
    }

    /// 子[Node](trait.Node.html)を追加するフラグを立てます。EngineのUpdate時に更新されます。
    fn add_child(&mut self, child: Rc<RefCell<dyn Node>>) -> AltseedResult<()> {
        self.node_base_mut().children.add(child.clone())?;
        Ok(())
    }

    /// 自身を親[Node](trait.Node.html)から削除するフラグを立てます。[Engine](../engine/struct.Engine.html)のUpdate時に更新されます。
    fn remove(&mut self) -> AltseedResult<()> {
        match self.node_base().state {
            NodeState::Registered | NodeState::Disconnected => {
                self.node_base_mut().state = NodeState::WaitingRemoved;
                Ok(())
            }
            state => Err(AltseedError::InvalidNodeState(
                "On removeing node".to_owned(),
                state,
                format!("{:?}", self),
            )),
        }
    }

    /// 全ての子[Node](trait.Node.html)を削除するフラグを立てます。[Engine](../engine/struct.Engine.html)のUpdate時に実行されます。
    fn clear_children(&mut self) {
        self.node_base().children.clear()
    }
}

// あるノードを更新中に他のノードの参照をとらないようにするため、Rcで渡す
pub(crate) fn update_node_recursive(
    node: &Rc<RefCell<dyn Node>>,
    engine: &mut Engine,
    ancestors: Option<&crate::math::Matrix44<f32>>,
    is_connected_root: bool,
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

    // 追加の反映
    node.borrow_mut()
        .node_base_mut()
        .children
        .add_waiting_nodes(Rc::downgrade(&node));

    // 一時格納キュー
    let mut items = VecDeque::new();
    // 可変参照の関係でswap
    std::mem::swap(
        &mut items,
        &mut node.borrow_mut().node_base_mut().children.items_mut(),
    );

    for _ in 0..items.len() {
        let item = items.pop_front().unwrap();
        let s = item.borrow().node_base().state.clone();

        match s {
            NodeState::WaitingAdded => {
                {
                    let mut x = item.borrow_mut();
                    x.node_base_mut().state = NodeState::Registered;
                    // 追加後
                    x.on_added(engine)?;

                    if is_connected_root {
                        x.on_connected_root(engine)?;
                    } else {
                        x.node_base_mut().state = NodeState::Disconnected;
                        x.on_disconnected_root(engine)?;
                    }
                }
                // 再帰
                update_node_recursive(&item, engine, ancestors, is_connected_root)?;

                items.push_back(item);
            }
            NodeState::Registered => {
                if is_connected_root {
                    // 更新
                    item.borrow_mut().on_updated(engine)?;
                } else {
                    let mut x = item.borrow_mut();
                    x.node_base_mut().state = NodeState::Disconnected;
                    x.on_disconnected_root(engine)?;
                }

                // 再帰
                update_node_recursive(&item, engine, ancestors, is_connected_root)?;
                items.push_back(item);
            }
            NodeState::Disconnected => {
                // rootに再接続されたら
                if is_connected_root {
                    let mut x = item.borrow_mut();
                    // stateを戻す
                    x.node_base_mut().state = NodeState::Registered;
                    x.on_connected_root(engine)?;
                }

                // 再帰
                update_node_recursive(&item, engine, ancestors, is_connected_root)?;
                items.push_back(item);
            }
            NodeState::WaitingRemoved => {
                {
                    let mut x = item.borrow_mut();
                    x.node_base_mut().state = NodeState::Free;
                    // 削除後
                    x.on_removed(engine)?;
                    x.on_disconnected_root(engine)?;
                }

                // 再帰
                update_node_recursive(&item, engine, ancestors, false)?;
            }
            _ => (),
        }
    }

    // swapで一時格納キューから戻す
    std::mem::swap(
        &mut items,
        &mut node.borrow_mut().node_base_mut().children.items_mut(),
    );

    Ok(())
}

#[allow(unused_variables)]
pub trait Node: HasBaseNode + Downcast {
    /// 親の[Node](trait.Node.html)に追加された時に実行されます。
    fn on_added(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        Ok(())
    }

    /// 毎フレーム実行されます。
    fn on_updated(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        Ok(())
    }

    /// 親[のNode](trait.Node.html)から削除された時に実行されます。
    fn on_removed(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        Ok(())
    }

    /// [RootNode](root/struct.RootNode.html)に接続された時に実行されます。
    fn on_connected_root(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        Ok(())
    }

    /// [RootNode](root/struct.RootNode.html)への接続が切れた時に実行されます。
    fn on_disconnected_root(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        Ok(())
    }
}

impl_downcast!(Node);

pub use camera::CameraNode;
pub(crate) use drawn::DrawnInternal;
pub use drawn::{Drawn, DrawnKind, DrawnNode};
pub use polygon::Polygon;
pub use sprite::Sprite;
pub use text::Text;
pub use transform::Transform;

use crate as altseed2;
use std::marker::PhantomData;

define_node! {
    /// [Engine](../engine/struct.Engine.html)のルートに登録されている[Node](trait.Node.html)を表します。
    pub struct RootNode {
        phantom: PhantomData<()>,
    }
}

impl Node for RootNode {}

impl RootNode {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(create_node! {
            RootNode { phantom: PhantomData }
        }))
    }
}
