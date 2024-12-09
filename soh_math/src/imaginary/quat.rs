//-----------------------------------------------------------------------------
use crate::traits::{RealConsts, WholeConsts};
use crate::Vec3;
//-----------------------------------------------------------------------------

#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quaternion<T> {
    pub scalar: T,
    pub vector: Vec3<T>,
}

//-----------------------------------------------------------------------------
// Constructors
impl<T> Quaternion<T> {
    pub const fn new(scalar: T, vector: Vec3<T>) -> Self {
        return Quaternion { scalar, vector };
    }
}

impl<T> WholeConsts for Quaternion<T>
where
    T: WholeConsts,
{
    const ZERO: Self = Quaternion {
        scalar: T::ZERO,
        vector: Vec3::ZERO,
    };

    const ONE: Self = Quaternion {
        scalar: T::ONE,
        vector: Vec3::ZERO,
    };

    const TWO: Self = Quaternion {
        scalar: T::TWO,
        vector: Vec3::ZERO,
    };
}

impl<T> Quaternion<T>
where
    T: WholeConsts,
{
    pub const fn zero() -> Self {
        return Self::ZERO;
    }

    pub const fn one() -> Self {
        return Self::ONE;
    }

    pub const fn two() -> Self {
        return Self::TWO;
    }
}

impl<T> Quaternion<T>
where
    T: num_traits::Float + RealConsts,
{
    /// Create a unit quaternion from rotation axis and angle
    pub fn from_axis_angle(axis: Vec3<T>, angle: T) -> Self {
        let half_angle = angle * T::ONE_HALF;

        let cos = half_angle.cos();
        let sin = half_angle.sin();

        return Self::new(cos, axis.normalized() * sin);
    }
}

//-----------------------------------------------------------------------------
// Math functions
impl<T> Quaternion<T>
where
    T: Copy,
{
    /// Scalar part
    pub const fn scalar(&self) -> T {
        return self.scalar;
    }

    /// Vector part
    pub const fn vector(&self) -> Vec3<T> {
        return self.vector;
    }
}

impl<T> Quaternion<T>
where
    T: num_traits::Num + Copy,
{
    /// Calculate the squared length
    pub fn len2(&self) -> T {
        return self.scalar * self.scalar + self.vector.len2();
    }
}

impl<T> Quaternion<T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    /// Get the conjugate
    pub fn conjugate(&self) -> Self {
        return Quaternion {
            scalar: self.scalar,
            vector: -self.vector,
        };
    }
}

impl<T> Quaternion<T>
where
    T: num_traits::Num + std::ops::Neg<Output = T> + Copy,
{
    /// Rotate a `point` using the quaternion
    pub fn rotate(&self, point: Vec3<T>) -> Vec3<T> {
        let p = Quaternion {
            scalar: T::zero(),
            vector: point,
        };

        return (*self * p * self.conjugate()).vector;
    }
}

impl<T> Quaternion<T>
where
    T: num_traits::Float + WholeConsts + RealConsts + Copy,
{
    /// Calculate the length (absolute value)
    pub fn len(&self) -> T {
        return self.len2().sqrt();
    }

    /// Calculate the exponential
    pub fn exp(&self) -> Self {
        let len_v = self.vector.len();
        let exp_s = self.scalar.exp();

        if len_v < T::epsilon() {
            return Self::new(self.scalar.exp(), Vec3::ZERO);
        }

        return Self::new(
            exp_s * len_v.cos(),
            self.vector * exp_s * len_v.sin() / len_v,
        );
    }

    /// Calculate the natural logarithm
    pub fn ln(&self) -> Self {
        let len_q = self.len();
        let len_v = self.vector.len();
        let ln_len = len_q.ln();

        return Self::new(ln_len, self.vector * (self.scalar / len_q).acos() / len_v);
    }

    /// Calculate the natural logarithm of the length
    pub fn ln_len(&self) -> T {
        return self.len2().ln() * T::ONE_HALF;
    }

    /// Calculate rotation axis and angle from a unit quaternion
    pub fn get_axis_angle(&self) -> (Vec3<T>, T) {
        let cos = self.scalar;
        let sin = self.vector.len();

        return (self.vector / sin, T::atan2(sin, cos));
    }

    /// Calculate the inverse
    pub fn invert(&self) -> Self {
        return self.conjugate() / self.len2();
    }
}

