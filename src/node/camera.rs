use std::{cell::RefCell, rc::Rc};

use crate::auto_generated_core_binding::{Graphics, RenderTexture, RenderedCamera, Renderer};
use crate::node::*;

use crate as altseed2;
use crate::{create_node, define_node};

define_node! {
    /// カメラ機能を保持するノードです。
    pub struct CameraNode {
        instance: RenderedCamera,
        weak: Option<Weak<RefCell<Self>>>,
        group: u32,
        /// 前フレームのカメラグループ
        last_group: Option<u32>,
        drawn_nodes: list::SortVec<i32, DrawnNode>,
    }
}

impl Node for CameraNode {
    fn on_connected_root(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        engine.add_camera_node(self.weak.clone().unwrap(), self.group);

        Ok(())
    }
}

impl CameraNode {
    /// 新しいカメラノードを作成します
    pub fn new() -> Rc<RefCell<Self>> {
        let rc = Rc::new(RefCell::new(create_node!(CameraNode {
            instance: RenderedCamera::create().unwrap(),
            weak: None,
            group: 0u32,
            // group_updated: true,
            last_group: None,
            drawn_nodes: list::SortVec::new(),
        })));

        rc.borrow_mut().weak = Some(Rc::downgrade(&rc));

        rc
    }

    pub(crate) fn on_drawn(
        &mut self,
        drawn_nodes: &SortVec<i32, DrawnNode>,
        graphics: &mut Graphics,
        renderer: &mut Renderer,
    ) {
        if self.is_key_updated() {
            self.drawn_nodes.clear();
            // EngineからDrawnNodeを引っ張ってくる
            for node in drawn_nodes.iter() {
                let rc = node.upgrade().expect("Already filtered");
                if rc.borrow().get_camera_group() & self.group == self.group {
                    self.add_drawn_node(node.clone(), rc.borrow().get_z_order())
                }
            }
        } else {
            // groupが更新されてないならfilterのみ
            let group = self.group;
            self.drawn_nodes.update_with(|x| {
                x.node_base().state == NodeState::Registered
                    && (x.get_camera_group() & group == group)
            });
        }

        self.last_group = Some(self.group);

        for node in self.drawn_nodes.iter() {
            let rc = node.upgrade().expect("Already filtered");
            let mut node_ref = rc.borrow_mut();
            node_ref.on_drawn(graphics, renderer)
        }
    }

    pub(crate) fn add_drawn_node(&mut self, node: Weak<RefCell<DrawnNode>>, z_order: i32) {
        self.drawn_nodes.add(node, z_order);
    }

    /// カメラグループ
    pub fn get_group(&self) -> u32 {
        self.group
    }

    /// カメラグループ
    pub fn set_group(&mut self, group: u32) -> &mut Self {
        self.group = group;
        self
    }

    /// 出力先のRenderTextureを取得します。
    pub fn get_target_texture(&mut self) -> Option<Rc<RefCell<RenderTexture>>> {
        self.instance.get_target_texture()
    }

    /// 出力先のRenderTextureを設定します。
    pub fn set_target_texture(&mut self, value: Rc<RefCell<RenderTexture>>) -> &mut Self {
        self.instance.set_target_texture(value);
        self
    }
}

impl list::SortedItem<u32> for CameraNode {
    fn sorted_key(&self) -> u32 {
        self.get_group()
    }

    fn is_key_updated(&self) -> bool {
        Some(self.group) != self.last_group
    }
}
