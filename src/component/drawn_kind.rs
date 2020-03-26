use crate::{
    math::*,
    structs::{Rect, Vertex},
};

use crate::auto_generated_core_binding::{
    AsTexture2D, Font, RenderedPolygon, RenderedSprite, RenderedText, Vector2FArray, VertexArray,
};

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub enum DrawnKind {
    Sprite(Sprite),
    Text(Text),
    Polygon(Polygon),
}

use super::drawn::DrawnComponent;

macro_rules! impl_has_transform {
    ($name: ident) => {
        impl $name {
            pub(crate) fn update_transform(&mut self) {
                if self.transform.update(None) {
                    self.instance.set_transform(self.transform.get().clone());
                }
            }

            /// 位置を設定します。
            pub fn with_pos(mut self, pos: Vector2<f32>) -> Self {
                *self.pos_mut() = pos;
                self
            }

            /// 中心座標を設定します。
            pub fn with_center(mut self, center: Vector2<f32>) -> Self {
                *self.center_mut() = center;
                self
            }

            /// 拡大率を設定します。
            pub fn with_scale(mut self, scale: Vector2<f32>) -> Self {
                *self.scale_mut() = scale;
                self
            }

            /// 角度を指定します。
            pub fn with_angle(mut self, angle: f32) -> Self {
                *self.angle_mut() = angle;
                self
            }

            pub fn build(self) -> DrawnComponent {
                DrawnComponent::new(DrawnKind::$name(self))
            }
        }

        impl HasTransform for $name {
            fn transform(&self) -> &Transform {
                &self.transform
            }

            fn transform_mut(&mut self) -> &mut Transform {
                &mut self.transform
            }
        }
    };
}

#[derive(Debug)]
pub struct Sprite {
    pub(crate) instance: RenderedSprite,
    transform: Transform,
}

impl_has_transform!(Sprite);

impl Sprite {
    pub fn new() -> Self {
        Sprite {
            instance: RenderedSprite::create().unwrap(),
            transform: Transform::new(),
        }
    }

    /// テクスチャを取得します。
    pub fn get_texture(&mut self) -> Option<Rc<RefCell<dyn AsTexture2D>>> {
        self.instance.get_texture()
    }

    /// テクスチャを設定します。
    pub fn set_texture<T: AsTexture2D + 'static>(&mut self, texture: Rc<RefCell<T>>) -> &mut Self {
        let size = texture.borrow_mut().get_size();
        self.instance.set_texture(texture).set_src(Rect::new(
            0.0,
            0.0,
            size.x as f32,
            size.y as f32,
        ));
        self
    }

    pub fn with_texture<T: AsTexture2D + 'static>(mut self, texture: Rc<RefCell<T>>) -> Self {
        self.set_texture(texture);
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

    /// 描画範囲を設定します
    pub fn with_src(mut self, src: Rect<f32>) -> Self {
        self.set_src(src);
        self
    }
}

#[derive(Debug)]
pub struct Text {
    pub(crate) instance: RenderedText,
    transform: Transform,
}

impl_has_transform!(Text);
impl Text {
    pub fn new() -> Self {
        Text {
            instance: RenderedText::create().unwrap(),
            transform: Transform::new(),
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

#[derive(Debug)]
pub struct Polygon {
    pub(crate) instance: RenderedPolygon,
    transform: Transform,
}

impl_has_transform!(Polygon);

impl Polygon {
    /// 新しい[Polygon](struct.Polygon.html)を作成します。
    pub fn new() -> Self {
        Polygon {
            instance: RenderedPolygon::create().unwrap(),
            transform: Transform::new(),
        }
    }

    /// テクスチャを取得します。
    pub fn get_texture(&mut self) -> Option<Rc<RefCell<dyn AsTexture2D>>> {
        self.instance.get_texture()
    }

    /// テクスチャを設定します。
    pub fn set_texture<T: AsTexture2D + 'static>(&mut self, texture: Rc<RefCell<T>>) -> &mut Self {
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
    pub fn with_texture<T: AsTexture2D + 'static>(mut self, texture: Rc<RefCell<T>>) -> Self {
        self.set_texture(texture);
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

    /// 描画範囲を設定します
    pub fn with_src(mut self, src: Rect<f32>) -> Self {
        self.set_src(src);
        self
    }

    /// 頂点情報を取得します。
    pub fn get_vertexes(&mut self) -> Vec<Vertex> {
        let v = self.instance.get_vertexes().unwrap();
        let v = v.borrow_mut().to_vec();
        v
    }

    /// 頂点情報を設定します。
    pub fn set_verts(&mut self, verts: &Vec<Vertex>) -> &mut Self {
        let v = VertexArray::from_vec(verts);
        self.instance.set_vertexes(v);
        self
    }

    /// 頂点情報を設定します。
    pub fn with_verts(mut self, verts: &Vec<Vertex>) -> Self {
        self.set_verts(verts);
        self
    }

    /// 位置を元に頂点情報を設定します。
    pub fn set_verts_pos(&mut self, vertexes: &Vec<Vector2<f32>>) -> &mut Self {
        let v = Vector2FArray::from_vec(vertexes);
        self.instance.set_vertexes_by_vector2f(&mut v.borrow_mut());
        self
    }

    /// 位置を元に頂点情報を設定します。
    pub fn with_verts_pos(mut self, verts: &Vec<Vector2<f32>>) -> Self {
        self.set_verts_pos(verts);
        self
    }
}