//-----------------------------------------------------------------------------
// Operator overloads
impl<T> std::ops::Add for Quaternion<T>
where
    T: num_traits::Num,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Quaternion {
            scalar: self.scalar + rhs.scalar,
            vector: self.vector + rhs.vector,
        };
    }
}

impl<T> std::ops::AddAssign for Quaternion<T>
where
    T: std::ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.scalar += rhs.scalar;
        self.vector += rhs.vector;
    }
}

impl<T> std::ops::Sub for Quaternion<T>
where
    T: num_traits::Num,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Quaternion {
            scalar: self.scalar - rhs.scalar,
            vector: self.vector - rhs.vector,
        };
    }
}

impl<T> std::ops::SubAssign for Quaternion<T>
where
    T: std::ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.scalar -= rhs.scalar;
        self.vector -= rhs.vector;
    }
}

impl<T> std::ops::Mul<T> for Quaternion<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        return Quaternion {
            scalar: self.scalar * rhs,
            vector: self.vector * rhs,
        };
    }
}

impl<T> std::ops::MulAssign<T> for Quaternion<T>
where
    T: std::ops::MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.scalar *= rhs;
        self.vector *= rhs;
    }
}

impl<T> std::ops::Mul for Quaternion<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return Quaternion {
            scalar: self.scalar * rhs.scalar - Vec3::dot(&self.vector, &rhs.vector),
            vector: self.vector * rhs.scalar
                + rhs.vector * self.scalar
                + Vec3::cross(&self.vector, &rhs.vector),
        };
    }
}

impl<T> std::ops::MulAssign for Quaternion<T>
where
    T: num_traits::Num + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T> std::ops::Div<T> for Quaternion<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        return Quaternion {
            scalar: self.scalar / rhs,
            vector: self.vector / rhs,
        };
    }
}

impl<T> std::ops::DivAssign<T> for Quaternion<T>
where
    T: std::ops::DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.scalar /= rhs;
        self.vector /= rhs;
    }
}

impl<T> std::ops::Div for Quaternion<T>
where
    T: num_traits::Num + std::ops::Neg<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        return self * rhs.conjugate() / rhs.len2();
    }
}

impl<T> std::ops::DivAssign for Quaternion<T>
where
    T: num_traits::Num + std::ops::Neg<Output = T> + Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T> std::ops::Neg for Quaternion<T>
where
    T: std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Quaternion {
            scalar: -self.scalar,
            vector: -self.vector,
        };
    }
}

//-----------------------------------------------------------------------------
// From implementations
impl<S, D> crate::Convert<Quaternion<D>> for Quaternion<S>
where
    S: Copy,
    D: Copy + From<S>,
{
    fn convert(&self) -> Quaternion<D> {
        return Quaternion {
            scalar: self.scalar.into(),
            vector: self.vector.convert(),
        };
    }
}

// Convert complex number to quaternion
impl<T> From<super::Complex<T>> for Quaternion<T>
where
    T: WholeConsts + Copy,
{
    fn from(value: super::Complex<T>) -> Self {
        return Quaternion {
            scalar: value.re,
            vector: Vec3::new(value.im, T::ZERO, T::ZERO),
        };
    }
}

// Convert scalar to quaternion
impl<T> From<T> for Quaternion<T>
where
    T: WholeConsts,
{
    fn from(value: T) -> Self {
        return Quaternion {
            scalar: value,
            vector: Vec3::zero(),
        };
    }
}

//-----------------------------------------------------------------------------

impl<T> std::fmt::Display for Quaternion<T>
where
    T: num_traits::Num + PartialOrd + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sx = if self.vector.x >= T::zero() { '+' } else { '-' };
        let sy = if self.vector.y >= T::zero() { '+' } else { '-' };
        let sz = if self.vector.z >= T::zero() { '+' } else { '-' };

        return write!(
            f,
            "{scalar} {sx} {vec_x}i {sy} {vec_y}j {sz} {vec_z}k",
            scalar = self.scalar,
            vec_x = self.vector.x,
            vec_y = self.vector.y,
            vec_z = self.vector.z
        );
    }
}

//-----------------------------------------------------------------------------
