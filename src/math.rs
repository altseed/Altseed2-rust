pub trait Dot<T> {
    type Output;

    fn dot(self, other: T) -> Self::Output;
}

pub mod vector;
pub mod matrix;