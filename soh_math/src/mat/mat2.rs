//-----------------------------------------------------------------------------
use crate::Vec2;
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
    pub const fn from_rows(rows: [Vec2<T>; 2]) -> Self {
        return Mat2([
            rows[0].x, rows[1].x,
            rows[0].y, rows[1].y
        ]);
    }

    /// Construct a matrix from columns
    pub const fn from_cols(cols: [Vec2<T>; 2]) -> Self {
        return Mat2([
            cols[0].x, cols[0].y,
            cols[1].x, cols[1].y
        ]);
    }

    /// Get the row
    pub const fn row(&self, row: usize) -> Vec2<T> {
        return Vec2::new(
            self.at(row, 0),
            self.at(row, 1),
        );
    }

    /// Get the column
    pub const fn col(&self, col: usize) -> Vec2<T> {
        return Vec2::new(
            self.at(0, col),
            self.at(1, col),
        );
    }

    /// Get the element at row `row` and column `col`
    /// (zero indexed)
    #[inline(always)]
    pub const fn at(&self, row: usize, col: usize) -> T {
        return self.0[col * 2 + row];
    }

    /// Get mut reference to element at row `row` and column `col`
    /// (zero indexed)
    pub const fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        return &mut self.0[col * 2 + row]
    }
}

impl<T> Mat2<T>
where
    T: num_traits::Num + crate::traits::WholeConsts + std::ops::Neg<Output = T> + Copy,
{
    /// Get the identity matrix
    pub const fn identity() -> Self {
        return Mat2([
            T::ONE,  T::ZERO,
            T::ZERO, T::ONE
        ]);
    }

    /// Construct a scaling matrix
    pub const fn scale(factor: T) -> Self {
        return Mat2([
            factor,  T::ZERO,
            T::ZERO, factor
        ]);
    }

    /// Get matrix determinant
    pub fn det(&self) -> T {
        return self.0[0] * self.0[3] - self.0[1] * self.0[2];
    }

    /// Get the transposed matrix
    pub const fn t(&self) -> Self {
        return Mat2([
            self.0[0], self.0[2],
            self.0[1], self.0[3]
        ]);
    }

    /// Get an inverse of the `self`
    pub fn invert(&self) -> Self {
        return self.invert_no_det() / self.det();
    }

    /// Get an inverse of `self` (but no devision by determinant)
    pub fn invert_no_det(&self) -> Self {
        return Mat2([
            self.0[3], -self.0[1],
            -self.0[2], self.0[0]
        ]);
    }
}

impl<T> Mat2<T>
where
    T: num_traits::Float + std::iter::Sum,
{
    /// Construct a rotation matrix for angle `phi`
    pub fn rot(phi: T) -> Self {
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();

        return Mat2([
             cos_phi, sin_phi,
            -sin_phi, cos_phi
        ]);
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

impl<T> std::ops::Neg for Mat2<T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Mat2([
            -self.0[0],
            -self.0[1],
            -self.0[2],
            -self.0[3],
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
