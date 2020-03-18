use std::{cell::RefCell, rc::Rc};

use crate::auto_generated_core_binding::{RenderTexture, RenderedCamera, Renderer};
use crate::math::Vector2;
use crate::node::*;

use crate as altseed2;
use crate::{create_node, define_node};

define_node! {
    pub struct CameraNode {
        instance: RenderedCamera,
        weak: Option<Weak<RefCell<Self>>>,
        group: u32,
    }
}

impl Node for CameraNode {
    fn on_added(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        engine.add_camera_node(self.weak.clone().unwrap());
        Ok(())
    }
}

impl CameraNode {
    pub fn new() -> Rc<RefCell<Self>> {
        let rc = Rc::new(RefCell::new(create_node!(CameraNode {
            instance: RenderedCamera::create().unwrap(),
            weak: None,
            group: 0u32,
        })));

        rc.borrow_mut().weak = Some(Rc::downgrade(&rc));

        rc
    }

    pub fn get_group(&self) -> u32 {
        self.group
    }

    pub fn set_group(&mut self) -> &mut u32 {
        &mut self.group
    }

    pub fn get_target_texture(&mut self) -> Option<Rc<RefCell<RenderTexture>>> {
        self.instance.get_target_texture()
    }

    pub fn set_target_texture(&mut self, value: Rc<RefCell<RenderTexture>>) -> &mut Self {
        self.instance.set_target_texture(value);
        self
    }

    pub(crate) fn instance(&mut self) -> &mut RenderedCamera {
        &mut self.instance
    }
}

impl list::SortedItem<u32> for CameraNode {
    fn sorted_key(&self) -> u32 {
        self.get_group()
    }
}
