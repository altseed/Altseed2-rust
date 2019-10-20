use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Default, Debug)]
pub struct Vec3<T> {
    pub x : T,
    pub y : T,
    pub z : T,
}

impl<T> Vec3<T> {
    pub fn new(x : T, y : T, z : T) -> Vec3<T> {
        Vec3{ x, y, z }
    }
}

impl<T> Neg for Vec3<T> where T : Copy + Neg<Output = T> {
    type Output = Vec3<T>;
    fn neg(self) -> Self::Output {
        Vec3 {
            x : -self.x,
            y : -self.y,
            z : -self.z,
        }
    }
}

impl<T> Add for Vec3<T> where T : Copy + Add<Output = T> {
    type Output = Vec3<T>;
    fn add(self, other : Vec3<T>) -> Self::Output {
        Vec3 {
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z,
        }
    }
}

impl<T> Sub for Vec3<T> where T : Copy + Sub<Output = T> {
    type Output = Vec3<T>;
    fn sub(self, other : Vec3<T>) -> Self::Output {
        Vec3 {
            x : self.x - other.x,
            y : self.y - other.y,
            z : self.z - other.z,
        }
    }
}

impl<T> Mul for Vec3<T> where T : Copy + Mul<Output = T> {
    type Output = Vec3<<T as Mul>::Output>;
    fn mul(self, other : Vec3<T>) -> Self::Output {
        Vec3 {
            x : self.x * other.x,
            y : self.y * other.y,
            z : self.z * other.z,
        }
    }
}

impl<T> Mul<T> for Vec3<T> where T : Copy + Mul<Output=T> {
    type Output = Vec3<T>;
    fn mul(self, other : T) -> Self::Output {
        Vec3 {
            x : self.x * other,
            y : self.y * other,
            z : self.z * other,
        }
    }
}

impl<T> Div for Vec3<T> where T : Copy + Div<Output = T> {
    type Output = Vec3<T>;
    fn div(self, other : Vec3<T>) -> Self::Output {
        Vec3 {
            x : self.x / other.x,
            y : self.y / other.y,
            z : self.z / other.z,
        }
    }
}

impl<T> Div<T> for Vec3<T> where T : Copy + Div<Output = T> {
    type Output = Vec3<T>;
    fn div(self, other : T) -> Self::Output {
        Vec3 {
            x : self.x / other,
            y : self.y / other,
            z : self.z / other,
        }
    }
}

impl<T> AddAssign for Vec3<T> where T : Copy + Add<Output = T> {
    fn add_assign(&mut self, other : Vec3<T>) {
        *self = *self + other;
    }
}

impl<T> SubAssign for Vec3<T> where T : Copy + Sub<Output = T> {
    fn sub_assign(&mut self, other : Vec3<T>) {
        *self = *self - other;
    }
}

impl<T> MulAssign<Vec3<T>> for Vec3<T> where T : Copy + Mul<Output = T> {
    fn mul_assign(&mut self, other : Vec3<T>) {
        *self = *self * other;
    }
}

impl<T> MulAssign<T> for Vec3<T> where T : Copy + Mul<Output = T> {
    fn mul_assign(&mut self, other : T) {
        *self = *self * other;
    }
}

impl<T> DivAssign<Vec3<T>> for Vec3<T> where T : Copy + Div<Output = T> {
    fn div_assign(&mut self, other : Vec3<T>) {
        *self = *self / other;
    }
}

impl<T> DivAssign<T> for Vec3<T> where T : Copy + Div<Output = T> {
    fn div_assign(&mut self, other : T) {
        *self = *self / other;
    }
}

use num::{Zero, One, Float};

impl<T> Zero for Vec3<T> where T : Copy + Add<T, Output=T> + Zero + PartialEq {
    fn zero() -> Self {
        let a = Zero::zero();
        Vec3::new(a, a, a)
    }

    fn is_zero(&self) -> bool {
        let a = Zero::zero();
        self.x == a && self.y == a && self.z == a
    }
}

impl<T> One for Vec3<T> where T : Copy + Add<T, Output=T> + One + PartialEq {
    fn one() -> Self {
        let a = One::one();
        Vec3::new(a, a, a)
    }

    fn is_one(&self) -> bool {
        let a = One::one();
        self.x == a && self.y == a && self.z == a
    }
}

impl<T> Vec3<T> where T : Float {
    pub fn cross(&self, other : &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x : self.y * other.z - self.z * other.y,
            y : self.z * other.x - self.x * other.z,
            z : self.x * other.y - self.y * other.x,
        }
    }
}