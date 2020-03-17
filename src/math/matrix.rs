use super::Dot;
use num::{One, Zero};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Matrix {}

// define `Matrix` structs
macro_rules! define_matrix {
    ($name:ident, $nx:expr, $ny:expr) => {
        #[derive(Clone, PartialEq, Default, Debug)]
        pub struct $name<T> {
            pub values: [[T; $nx]; $ny],
        }

        impl<T> $name<T> {
            pub fn new(values: [[T; $nx]; $ny]) -> Self {
                $name { values }
            }
        }

        impl<T> Matrix for $name<T> {}

        impl<T> Zero for $name<T>
        where
            T: Zero + Copy + PartialEq,
        {
            fn zero() -> Self {
                let a = T::zero();
                $name {
                    values: [[a; $nx]; $ny],
                }
            }

            fn is_zero(&self) -> bool {
                let a: T = Zero::zero();

                for y in 0..$ny {
                    for x in 0..$nx {
                        if self.values[y][x] != a {
                            return false;
                        }
                    }
                }

                return true;
            }
        }

        impl<T> Neg for &$name<T>
        where
            T: Copy + Neg<Output = T>,
        {
            type Output = $name<T>;
            fn neg(self) -> Self::Output {
                let mut m = self.clone();

                for y in 0..$ny {
                    for x in 0..$nx {
                        m.values[y][x] = -m.values[y][x];
                    }
                }

                m
            }
        }

        impl<T> Neg for $name<T>
        where
            T: Copy + Neg<Output = T>,
        {
            type Output = $name<T>;
            fn neg(self) -> Self::Output {
                -&self
            }
        }

        impl<T> Add<&$name<T>> for &$name<T>
        where
            T: Copy + Add<Output = T>,
        {
            type Output = $name<T>;
            fn add(self, other: &$name<T>) -> Self::Output {
                let mut m = self.clone();

                for y in 0..$ny {
                    for x in 0..$nx {
                        m.values[y][x] = m.values[y][x] + other.values[y][x];
                    }
                }

                m
            }
        }

        impl<T> Add for $name<T>
        where
            T: Copy + Add<Output = T>,
        {
            type Output = $name<T>;
            fn add(self, other: Self) -> Self::Output {
                &self + &other
            }
        }

        impl<T> Sub<&$name<T>> for &$name<T>
        where
            T: Copy + Sub<Output = T>,
        {
            type Output = $name<T>;
            fn sub(self, other: &$name<T>) -> Self::Output {
                let mut m = self.clone();

                for y in 0..$ny {
                    for x in 0..$nx {
                        m.values[y][x] = m.values[y][x] - other.values[y][x];
                    }
                }

                m
            }
        }

        impl<T> Sub for $name<T>
        where
            T: Copy + Sub<Output = T>,
        {
            type Output = $name<T>;
            fn sub(self, other: Self) -> Self::Output {
                &self - &other
            }
        }

        impl<T> Mul<T> for &$name<T>
        where
            T: Copy + Default + Mul<T, Output = T>,
        {
            type Output = $name<T>;
            fn mul(self, other: T) -> Self::Output {
                let mut m = self.clone();
                for y in 0..$ny {
                    for x in 0..$nx {
                        m.values[y][x] = self.values[y][x] * other;
                    }
                }

                m
            }
        }

        impl<T> Mul<T> for $name<T>
        where
            T: Copy + Default + Mul<T, Output = T>,
        {
            type Output = $name<T>;
            fn mul(self, other: T) -> Self::Output {
                &self * other
            }
        }

        impl<T> AddAssign for $name<T>
        where
            T: Copy + Add<Output = T>,
        {
            fn add_assign(&mut self, other: Self) {
                for y in 0..$ny {
                    for x in 0..$nx {
                        self.values[y][x] = self.values[y][x] + other.values[y][x];
                    }
                }
            }
        }

        impl<T> SubAssign for $name<T>
        where
            T: Copy + Sub<Output = T>,
        {
            fn sub_assign(&mut self, other: Self) {
                for y in 0..$ny {
                    for x in 0..$nx {
                        self.values[y][x] = self.values[y][x] - other.values[y][x];
                    }
                }
            }
        }

        impl<T> MulAssign<T> for $name<T>
        where
            T: Copy + Default + Mul<T, Output = T>,
        {
            fn mul_assign(&mut self, other: T) {
                for y in 0..$ny {
                    for x in 0..$nx {
                        self.values[y][x] = self.values[y][x] * other;
                    }
                }
            }
        }
    };
}

