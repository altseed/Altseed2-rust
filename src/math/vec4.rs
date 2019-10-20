use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Default, Debug)]
pub struct Vec4<T> {
    pub x : T,
    pub y : T,
    pub z : T,
    pub w : T,
}

impl<T> Vec4<T> {
    pub fn new(x : T, y : T, z : T, w : T) -> Vec4<T> {
        Vec4{ x, y, z, w }
    }
}

impl<T> Neg for Vec4<T> where T : Copy + Neg<Output = T> {
    type Output = Vec4<T>;
    fn neg(self) -> Self::Output {
        Vec4 {
            x : -self.x,
            y : -self.y,
            z : -self.z,
            w : -self.w,
        }
    }
}

impl<T> Add for Vec4<T> where T : Copy + Add<Output = T> {
    type Output = Vec4<T>;
    fn add(self, other : Vec4<T>) -> Self::Output {
        Vec4 {
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z,
            w : self.w + other.w,
        }
    }
}

impl<T> Sub for Vec4<T> where T : Copy + Sub<Output = T> {
    type Output = Vec4<T>;
    fn sub(self, other : Vec4<T>) -> Self::Output {
        Vec4 {
            x : self.x - other.x,
            y : self.y - other.y,
            z : self.z - other.z,
            w : self.w - other.w,
        }
    }
}

impl<T> Mul for Vec4<T> where T : Copy + Mul<Output = T> {
    type Output = Vec4<<T as Mul>::Output>;
    fn mul(self, other : Vec4<T>) -> Self::Output {
        Vec4 {
            x : self.x * other.x,
            y : self.y * other.y,
            z : self.z * other.z,
            w : self.w * other.w,
        }
    }
}

impl<T> Mul<T> for Vec4<T> where T : Copy + Mul<Output=T> {
    type Output = Vec4<T>;
    fn mul(self, other : T) -> Self::Output {
        Vec4 {
            x : self.x * other,
            y : self.y * other,
            z : self.z * other,
            w : self.w * other,
        }
    }
}

impl<T> Div for Vec4<T> where T : Copy + Div<Output = T> {
    type Output = Vec4<T>;
    fn div(self, other : Vec4<T>) -> Self::Output {
        Vec4 {
            x : self.x / other.x,
            y : self.y / other.y,
            z : self.z / other.z,
            w : self.w / other.w,
        }
    }
}

impl<T> Div<T> for Vec4<T> where T : Copy + Div<Output = T> {
    type Output = Vec4<T>;
    fn div(self, other : T) -> Self::Output {
        Vec4 {
            x : self.x / other,
            y : self.y / other,
            z : self.z / other,
            w : self.w / other,
        }
    }
}

impl<T> AddAssign for Vec4<T> where T : Copy + Add<Output = T> {
    fn add_assign(&mut self, other : Vec4<T>) {
        *self = *self + other;
    }
}

impl<T> SubAssign for Vec4<T> where T : Copy + Sub<Output = T> {
    fn sub_assign(&mut self, other : Vec4<T>) {
        *self = *self - other;
    }
}

impl<T> MulAssign<Vec4<T>> for Vec4<T> where T : Copy + Mul<Output = T> {
    fn mul_assign(&mut self, other : Vec4<T>) {
        *self = *self * other;
    }
}

impl<T> MulAssign<T> for Vec4<T> where T : Copy + Mul<Output = T> {
    fn mul_assign(&mut self, other : T) {
        *self = *self * other;
    }
}

impl<T> DivAssign<Vec4<T>> for Vec4<T> where T : Copy + Div<Output = T> {
    fn div_assign(&mut self, other : Vec4<T>) {
        *self = *self / other;
    }
}

impl<T> DivAssign<T> for Vec4<T> where T : Copy + Div<Output = T> {
    fn div_assign(&mut self, other : T) {
        *self = *self / other;
    }
}

use num::{Zero, One};

impl<T> Zero for Vec4<T> where T : Copy + Add<T, Output=T> + Zero + PartialEq {
    fn zero() -> Self {
        let a = Zero::zero();
        Vec4::new(a, a, a, a)
    }

    fn is_zero(&self) -> bool {
        let a = Zero::zero();
        self.x == a && self.y == a && self.z == a && self.z == a
    }
}

impl<T> One for Vec4<T> where T : Copy + Add<T, Output=T> + One + PartialEq {
    fn one() -> Self {
        let a = One::one();
        Vec4::new(a, a, a, a)
    }

    fn is_one(&self) -> bool {
        let a = One::one();
        self.x == a && self.y == a && self.z == a && self.w == a
    }
}