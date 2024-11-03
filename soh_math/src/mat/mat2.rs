//-----------------------------------------------------------------------------
use crate::Vec2;
use num_traits::Float;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
//-----------------------------------------------------------------------------

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mat2<F>(pub [F; 4]);

//-----------------------------------------------------------------------------

impl<F> Mat2<F>
where
    F: Float,
{
    /// Construct a matrix:
    /// ```notrust
    /// | a b |
    /// | c d |
    /// ```
    pub fn new(a: F, b: F, c: F, d: F) -> Self {
        return Mat2([a, b, c, d]);
    }

    /// Get the identity matrix
    pub fn identity() -> Self {
        return Mat2([F::one(), F::zero(), F::zero(), F::one()]);
    }

    /// Construct a rotation matrix for angle `phi`
    pub fn rot(phi: F) -> Self {
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();

        return Mat2([cos_phi, -sin_phi, sin_phi, cos_phi]);
    }

    /// Construct a scaling matrix
    pub fn scale(factor: F) -> Self {
        return Mat2([factor, F::zero(), F::zero(), factor]);
    }

    /// Get matrix determinant
    pub fn det(&self) -> F {
        return self.0[0] * self.0[3] - self.0[1] * self.0[2];
    }

    /// Get the transposed matrix
    pub fn t(&self) -> Self {
        return Mat2([self.0[0], self.0[2], self.0[1], self.0[3]]);
    }

    /// Get an inverse of the `self`
    pub fn invert(&self) -> Self {
        return Mat2([self.0[3], -self.0[1], -self.0[2], self.0[0]]) / self.det();
    }
}

//-----------------------------------------------------------------------------
// Operator overloads
impl<F> std::ops::Add for Mat2<F>
where
    F: Float,
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

impl<F> std::ops::Sub for Mat2<F>
where
    F: Float,
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

impl<F> std::ops::Mul<F> for Mat2<F>
where
    F: Float,
{
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        return Mat2([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
        ]);
    }
}

impl<F> std::ops::Mul<Vec2<F>> for Mat2<F>
where
    F: Float,
{
    type Output = Vec2<F>;

    fn mul(self, rhs: Vec2<F>) -> Self::Output {
        return Vec2 {
            x: self.0[0] * rhs.x + self.0[1] * rhs.y,
            y: self.0[2] * rhs.x + self.0[3] * rhs.y,
        };
    }
}

impl<F> std::ops::Mul for Mat2<F>
where
    F: Float,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return Mat2([
            self.0[0] * rhs.0[0] + self.0[1] * rhs.0[2],
            self.0[0] * rhs.0[1] + self.0[1] * rhs.0[3],
            self.0[2] * rhs.0[0] + self.0[3] * rhs.0[2],
            self.0[2] * rhs.0[1] + self.0[3] * rhs.0[3],
        ]);
    }
}

impl<F> std::ops::Div<F> for Mat2<F>
where
    F: Float,
{
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
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
impl<F> AsRef<[F]> for Mat2<F> {
    fn as_ref(&self) -> &[F] {
        return &self.0;
    }
}

//-----------------------------------------------------------------------------