// define `Square Matrix` structs
// calling define_matrix inside
macro_rules! define_square_matrix {
    ($name:ident, $n:expr) => {
        define_matrix!($name, $n, $n);

        impl<T> One for $name<T>
        where
            T: Zero + One + Copy + PartialEq + Default + Add<T, Output = T> + Mul<T, Output = T>,
        {
            fn one() -> Self {
                let mut mat = $name {
                    values: [[T::zero(); $n]; $n],
                };
                for x in 0..$n {
                    mat.values[x][x] = T::one();
                }
                mat
            }
        }

        impl<T> Dot<&$name<T>> for &$name<T>
        where
            T: Copy + Default + Add<T, Output = T> + Mul<T, Output = T>,
        {
            type Output = $name<T>;
            fn dot(self, other: &$name<T>) -> Self::Output {
                let mut result = $name::default();

                for y in 0..$n {
                    for x in 0..$n {
                        for i in 0..$n {
                            result.values[y][x] =
                                result.values[y][x] + self.values[y][i] * other.values[i][x];
                        }
                    }
                }

                result
            }
        }

        impl<T> Dot<Self> for $name<T>
        where
            T: Copy + Default + Add<T, Output = T> + Mul<T, Output = T>,
        {
            type Output = $name<T>;
            fn dot(self, other: Self) -> Self {
                Dot::dot(&self, &other)
            }
        }

        impl<T> Mul<&$name<T>> for &$name<T>
        where
            T: Copy + Default + Add<T, Output = T> + Mul<T, Output = T>,
        {
            type Output = $name<T>;
            fn mul(self, other: &$name<T>) -> Self::Output {
                Dot::dot(self, other)
            }
        }

        impl<T> Mul<$name<T>> for $name<T>
        where
            T: Copy + Default + Add<T, Output = T> + Mul<T, Output = T>,
        {
            type Output = $name<T>;
            fn mul(self, other: $name<T>) -> Self::Output {
                Dot::dot(self, other)
            }
        }

        impl<T> MulAssign<$name<T>> for $name<T>
        where
            T: Copy + Default + Add<T, Output = T> + Mul<T, Output = T>,
        {
            fn mul_assign(&mut self, other: $name<T>) {
                *self = self.clone() * other;
            }
        }

        impl<T> $name<T>
        where
            T: Copy,
        {
            pub fn set_transposed(&mut self) -> &mut Self {
                for y in 0..$n {
                    for x in 0..$n {
                        let t = self.values[y][x];
                        self.values[y][x] = self.values[x][y];
                        self.values[x][y] = t;
                    }
                }

                self
            }

            pub fn get_transposed(&self) -> Self {
                let mut m = self.clone();
                m.set_transposed();
                m
            }
        }
    };
}

use crate::math::vector::{Vector3, Vector4};

define_square_matrix!(Matrix33, 3);
define_square_matrix!(Matrix44, 4);

impl Matrix44<f32> {
    pub fn identity() -> &'static Self {
        lazy_static! {
            static ref IDENITTY: Matrix44<f32> = Matrix44::one();
        }
        &IDENITTY
    }
}

