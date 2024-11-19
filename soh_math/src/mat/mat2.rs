//-----------------------------------------------------------------------------
use crate::Vec2;
use num_traits::Float;
//-----------------------------------------------------------------------------
/// 2x2 matrix ( column major )
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mat2<T>(pub [T; 4]);

//-----------------------------------------------------------------------------

impl<T> Mat2<T>
where
    T: Copy,
{
    /// Construct a matrix from values ( column major )
    pub const fn new(m: [T; 4]) -> Self {
        return Mat2(m);
    }

    /// Construct a matrix from rows
    pub const fn from_rows(rows: [[T; 2]; 2]) -> Self {
        return Mat2([rows[0][0], rows[1][0], rows[0][1], rows[1][1]]);
    }

    /// Construct a matrix from columns
    pub const fn from_cols(cols: [[T; 2]; 2]) -> Self {
        return Mat2([cols[0][0], cols[0][1], cols[1][0], cols[1][1]]);
    }

    /// Get the element at row `row` and column `col`
    #[inline(always)]
    pub const fn at(&self, row: usize, col: usize) -> T {
        return self.0[col * 2 + row];
    }
}

impl<T> Mat2<T>
where
    T: num_traits::Num + std::ops::Neg<Output = T> + Copy,
{
    /// Get the identity matrix
    pub fn identity() -> Self {
        return Mat2([T::one(), T::zero(), T::zero(), T::one()]);
    }

    /// Construct a scaling matrix
    pub fn scale(factor: T) -> Self {
        return Mat2([factor, T::zero(), T::zero(), factor]);
    }

    /// Get matrix determinant
    pub fn det(&self) -> T {
        return self.0[0] * self.0[3] - self.0[1] * self.0[2];
    }

    /// Get the transposed matrix
    pub const fn t(&self) -> Self {
        return Mat2([self.0[0], self.0[2], self.0[1], self.0[3]]);
    }

    /// Get an inverse of the `self`
    pub fn invert(&self) -> Self {
        return Mat2([self.0[3], -self.0[1], -self.0[2], self.0[0]]) / self.det();
    }
}

impl<T> Mat2<T>
where
    T: Float + std::iter::Sum,
{
    /// Construct a rotation matrix for angle `phi`
    pub fn rot(phi: T) -> Self {
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();

        return Mat2([cos_phi, sin_phi, -sin_phi, cos_phi]);
    }

    /// Get the norm
    pub fn norm(&self) -> T {
        return self.0.iter().map(|&x| x * x).sum::<T>().sqrt();
    }
}

//-----------------------------------------------------------------------------
// Operator overloads
impl<T> std::ops::Add for Mat2<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Mat2([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ]);
    }
}

impl<T> std::ops::Sub for Mat2<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Mat2([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ]);
    }
}

impl<T> std::ops::Mul<T> for Mat2<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        return Mat2([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
        ]);
    }
}

impl<T> std::ops::Mul<Vec2<T>> for Mat2<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        return Vec2 {
            x: self.0[0] * rhs.x + self.0[2] * rhs.y,
            y: self.0[1] * rhs.x + self.0[3] * rhs.y,
        };
    }
}

impl<T> std::ops::Mul for Mat2<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return Mat2([
            self.0[0] * rhs.0[0] + self.0[2] * rhs.0[1],
            self.0[1] * rhs.0[0] + self.0[3] * rhs.0[1],
            self.0[0] * rhs.0[2] + self.0[2] * rhs.0[3],
            self.0[1] * rhs.0[2] + self.0[3] * rhs.0[3],
        ]);
    }
}

impl<T> std::ops::Div<T> for Mat2<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        return Mat2([
            self.0[0] / rhs,
            self.0[1] / rhs,
            self.0[2] / rhs,
            self.0[3] / rhs,
        ]);
    }
}

//-----------------------------------------------------------------------------
// Other
impl<T> AsRef<[T]> for Mat2<T> {
    fn as_ref(&self) -> &[T] {
        return &self.0;
    }
}

//-----------------------------------------------------------------------------
