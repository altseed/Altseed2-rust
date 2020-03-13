use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate as altseed2;
use crate::auto_generated_core_binding::{Font, Graphics, RenderedText, Renderer};
use crate::node::*;
use crate::{create_node, define_node};

define_drawn_node! {
    pub struct TextNode: RenderedText { }
}

impl Node for TextNode {}

impl TextNode {
    pub(crate) fn on_drawn(&mut self, _: &mut Graphics, renderer: &mut Renderer) {
        if self.trans.is_updated() {
            self.trans.update();
            self.instance.set_transform(self.trans.get());
        }
        renderer.draw_text(&mut self.instance);
    }
}

impl TextNode {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(create_node!(TextNode {
            instance: RenderedText::create().unwrap(),
            trans: Transform::default(),
            z_order: 0,
        })))
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
