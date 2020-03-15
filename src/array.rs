macro_rules! impl_array {
    ($name: ty, $ty: ty) => {
        impl_array!($name, $ty, $ty);
    };

    ($name: ty, $ty: ty, $tyraw: ty) => {
        #[allow(dead_code)]
        impl $name {
            pub fn to_vec(&mut self) -> Vec<$ty> {
                let mut v = Vec::<$tyraw>::with_capacity(self.get_count() as usize);
                self.copy_to(v.as_mut_ptr() as *mut RawPtr);
                v.into_iter().map(|x| x.into()).collect()
            }

            pub fn from_vec(vec: &Vec<$ty>) -> Rc<RefCell<Self>> {
                let v = <$name>::create(vec.len() as i32).unwrap();
                v.borrow_mut()
                    .assign(vec.as_ptr() as *mut RawPtr, vec.len() as i32);
                v
            }
        }
    };
}

use crate::auto_generated_core_binding::*;
use crate::prelude::*;
use crate::structs::*;

use std::{cell::RefCell, rc::Rc};

impl_array!(Int8Array, u8);
impl_array!(Int32Array, i32);
impl_array!(FloatArray, f32);
impl_array!(VertexArray, vertex::Vertex, Vertex);
impl_array!(Vector2FArray, Vector2<f32>, Vector2F);
