/// 内積を表します。
pub trait Dot<T> {
    type Output;

    fn dot(self, other: T) -> Self::Output;
}

pub mod easing;
pub mod matrix;
pub mod vector;

pub use easing::Easing;
pub use matrix::*;
pub use vector::*;
