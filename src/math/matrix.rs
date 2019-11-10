use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use num::{Zero, One};
use super::Dot;

pub trait Matrix {
    fn identity() -> Self;
    fn inverse(&self) -> Self;
}

macro_rules! define_matrix { ($name:ident, $nx:expr, $ny:expr) => {
    #[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
    pub struct $name<T> {
        pub values : [[T; $nx]; $ny]
    }

    impl<T> $name<T> {
        pub fn new(values: [[T; $nx]; $ny]) -> Self {
            $name { values }
        }
    }

    impl<T> Zero for $name<T> where T : Zero + Copy + PartialEq {
        fn zero() -> Self {
            let a = T::zero();
            $name { values : [[a; $nx]; $ny]}
        }

        fn is_zero(&self) -> bool {
            let a : T = Zero::zero();

            for x in 0..$nx {
            for y in 0..$ny {
                if self.values[x][y] != a {
                    return false;
                }
            }}

            return true;
        }
    }

    impl<T> One for $name<T> where T : One + Copy + PartialEq {
        fn one() -> Self {
            let a = T::one();
            $name { values : [[a; $nx]; $ny]}
        }
    }

    impl<T> Neg for $name<T> where T : Copy + Neg<Output = T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            let mut m = self;

            for x in 0..$nx {
            for y in 0..$ny {
                m.values[x][y] = -m.values[x][y];
            }}

            m
        }
    }

    impl<T> Add for $name<T> where T : Copy + Add<Output = T> {
        type Output = Self;
        fn add(self, other : Self) -> Self::Output {
            let mut m = self;

            for x in 0..$nx {
            for y in 0..$ny {
                m.values[x][y] = m.values[x][y] + other.values[x][y];
            }}

            m
        }
    }

    impl<T> Sub for $name<T> where T : Copy + Sub<Output = T> {
        type Output = Self;
        fn sub(self, other : Self) -> Self::Output {
            let mut m = self;

            for x in 0..$nx {
            for y in 0..$ny {
                m.values[x][y] = m.values[x][y] - other.values[x][y];
            }}

            m
        }
    }

    impl<T> Mul<T> for $name<T> where T : Copy + Mul<Output = T> {
        type Output = Self;
        fn mul(self, other : T) -> Self::Output {
            let mut m = self;

            for x in 0..$nx {
            for y in 0..$ny {
                m.values[x][y] = m.values[x][y] * other;
            }}

            m
        }
    }

    impl<T> Mul<$name<T>> for $name<T> where T : Copy + Mul<Output = T> {
        type Output = Self;
        fn mul(self, other : $name<T>) -> Self::Output {
            let mut m = self;

            for x in 0..$nx {
            for y in 0..$ny {
                m.values[x][y] = m.values[x][y] * other.values[x][y];
            }}

            m
        }
    }

    impl<T> Div<T> for $name<T> where T : Copy + Div<Output = T> {
        type Output = Self;
        fn div(self, other : T) -> Self::Output {
            let mut m = self;

            for x in 0..$nx {
            for y in 0..$ny {
                m.values[x][y] = m.values[x][y] / other;
            }}

            m
        }
    }

    impl<T> Div<$name<T>> for $name<T> where T : Copy + Div<Output = T> {
        type Output = Self;
        fn div(self, other : $name<T>) -> Self::Output {
            let mut m = self;

            for x in 0..$nx {
            for y in 0..$ny {
                m.values[x][y] = m.values[x][y] / other.values[x][y];
            }}

            m
        }
    }

    impl<T> AddAssign for $name<T> where T : Copy + Add<Output = T> {
        fn add_assign(&mut self, other : Self) {
            *self = *self + other;
        }
    }

    impl<T> SubAssign for $name<T> where T : Copy + Sub<Output = T> {
        fn sub_assign(&mut self, other : Self) {
            *self = *self - other;
        }
    }

    impl<T> MulAssign<T> for $name<T> where T : Copy + Mul<Output = T> {
        fn mul_assign(&mut self, other : T) {
            *self = *self * other;
        }
    }

    impl<T> DivAssign<T> for $name<T> where T : Copy + Div<Output = T> {
        fn div_assign(&mut self, other : T) {
            *self = *self / other;
        }
    }
};}

macro_rules! define_square_matrix { ($name:ident, $n:expr) => {
    define_matrix!($name, $n, $n);

    impl<T> Dot<Self> for $name<T> where T : Copy + Default + Add<T, Output=T> + Mul<T, Output=T> {
        type Output = Self;
        fn dot(self, other: Self) -> Self {
            let mut result = Self::default();

            for x in 0..$n {
            for y in 0..$n {
                result.values[x][y] = self.values[0][y] * other.values[x][0];
                for i in 1..$n {
                    result.values[x][y] = result.values[x][y] + self.values[i][y] * other.values[x][i];
                }
            }}

            result
        }
    }
};}

macro_rules! dot_vector {
    ($matrix:ident, $vector:ident[$($x:ident),+]) => {
        dot_vector!(@step 0usize, $matrix, $vector[], $($x,)+);
    };

    (@step $idx:expr, $matrix:ident, $vector:ident[$( ($x:ident, $i:expr) ),*], $head:ident, $($tail:ident,)*) => {
        dot_vector!(@step $idx + 1usize, $matrix, $vector[$( ($x, $i), )* ($head, $idx)], $($tail,)*);
    };

    (@step $_idx:expr, $matrix:ident, $vector:ident[$( ($x:ident, $i:expr) ),+], ) => {
        impl<T> Dot<$matrix<T>> for $vector<T> where T : Copy + Default + Zero + Add<Output = T> + Mul<Output = T> {
            type Output = $vector<T>;
            fn dot(self, other: $matrix<T>) -> Self::Output {
                let mut v = Self::Output::default();
                let mut i = 0;
                for a in self.to_array().into_iter() {
                    $(
                        v.$x = v.$x + *a * other.values[$i][i];
                    )+

                    i += 1;
                }

                v
            }
        }

        impl<T> Dot<$vector<T>> for $matrix<T> where T : Copy + Default + Add<Output = T> + Mul<Output = T> {
            type Output = $vector<T>;
            fn dot(self, other: $vector<T>) -> Self::Output {
                let mut v = Self::Output::default();
                let mut i = 0;
                for a in other.to_array().into_iter() {
                    $(
                        v.$x = v.$x + *a * self.values[i][$i];
                    )+

                    i += 1;
                }

                v
            }
        }
    };
}

use crate::math::vector::{Vector2, Vector3, Vector4};

define_square_matrix!(Matrix2x2, 2);
define_square_matrix!(Matrix3x3, 3);
define_square_matrix!(Matrix4x4, 4);
dot_vector!(Matrix2x2, Vector2[x, y]);
dot_vector!(Matrix3x3, Vector3[x, y, z]);
dot_vector!(Matrix4x4, Vector4[x, y, z, w]);
