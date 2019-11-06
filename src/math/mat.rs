use std::ops::{Add, Mul};
use num::{Zero, One};

use super::mat22::Mat22;

pub trait Mat : Zero + One {
    fn transpose(&self) -> Self;
}

impl<T> Mat for Mat22<T> where T : Copy + Zero + One + PartialEq + Add<Output = T> + Mul<Output = T> {
    fn transpose(&self) -> Self {
        Mat22 { values : [
            [self.values[0][0], self.values[1][0]],
            [self.values[0][1], self.values[1][1]]
        ]}
    }
}