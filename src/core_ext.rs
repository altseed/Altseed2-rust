use crate::auto_generated_core_binding::{
    ButtonState, Font, Keyboard, Keys, PolygonCollider, Texture2D, Vector2FArray,
};
use crate::math::*;

impl Keyboard {
    pub fn is_free(&mut self, key: Keys) -> bool {
        self.get_key_state(key) == ButtonState::Free
    }

    pub fn is_push(&mut self, key: Keys) -> bool {
        self.get_key_state(key) == ButtonState::Push
    }

    pub fn is_hold(&mut self, key: Keys) -> bool {
        self.get_key_state(key) == ButtonState::Hold
    }

    pub fn is_release(&mut self, key: Keys) -> bool {
        self.get_key_state(key) == ButtonState::Release
    }
}

impl Font {
    pub fn add_image_glyph(&mut self, character: char, texture: &mut Texture2D) {
        self.__add_image_glyph(character as i32, texture);
    }
}

impl PolygonCollider {
    pub fn get_verts(&mut self) -> Vec<Vector2<f32>> {
        self.__get_vertexes().unwrap().borrow_mut().to_vec()
    }

    pub fn set_verts(&mut self, verts: &Vec<Vector2<f32>>) {
        self.__set_vertexes(Vector2FArray::from_vec(verts));
    }
}
