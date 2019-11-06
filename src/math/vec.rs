use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use num::{Zero, One, Float};

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

macro_rules! define_vec {($name:ident[$( $x:ident ),+]) => {
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
    pub struct $name<T> {
        $(
            pub $x : T,
        )+
    }

    impl<T> $name<T> {
        pub fn new( $($x : T),+) -> Self {
            $name{
                $( $x ),+
            }
        }
    }

    impl<T> Vec<T> for $name<T> where T : Copy + Div<T, Output=T> + Float {
        fn dot(&self, other: &$name<T>) -> T {
            $( (self.$x * other.$x)+ )+ Zero::zero()
        }
    }

    impl<T> Zero for $name<T> where T : Copy + Add<T, Output=T> + Zero + PartialEq {
        fn zero() -> Self {
            let a = Zero::zero();
            $name { $($x : a),+ }
        }

        fn is_zero(&self) -> bool {
            let a = Zero::zero();
            $(self.$x == a)&&+
        }
    }

    impl<T> One for $name<T> where T : Copy + Add<T, Output=T> + One + PartialEq {
        fn one() -> Self {
            let a = One::one();
            $name { $($x : a),+ }
        }

        fn is_one(&self) -> bool {
            let a = One::one();
            $(self.$x == a)&&+
        }
    }

    impl<T> Neg for $name<T> where T : Copy + Neg<Output = T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            $name { $($x : -self.$x),+ }
        }
    }

    impl<T> Add for $name<T> where T : Copy + Add<Output = T> {
        type Output = Self;
        fn add(self, other : $name<T>) -> Self::Output {
            $name { $($x : self.$x + other.$x),+ }
        }
    }

    impl<T> Sub for $name<T> where T : Copy + Sub<Output = T> {
        type Output = Self;
        fn sub(self, other : $name<T>) -> Self::Output {
            $name { $($x : self.$x - other.$x),+ }
        }
    }

    impl<T> Mul for $name<T> where T : Copy + Mul<Output = T> {
        type Output = $name<<T as Mul>::Output>;
        fn mul(self, other : $name<T>) -> Self::Output {
            $name { $($x : self.$x * other.$x),+ }
        }
    }

    impl<T> Mul<T> for $name<T> where T : Copy + Mul<Output=T> {
        type Output = Self;
        fn mul(self, other : T) -> Self::Output {
            $name { $($x : self.$x * other),+ }
        }
    }

    impl<T> Div for $name<T> where T : Copy + Div<Output = T> {
        type Output = Self;
        fn div(self, other : $name<T>) -> Self::Output {
            $name { $($x : self.$x / other.$x),+ }
        }
    }

    impl<T> Div<T> for $name<T> where T : Copy + Div<Output = T> {
        type Output = Self;
        fn div(self, other : T) -> Self::Output {
            $name { $($x : self.$x / other),+ }
        }
    }

    impl<T> AddAssign for $name<T> where T : Copy + Add<Output = T> {
        fn add_assign(&mut self, other : $name<T>) {
            *self = *self + other;
        }
    }

    impl<T> SubAssign for $name<T> where T : Copy + Sub<Output = T> {
        fn sub_assign(&mut self, other : $name<T>) {
            *self = *self - other;
        }
    }

    impl<T> MulAssign<$name<T>> for $name<T> where T : Copy + Mul<Output = T> {
        fn mul_assign(&mut self, other : $name<T>) {
            *self = *self * other;
        }
    }

    impl<T> MulAssign<T> for $name<T> where T : Copy + Mul<Output = T> {
        fn mul_assign(&mut self, other : T) {
            *self = *self * other;
        }
    }

    impl<T> DivAssign<$name<T>> for $name<T> where T : Copy + Div<Output = T> {
        fn div_assign(&mut self, other : $name<T>) {
            *self = *self / other;
        }
    }

    impl<T> DivAssign<T> for $name<T> where T : Copy + Div<Output = T> {
        fn div_assign(&mut self, other : T) {
            *self = *self / other;
        }
    }
};}

define_vec!(Vec1[x]);
define_vec!(Vec2[x, y]);
define_vec!(Vec3[x, y, z]);
define_vec!(Vec4[x, y, z, w]);

impl<T> Vec2<T> where T : Float {
    pub fn angle(&self) -> T {
        self.y.atan2(self.x)
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