use std::{cell::RefCell, rc::Rc};

use crate::node::*;
use crate::prelude::Rect;

use crate as altseed2;
use crate::auto_generated_core_binding::{Graphics, RenderedSprite, Renderer, Texture2D};
use crate::create_node;

define_drawn_node! {
    pub struct SpriteNode: RenderedSprite { }
}

impl Node for SpriteNode {}

impl SpriteNode {
    pub(crate) fn on_drawn(&mut self, _: &mut Graphics, renderer: &mut Renderer) {
        if self.trans.is_updated() {
            self.trans.update();
            self.instance.set_transform(self.trans.get());
        }
        renderer.draw_sprite(&mut self.instance);
    }
}

impl SpriteNode {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(create_node!(SpriteNode {
            instance: RenderedSprite::create().unwrap(),
            trans: Transform::default(),
            z_order: 0,
        })))
    }

    /// テクスチャを取得します。
    pub fn get_texture(&mut self) -> Option<Rc<RefCell<Texture2D>>> {
        self.instance.get_texture()
    }

    /// テクスチャを設定します。
    pub fn set_texture(&mut self, texture: &Rc<RefCell<Texture2D>>) -> &mut Self {
        let size = texture.borrow_mut().get_size();
        self.instance
            .set_texture(texture.clone())
            .set_src(Rect::new(0.0, 0.0, size.x as f32, size.y as f32));
        self
    }

    pub fn get_src(&mut self) -> Rect<f32> {
        self.instance.get_src()
    }

    /// 描画範囲を設定します
    pub fn set_src(&mut self, src: Rect<f32>) -> &mut Self {
        self.instance.set_src(src);
        self
    }
}
