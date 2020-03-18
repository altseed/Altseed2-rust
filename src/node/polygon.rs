use std::{cell::RefCell, rc::Rc};

use crate::auto_generated_core_binding::{
    AsTexture2D, RenderedPolygon, Renderer, Vector2FArray, VertexArray,
};
use crate::prelude::{Drawn, Rect, Vector2};
use crate::structs::Vertex;

define_drawn! {
    pub struct Polygon {
        instance: RenderedPolygon,
    }
}

impl super::DrawnInternal for Polygon {
    fn on_drawn(&mut self, renderer: &mut Renderer) {
        renderer.draw_polygon(&mut self.instance);
    }

    fn update_transform(
        &mut self,
        ancestors: Option<&crate::math::Matrix44<f32>>,
    ) -> Option<&crate::math::Matrix44<f32>> {
        self.update_transform(ancestors)
    }
}

impl Polygon {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Polygon {
            instance: RenderedPolygon::create().unwrap(),
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
        self.instance.set_texture(texture).set_src(Rect::new(
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

    /// 頂点情報を取得します。
    pub fn get_vertexes(&mut self) -> Vec<Vertex> {
        let v = self.instance.get_vertexes().unwrap();
        let v = v.borrow_mut().to_vec();
        v
    }

    pub fn set_vertexes(&mut self, vertexes: &Vec<Vertex>) {
        let v = VertexArray::from_vec(vertexes);
        self.instance.set_vertexes(v);
    }

    /// 頂点情報を設定します。
    pub fn set_vertexes_with_position(&mut self, vertexes: &Vec<Vector2<f32>>) {
        let v = Vector2FArray::from_vec(vertexes);
        self.instance.set_vertexes_by_vector2f(&mut v.borrow_mut());
    }
}
