use std::{cell::RefCell, rc::Rc};

use crate::auto_generated_core_binding::{
    AsTexture2D, RenderedPolygon, Renderer, Vector2FArray, VertexArray,
};
use crate::math::{Matrix44, Vector2};
use crate::structs::{Rect, Vertex};

use super::{DrawnInternal, HasTransform, Transform};

/// ポリゴンを描画するためのAltseedのクラスを表します。
#[derive(Debug)]
pub struct Polygon {
    instance: RenderedPolygon,
    trans: Transform,
}

impl_drawn!(Polygon);

impl DrawnInternal for Polygon {
    fn on_drawn(&mut self, renderer: &mut Renderer) {
        renderer.draw_polygon(&mut self.instance);
    }

    fn set_transform(&mut self, transform: Matrix44<f32>) {
        self.instance.set_transform(transform);
    }
}

impl Polygon {
    /// 新しい[Polygon](struct.Polygon.html)を作成します。
    pub fn new() -> Self {
        Polygon {
            instance: RenderedPolygon::create().unwrap(),
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

    /// 描画範囲を取得します
    pub fn get_src(&mut self) -> Rect<f32> {
        self.instance.get_src()
    }

    /// 描画範囲を設定します
    pub fn set_src(&mut self, src: Rect<f32>) -> &mut Self {
        self.instance.set_src(src);
        self
    }

    /// 頂点情報を取得します。
    pub fn get_vertexes(&mut self) -> Vec<Vertex> {
        let v = self.instance.get_vertexes().unwrap();
        let v = v.borrow_mut().to_vec();
        v
    }

    /// 頂点情報を設定します。
    pub fn set_verts(&mut self, vertexes: &Vec<Vertex>) -> &mut Self {
        let v = VertexArray::from_vec(vertexes);
        self.instance.set_vertexes(v);
        self
    }

    /// 頂点情報を設定します。
    pub fn set_verts_positions(&mut self, vertexes: &Vec<Vector2<f32>>) -> &mut Self {
        let v = Vector2FArray::from_vec(vertexes);
        self.instance.set_vertexes_by_vector2f(&mut v.borrow_mut());
        self
    }
}
