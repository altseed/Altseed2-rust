use std::{cell::RefCell, rc::Rc};

use crate::auto_generated_core_binding::{AsTexture2D, RenderedSprite, Renderer};
use crate::prelude::{Drawn, Rect, Vector2};

define_drawn! {
    pub struct Sprite {
        instance: RenderedSprite,
    }
}

impl super::DrawnInternal for Sprite {
    fn on_drawn(&mut self, renderer: &mut Renderer) {
        renderer.draw_sprite(&mut self.instance);
    }

    fn update_transform(
        &mut self,
        ancestors: Option<&crate::math::Matrix44<f32>>,
    ) -> Option<&crate::math::Matrix44<f32>> {
        self.update_transform(ancestors)
    }
}

impl Sprite {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Sprite {
            instance: RenderedSprite::create().unwrap(),
            trans: super::Transform::default(),
            z_order: 0,
            is_drawn: true,
        }))
    }

    /// テクスチャを取得します。
    pub fn get_texture(&mut self) -> Option<Rc<RefCell<dyn AsTexture2D>>> {
        self.instance.get_texture()
    }

    /// テクスチャを設定します。
    pub fn set_texture<T: AsTexture2D + 'static>(&mut self, texture: &Rc<RefCell<T>>) -> &mut Self {
        let size = texture.borrow_mut().get_size();
        self.instance.set_texture(&texture).set_src(Rect::new(
            0.0,
            0.0,
            size.x as f32,
            size.y as f32,
        ));
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
