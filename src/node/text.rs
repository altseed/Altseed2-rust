use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::auto_generated_core_binding::{Font, RenderedText, Renderer};
use crate::prelude::{Drawn, Vector2};

define_drawn! {
    pub struct Text {
        instance: RenderedText,
    }
}

impl super::DrawnInternal for Text {
    fn on_drawn(&mut self, renderer: &mut Renderer) {
        renderer.draw_text(&mut self.instance);
    }

    fn update_transform(
        &mut self,
        ancestors: Option<&crate::math::Matrix44<f32>>,
    ) -> Option<crate::math::Matrix44<f32>> {
        self.update_transform(ancestors)
    }
}

impl Text {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Text {
            instance: RenderedText::create().unwrap(),
            trans: super::Transform::default(),
            z_order: 0,
            is_drawn: true,
        }))
    }

    /// テキストを取得します。
    pub fn get_text(&mut self) -> String {
        self.instance.get_text()
    }

    /// テキストを設定します。
    pub fn set_text(&mut self, text: &str) -> &mut Self {
        self.instance.set_text(text.to_owned());
        self
    }

    /// フォントを取得します。
    pub fn get_font(&mut self) -> Option<Arc<Mutex<Font>>> {
        self.instance.get_font()
    }

    /// フォントを設定します。
    pub fn set_font(&mut self, font: &Arc<Mutex<Font>>) -> &mut Self {
        self.instance.set_font(font.clone());
        self
    }

    /// 文字の太さを取得します。(0 ~ 255)
    pub fn get_weight(&mut self) -> f32 {
        self.instance.get_weight()
    }

    /// 文字の太さを設定します。(0 ~ 255)
    pub fn set_weight(&mut self, weight: f32) -> &mut Self {
        self.instance.set_weight(weight);
        self
    }
}
