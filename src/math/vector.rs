use super::Dot;
use num::{Float, One, Zero};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Vector<T: Float>: Dot<Self, Output = T> + Copy + Div<T, Output = Self> {
    /// ベクトルの長さの二乗の値を取得します。
    fn squared_len(&self) -> T {
        self.dot(self.clone())
    }

    /// ベクトルの長さを取得します。
    fn len(&self) -> T {
        self.squared_len().sqrt()
    }

    /// ベクトルを正規化した値を取得します。
    fn norm(&self) -> Self {
        *self / self.len()
    }
}

// implement for Vector structs `to_array` method which converts `Vector` to array
// get the length of repetation pattern using recursive
// https://stackoverflow.com/questions/32817193/how-to-get-index-of-macro-repetition-single-element
macro_rules! vector_to_array {
    (@step $idx:expr, [$($x:ident,)*],) => {
        #[inline(always)]
        /// 固定長配列へ変換します。
        pub fn to_array(&self) -> [T; $idx] {
            [$(self.$x,)+]
        }
    };

    (@step $idx:expr, [$($x:ident,)+], $head:ident, $($tail:ident,)*) => {
        vector_to_array!(@step $idx + 1usize, [$($x,)+], $($tail,)*);
    };

    ($name:ident[$($n:ident),+]) => {
        impl<T> $name<T> where T : Copy {
            vector_to_array!(@step 0usize, [$($n,)+], $($n,)+);
        }
    };
}

// define `Vector` structs
macro_rules! define_vector {
    ($(#[$meta_s:meta])*
    $name:ident[$( $x:ident ),+]
    ) => {
    $(#[$meta_s])*
    #[repr(C)]
    #[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug)]
    pub struct $name<T> {
        $(
            pub $x : T,
        )+
    }

    impl<T> $name<T> {
        pub fn new( $($x : T),+) -> Self {
            $name {
                $( $x ),+
            }
        }
    }

    vector_to_array!($name[$($x),+]);

    impl<T> Dot<$name<T>> for $name<T> where T : Copy + Zero + Add<T, Output=T> + Mul<T, Output=T> {
        type Output = T;
        fn dot(self, other: $name<T>) -> Self::Output {
            $( (self.$x * other.$x)+ )+ Zero::zero()
        }
    }

    impl<T> Vector<T> for $name<T> where T : Float {}

    impl<T> Zero for $name<T> where T : Zero + Copy + PartialEq {
        fn zero() -> Self {
            let a = Zero::zero();
            $name { $($x : a),+ }
        }

        fn is_zero(&self) -> bool {
            let a = Zero::zero();
            $(self.$x == a)&&+
        }
    }

    impl<T> One for $name<T> where T : One + Copy + PartialEq {
        fn one() -> Self {
            let a = One::one();
            $name { $($x : a),+ }
        }

        fn is_one(&self) -> bool {
            let a = One::one();
            $(self.$x == a)&&+
        }
    }

    impl<T> Default for $name<T> where T : Default {
        fn default() -> Self {
            $name { $($x : T::default()),+ }
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

macro_rules! impl_convert {
    ($name:ident[$( $x:ident ),+], $t1: ty, $t2: ty) => {
        impl From<$name<$t1>> for $name<$t2> {
            fn from(item: $name<$t1>) -> $name<$t2> {
                $name { $($x: item.$x as $t2),+ }
            }
        }

        impl From<$name<$t2>> for $name<$t1> {
            fn from(item: $name<$t2>) -> $name<$t1> {
                $name { $($x: item.$x as $t1),+ }
            }
        }
    };
}

define_vector! {
    /// 2次元ベクトルを表します。
    Vector2[x, y]
}

define_vector! {
    /// 3次元ベクトルを表します。
    Vector3[x, y, z]
}

define_vector! {
    /// 4次元ベクトルを表します。
    Vector4[x, y, z, w]
}

impl_convert!(Vector2[x, y], i32, f32);
impl_convert!(Vector3[x, y, z], i32, f32);
impl_convert!(Vector4[x, y, z, w], i32, f32);

impl<T> Vector2<T>
where
    T: Float,
{
    pub fn angle(&self) -> T {
        self.y.atan2(self.x)
    }
}

impl<T> Vector3<T>
where
    T: Float,
{
    pub fn cross(&self, other: &Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}
