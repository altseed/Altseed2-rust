use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use num::{Zero, One};

use super::Vec2;

#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub struct Mat22<T> {
    pub(crate) values : [[T; 2]; 2]
}

impl<T> Mat22<T> {
    pub fn new(values: [[T; 2]; 2]) -> Self {
        Mat22 { values }
    }
}

impl<T> Zero for Mat22<T> where T : Copy + Add<T, Output=T> + Zero + PartialEq {
    fn zero() -> Self {
        Mat22 { values : [[Zero::zero(); 2]; 2]}
    }

    fn is_zero(&self) -> bool {
        *self == Zero::zero()
    }
}

impl<T> One for Mat22<T> where T : Copy + Add<Output = T> + Mul<Output = T> + One + PartialEq {
    fn one() -> Self {
        Mat22 { values : [[One::one(); 2]; 2]}
    }

    fn is_one(&self) -> bool {
        *self == One::one()
    }
}

impl<T> Neg for Mat22<T> where T : Copy + Neg<Output = T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Mat22 { values : [
            [ -self.values[0][0], -self.values[0][1]],
            [ -self.values[1][0], -self.values[1][1]] ]}
    }
}

impl<T> Add for Mat22<T> where T : Copy + Add<Output = T> {
    type Output = Self;
    fn add(self, other : Self) -> Self::Output {
        Mat22 { values : [
            [ self.values[0][0] + other.values[0][0], self.values[0][1] + other.values[0][1] ],
            [ self.values[1][0] + other.values[1][0], self.values[1][1] + other.values[1][1] ] ]}
    }
}

impl<T> Sub for Mat22<T> where T : Copy + Sub<Output = T> {
    type Output = Self;
    fn sub(self, other : Self) -> Self::Output {
        Mat22 { values : [
            [ self.values[0][0] - other.values[0][0], self.values[0][1] - other.values[0][1] ],
            [ self.values[1][0] - other.values[1][0], self.values[1][1] - other.values[1][1] ] ]}
    }
}

impl<T> Mul<T> for Mat22<T> where T : Copy + Mul<Output = T> {
    type Output = Self;
    fn mul(self, other : T) -> Self::Output {
        Mat22 { values : [
            [ self.values[0][0] * other, self.values[0][1] * other ],
            [ self.values[1][0] * other, self.values[1][1] * other ] ]}
    }
}

impl<T> Mul for Mat22<T> where T : Copy + Add<Output = T> + Mul<Output = T> {
    type Output = Self;
    fn mul(self, other : Mat22<T>) -> Self::Output {
        Mat22 { values : [[
            self.values[0][0] * other.values[0][0] + self.values[0][1] * other.values[1][0],
            self.values[0][0] * other.values[0][1] + self.values[0][1] * other.values[1][1]
            ], [
            self.values[1][0] * other.values[0][0] + self.values[1][1] * other.values[1][0],
            self.values[1][0] * other.values[0][1] + self.values[1][1] * other.values[1][1]
        ]]}
    }
}

impl<T> Mul<Vec2<T>> for Mat22<T> where T : Copy + Add<Output = T> + Mul<Output = T> {
    type Output = Vec2<T>;
    fn mul(self, other: Vec2<T>) -> Self::Output {
        Vec2 {
            x : other.x * self.values[0][0] + other.y * self.values[0][1],
            y : other.x * self.values[1][0] + other.y * self.values[1][1],
        }
    }
}

impl<T> Mul<Mat22<T>> for Vec2<T> where T : Copy + Add<Output = T> + Mul<Output = T> {
    type Output = Vec2<T>;
    fn mul(self, other: Mat22<T>) -> Self::Output {
        Vec2 {
            x : self.x * other.values[0][0] + self.y * other.values[1][0],
            y : self.x * other.values[0][1] + self.y * other.values[1][1],
        }
    }
}

impl<T> Div<T> for Mat22<T> where T : Copy + Div<Output = T> {
    type Output = Self;
    fn div(self, other : T) -> Self::Output {
        Mat22 { values : [
            [ self.values[0][0] / other, self.values[0][1] / other ],
            [ self.values[1][0] / other, self.values[1][1] / other ] ]}
    }
}

impl<T> AddAssign for Mat22<T> where T : Copy + Add<Output = T> {
    fn add_assign(&mut self, other : Self) {
        *self = *self + other;
    }
}

impl<T> SubAssign for Mat22<T> where T : Copy + Sub<Output = T> {
    fn sub_assign(&mut self, other : Self) {
        *self = *self - other;
    }
}

impl<T> MulAssign for Mat22<T> where T : Copy + Add<Output = T> + Mul<Output = T> {
    fn mul_assign(&mut self, other : Self) {
        *self = *self * other;
    }
}

impl<T> MulAssign<T> for Mat22<T> where T : Copy + Mul<Output = T> {
    fn mul_assign(&mut self, other : T) {
        *self = *self * other;
    }
}

impl<T> DivAssign<T> for Mat22<T> where T : Copy + Div<Output = T> {
    fn div_assign(&mut self, other : T) {
        *self = *self / other;
    }
}
