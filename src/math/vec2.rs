use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use num::{Zero, One, Float};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct Vec2<T> {
    pub x : T,
    pub y : T,
}

impl<T> Vec2<T> {
    pub fn new(x : T, y : T) -> Vec2<T> {
        Vec2{ x, y }
    }
}

impl<T> Vec2<T> where T : Float {
    pub fn angle(&self) -> T {
        self.y.atan2(self.x)
    }
}

impl<T> Zero for Vec2<T> where T : Copy + Add<T, Output=T> + Zero + PartialEq {
    fn zero() -> Self {
        let a = Zero::zero();
        Vec2::new(a, a)
    }

    fn is_zero(&self) -> bool {
        let a = Zero::zero();
        self.x == a && self.y == a
    }
}

impl<T> One for Vec2<T> where T : Copy + Add<T, Output=T> + One + PartialEq {
    fn one() -> Self {
        let a = One::one();
        Vec2::new(a, a)
    }

    fn is_one(&self) -> bool {
        let a = One::one();
        self.x == a && self.y == a
    }
}

impl<T> Neg for Vec2<T> where T : Copy + Neg<Output = T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec2 {
            x : -self.x,
            y : -self.y,
        }
    }
}

impl<T> Add for Vec2<T> where T : Copy + Add<Output = T> {
    type Output = Self;
    fn add(self, other : Vec2<T>) -> Self::Output {
        Vec2 {
            x : self.x + other.x,
            y : self.y + other.y,
        }
    }
}

impl<T> Sub for Vec2<T> where T : Copy + Sub<Output = T> {
    type Output = Self;
    fn sub(self, other : Vec2<T>) -> Self::Output {
        Vec2 {
            x : self.x - other.x,
            y : self.y - other.y,
        }
    }
}

impl<T> Mul for Vec2<T> where T : Copy + Mul<Output = T> {
    type Output = Vec2<<T as Mul>::Output>;
    fn mul(self, other : Vec2<T>) -> Self::Output {
        Vec2 {
            x : self.x * other.x,
            y : self.y * other.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T> where T : Copy + Mul<Output=T> {
    type Output = Self;
    fn mul(self, other : T) -> Self::Output {
        Vec2 {
            x : self.x * other,
            y : self.y * other,
        }
    }
}

impl<T> Div for Vec2<T> where T : Copy + Div<Output = T> {
    type Output = Self;
    fn div(self, other : Vec2<T>) -> Self::Output {
        Vec2 {
            x : self.x / other.x,
            y : self.y / other.y,
        }
    }
}

impl<T> Div<T> for Vec2<T> where T : Copy + Div<Output = T> {
    type Output = Self;
    fn div(self, other : T) -> Self::Output {
        Vec2 {
            x : self.x / other,
            y : self.y / other,
        }
    }
}

impl<T> AddAssign for Vec2<T> where T : Copy + Add<Output = T> {
    fn add_assign(&mut self, other : Vec2<T>) {
        *self = *self + other;
    }
}

impl<T> SubAssign for Vec2<T> where T : Copy + Sub<Output = T> {
    fn sub_assign(&mut self, other : Vec2<T>) {
        *self = *self - other;
    }
}

impl<T> MulAssign<Vec2<T>> for Vec2<T> where T : Copy + Mul<Output = T> {
    fn mul_assign(&mut self, other : Vec2<T>) {
        *self = *self * other;
    }
}

impl<T> MulAssign<T> for Vec2<T> where T : Copy + Mul<Output = T> {
    fn mul_assign(&mut self, other : T) {
        *self = *self * other;
    }
}

impl<T> DivAssign<Vec2<T>> for Vec2<T> where T : Copy + Div<Output = T> {
    fn div_assign(&mut self, other : Vec2<T>) {
        *self = *self / other;
    }
}

impl<T> DivAssign<T> for Vec2<T> where T : Copy + Div<Output = T> {
    fn div_assign(&mut self, other : T) {
        *self = *self / other;
    }
}