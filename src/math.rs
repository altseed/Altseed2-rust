/// 内積を表します。
pub trait Dot<T> {
    type Output;

    fn dot(self, other: T) -> Self::Output;
}

mod easing;
mod matrix;
mod vector;

pub use easing::Easing;
pub use matrix::*;
pub use vector::*;
