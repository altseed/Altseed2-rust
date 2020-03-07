// Inner product
pub trait Dot<T> {
    type Output;

    fn dot(self, other: T) -> Self::Output;
}

pub mod easing;
pub mod vector;
pub mod matrix;
pub use vector::{Vector2, Vector3, Vector4};
pub use matrix::{Matrix44};

// bindings to ffi structs
macro_rules! vector_to_ffi {($name:ident<$t:ty>[$( $x:ident ),+], $target:ident) => {
    impl From<crate::structs::$target> for $name<$t> {
        fn from(item: crate::structs::$target) -> Self {
            Self { $($x : item.$x),+ }
        }
    }
    impl Into<crate::structs::$target> for $name<$t> {
        fn into(self) -> crate::structs::$target {
            crate::structs::$target { $($x : self.$x),+ }
        }
    }
};}

vector_to_ffi!(Vector2<i32>[x, y], Vector2I);
vector_to_ffi!(Vector2<f32>[x, y], Vector2F);
vector_to_ffi!(Vector3<i32>[x, y, z], Vector3I);
vector_to_ffi!(Vector3<f32>[x, y, z], Vector3F);
vector_to_ffi!(Vector4<i32>[x, y, z, w], Vector4I);
vector_to_ffi!(Vector4<f32>[x, y, z, w], Vector4F);

macro_rules! matrix_to_ffi {($name:ident<$t:ty>, $target:ident) => {
    impl From<crate::structs::$target> for $name<$t> {
        fn from(item: crate::structs::$target) -> Self {
            Self { values: item.values.clone() }
        }
    }
    impl Into<crate::structs::$target> for $name<$t> {
        fn into(self) -> crate::structs::$target {
            crate::structs::$target { values: self.values.clone() }
        }
    }
};}

matrix_to_ffi!(Matrix44<i32>, Matrix44I);
matrix_to_ffi!(Matrix44<f32>, Matrix44F);