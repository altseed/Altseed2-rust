use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::auto_generated_core_binding::{AsRendered, Font, RenderedText, Renderer};

/// 文字列を描画するためのAltseedのクラスを表します。
#[derive(Debug)]
pub struct Text {
    instance: RenderedText,
}

impl_material!(Text);

impl super::DrawnInternal for Text {
    fn on_drawn(&mut self, renderer: &mut Renderer) {
        renderer.draw_text(&mut self.instance);
    }

    fn rendered_mut(&mut self) -> &mut dyn AsRendered {
        &mut self.instance
    }
}

impl Text {
    /// 新しい`Text`を作成します。
    pub fn new() -> Self {
        Text {
            instance: RenderedText::create().unwrap(),
        }
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

    /// テキストを設定します。
    pub fn with_text(mut self, text: &str) -> Self {
        self.set_text(text);
        self
    }

    /// フォントを取得します。
    pub fn get_font(&mut self) -> Option<Arc<Mutex<Font>>> {
        self.instance.get_font()
    }

    /// フォントを設定します。
    pub fn set_font(&mut self, font: Arc<Mutex<Font>>) -> &mut Self {
        self.instance.set_font(font);
        self
    }

    /// フォントを設定します。
    pub fn with_font(mut self, font: Arc<Mutex<Font>>) -> Self {
        self.set_font(font);
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

    /// 文字の太さを設定します。(0 ~ 255)
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.set_weight(weight);
        self
    }
}