impl Matrix44<f32> {
    /// 平行移動行列を取得する。
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut m = Self::identity().clone();
        m.values[0][3] = x;
        m.values[1][3] = y;
        m.values[2][3] = z;
        m
    }

    // 拡大行列を取得する。
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        let mut m = Self::identity().clone();
        m.values[0][0] = x;
        m.values[1][1] = y;
        m.values[2][2] = z;
        m.values[3][3] = 1.0;
        m
    }

    /// Z軸回転行列(右手)を取得する。
    /// # Arguments
    /// * angle - Z軸回転量(ラジアン)
    pub fn rotation_z(angle: f32) -> Self {
        let s = angle.sin();
        let c = angle.cos();

        let mut m = Self::identity().clone();

        m.values[0][0] = c;
        m.values[1][0] = s;
        m.values[0][1] = -s;
        m.values[1][1] = c;

        m
    }

    /// 2次元ベクトルを変形させる。
    pub fn transform_3d(&self, v: Vector3<f32>) -> Vector3<f32> {
        let mut values = [0.0; 4];

        for i in 0..4 {
            values[i] = v.x * self.values[i][0]
                + v.y * self.values[i][1]
                + v.z * self.values[i][2]
                + self.values[i][3];
        }

        Vector3 {
            x: values[0] / values[3],
            y: values[1] / values[3],
            z: values[2] / values[3],
        }
    }

    /// 3次元ベクトルを変形させる。
    pub fn transform_4d(&self, v: Vector4<f32>) -> Vector4<f32> {
        let mut values = [0.0; 4];

        for i in 0..4 {
            values[i] = v.x * self.values[i][0]
                + v.y * self.values[i][1]
                + v.z * self.values[i][2]
                + self.values[i][3];
        }

        Vector4 {
            x: values[0],
            y: values[1],
            z: values[2],
            w: values[3],
        }
    }

    // 逆行列を取得する
    pub fn inverted(&self) -> Self {
        let mut m = self.clone();
        m.set_inverted();
        m
    }

    fn set_inverted(&mut self) -> &mut Self {
        let e = 0.00001f32;

        let a11 = self.values[0][0];
        let a12 = self.values[0][1];
        let a13 = self.values[0][2];
        let a14 = self.values[0][3];
        let a21 = self.values[1][0];
        let a22 = self.values[1][1];
        let a23 = self.values[1][2];
        let a24 = self.values[1][3];
        let a31 = self.values[2][0];
        let a32 = self.values[2][1];
        let a33 = self.values[2][2];
        let a34 = self.values[2][3];
        let a41 = self.values[3][0];
        let a42 = self.values[3][1];
        let a43 = self.values[3][2];
        let a44 = self.values[3][3];

        /* 行列式の計算 */
        let b11 = a22 * (a33 * a44 - a43 * a34) - a23 * (a32 * a44 - a42 * a34)
            + a24 * (a32 * a43 - a42 * a33);
        let b12 = -a12 * (a33 * a44 - a43 * a34) + a13 * (a32 * a44 - a42 * a34)
            - a14 * (a32 * a43 - a42 * a33);
        let b13 = a12 * (a23 * a44 - a43 * a24) - a13 * (a22 * a44 - a42 * a24)
            + a14 * (a22 * a43 - a42 * a23);
        let b14 = -a12 * (a23 * a34 - a33 * a24) + a13 * (a22 * a34 - a32 * a24)
            - a14 * (a22 * a33 - a32 * a23);

        let b21 = -a21 * (a33 * a44 - a43 * a34) + a23 * (a31 * a44 - a41 * a34)
            - a24 * (a31 * a43 - a41 * a33);
        let b22 = a11 * (a33 * a44 - a43 * a34) - a13 * (a31 * a44 - a41 * a34)
            + a14 * (a31 * a43 - a41 * a33);
        let b23 = -a11 * (a23 * a44 - a43 * a24) + a13 * (a21 * a44 - a41 * a24)
            - a14 * (a21 * a43 - a41 * a23);
        let b24 = a11 * (a23 * a34 - a33 * a24) - a13 * (a21 * a34 - a31 * a24)
            + a14 * (a21 * a33 - a31 * a23);

        let b31 = a21 * (a32 * a44 - a42 * a34) - a22 * (a31 * a44 - a41 * a34)
            + a24 * (a31 * a42 - a41 * a32);
        let b32 = -a11 * (a32 * a44 - a42 * a34) + a12 * (a31 * a44 - a41 * a34)
            - a14 * (a31 * a42 - a41 * a32);
        let b33 = a11 * (a22 * a44 - a42 * a24) - a12 * (a21 * a44 - a41 * a24)
            + a14 * (a21 * a42 - a41 * a22);
        let b34 = -a11 * (a22 * a34 - a32 * a24) + a12 * (a21 * a34 - a31 * a24)
            - a14 * (a21 * a32 - a31 * a22);

        let b41 = -a21 * (a32 * a43 - a42 * a33) + a22 * (a31 * a43 - a41 * a33)
            - a23 * (a31 * a42 - a41 * a32);
        let b42 = a11 * (a32 * a43 - a42 * a33) - a12 * (a31 * a43 - a41 * a33)
            + a13 * (a31 * a42 - a41 * a32);
        let b43 = -a11 * (a22 * a43 - a42 * a23) + a12 * (a21 * a43 - a41 * a23)
            - a13 * (a21 * a42 - a41 * a22);
        let b44 = a11 * (a22 * a33 - a32 * a23) - a12 * (a21 * a33 - a31 * a23)
            + a13 * (a21 * a32 - a31 * a22);

        // 行列式の逆数をかける
        let det = (a11 * b11) + (a12 * b21) + (a13 * b31) + (a14 * b41);
        if (-e <= det) && (det <= e) {
            return self;
        }

        let inv_det = 1.0 / det;

        self.values[0][0] = b11 * inv_det;
        self.values[0][1] = b12 * inv_det;
        self.values[0][2] = b13 * inv_det;
        self.values[0][3] = b14 * inv_det;
        self.values[1][0] = b21 * inv_det;
        self.values[1][1] = b22 * inv_det;
        self.values[1][2] = b23 * inv_det;
        self.values[1][3] = b24 * inv_det;
        self.values[2][0] = b31 * inv_det;
        self.values[2][1] = b32 * inv_det;
        self.values[2][2] = b33 * inv_det;
        self.values[2][3] = b34 * inv_det;
        self.values[3][0] = b41 * inv_det;
        self.values[3][1] = b42 * inv_det;
        self.values[3][2] = b43 * inv_det;
        self.values[3][3] = b44 * inv_det;

        self
    }
}
