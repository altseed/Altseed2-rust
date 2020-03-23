use std::{cell::RefCell, rc::Rc};

use crate::auto_generated_core_binding::{AsTexture2D, RenderedSprite, Renderer};
use crate::math::Matrix44;
use crate::structs::Rect;

use super::{DrawnInternal, HasTransform, Transform};

/// 画像を描画するためのAltseedのクラスを表します。
#[derive(Debug)]
pub struct Sprite {
    instance: RenderedSprite,
    trans: Transform,
}

impl_drawn!(Sprite);

impl DrawnInternal for Sprite {
    fn on_drawn(&mut self, renderer: &mut Renderer) {
        renderer.draw_sprite(&mut self.instance);
    }

    fn set_transform(&mut self, transform: Matrix44<f32>) {
        self.instance.set_transform(transform);
    }
}

impl Sprite {
    /// 新しい[Sprite](struct.Sprite.html)を作成します。
    pub fn new() -> Self {
        Sprite {
            instance: RenderedSprite::create().unwrap(),
            trans: Transform::new(),
        }
    }

    /// テクスチャを取得します。
    pub fn get_tex(&mut self) -> Option<Rc<RefCell<dyn AsTexture2D>>> {
        self.instance.get_texture()
    }

    /// テクスチャを設定します。
    pub fn set_tex<T: AsTexture2D + 'static>(&mut self, texture: Rc<RefCell<T>>) -> &mut Self {
        let size = texture.borrow_mut().get_size();
        self.instance.set_texture(texture).set_src(Rect::new(
            0.0,
            0.0,
            size.x as f32,
            size.y as f32,
        ));
        self
    }

    /// テクスチャを設定します。
    pub fn with_tex<T: AsTexture2D + 'static>(mut self, texture: Rc<RefCell<T>>) -> Self {
        self.set_tex(texture);
        self
    }

    /// 描画範囲を取得します
    pub fn get_src(&mut self) -> Rect<f32> {
        self.instance.get_src()
    }

    /// 描画範囲を設定します
    pub fn set_src(&mut self, src: Rect<f32>) -> &mut Self {
        self.instance.set_src(src);
        self
    }

    /// 描画範囲を設定します。
    pub fn with_src(mut self, src: Rect<f32>) -> Self {
        self.set_src(src);
        self
    }
}
