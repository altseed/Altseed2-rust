use std::ops::{Div};
use num::{Zero, One, Float};

use super::{Vec2, Vec3, Vec4};

pub trait Vec<T> : Zero + One + Copy + Div<T, Output=Self> where T : Float {
    fn dot(&self, other: &Self) -> T;

    fn squared_len(&self) -> T {
        self.dot(self)
    }

    fn len(&self) -> T {
        self.squared_len().sqrt()
    }

    fn norm(&self) -> Self {
        *self / self.len()
    }
}

impl<T> Vec<T> for Vec2<T> where T : Copy + Div<T, Output=T> + Float {
    fn dot(&self, other: &Vec2<T>) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T> Vec<T> for Vec3<T> where T : Copy + Div<T, Output=T> + Float {
    fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T> Vec<T> for Vec4<T> where T : Copy + Div<T, Output=T> + Float {
    fn dot(&self, other: &Vec4<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}