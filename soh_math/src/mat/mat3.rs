//-----------------------------------------------------------------------------
use crate::Vec3;
use num_traits::Float;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
//-----------------------------------------------------------------------------

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mat3<F>(pub [F; 9]);

//-----------------------------------------------------------------------------

impl<F> Mat3<F>
where
    F: Float,
{
    /// Construct a matrix:
    /// ```
    /// | a b c |
    /// | d e f |
    /// | g h i |
    /// ```
    pub fn new(a: F, b: F, c: F, d: F, e: F, f: F, g: F, h: F, i: F) -> Self {
        return Mat3([a, b, c, d, e, f, g, h, i]);
    }

    /// Get the identity matrix
    pub fn identity() -> Self {
        return Mat3([
            F::one(),
            F::zero(),
            F::zero(),
            F::zero(),
            F::one(),
            F::zero(),
            F::zero(),
            F::zero(),
            F::one(),
        ]);
    }

    /// Construct a scaling matrix
    pub fn scale(factor: F) -> Self {
        return Mat3([
            factor,
            F::zero(),
            F::zero(),
            F::zero(),
            factor,
            F::zero(),
            F::zero(),
            F::zero(),
            factor,
        ]);
    }

    /// Get matrix determinant
    pub fn det(&self) -> F {
        return self.0[0] * (self.0[4] * self.0[8] - self.0[5] * self.0[7])
            - self.0[1] * (self.0[3] * self.0[8] - self.0[5] * self.0[6])
            + self.0[2] * (self.0[3] * self.0[7] - self.0[4] * self.0[6]);
    }

    /// Get the transposed matrix
    pub fn t(&self) -> Self {
        // rustfmt is crazy
        return Mat3([
            self.0[0], self.0[3], self.0[6], self.0[1], self.0[4], self.0[7], self.0[2], self.0[5],
            self.0[8],
        ]);
    }

    /// Get an inverse of the `self`
    pub fn invert(&self) -> Self {
        return Mat3([
            // First row
            (self.0[4] * self.0[8] - self.0[5] * self.0[7]),
            (self.0[2] * self.0[7] - self.0[1] * self.0[8]),
            (self.0[1] * self.0[5] - self.0[2] * self.0[4]),
            // Second row
            (self.0[5] * self.0[6] - self.0[3] * self.0[8]),
            (self.0[0] * self.0[8] - self.0[2] * self.0[6]),
            (self.0[2] * self.0[3] - self.0[0] * self.0[5]),
            // Third row
            (self.0[3] * self.0[7] - self.0[4] * self.0[6]),
            (self.0[1] * self.0[6] - self.0[0] * self.0[7]),
            (self.0[0] * self.0[4] - self.0[1] * self.0[3]),
        ]) / self.det();
    }
}

//-----------------------------------------------------------------------------
// Operator overloads
impl<F> std::ops::Add for Mat3<F>
where
    F: Float,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Mat3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
            self.0[4] + rhs.0[4],
            self.0[5] + rhs.0[5],
            self.0[6] + rhs.0[6],
            self.0[7] + rhs.0[7],
            self.0[8] + rhs.0[8],
        ]);
    }
}

impl<F> std::ops::Sub for Mat3<F>
where
    F: Float,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Mat3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
            self.0[4] - rhs.0[4],
            self.0[5] - rhs.0[5],
            self.0[6] - rhs.0[6],
            self.0[7] - rhs.0[7],
            self.0[8] - rhs.0[8],
        ]);
    }
}

impl<F> std::ops::Mul<F> for Mat3<F>
where
    F: Float,
{
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        return Mat3([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
            self.0[4] * rhs,
            self.0[5] * rhs,
            self.0[6] * rhs,
            self.0[7] * rhs,
            self.0[8] * rhs,
        ]);
    }
}

impl<F> std::ops::Mul<Vec3<F>> for Mat3<F>
where
    F: Float,
{
    type Output = Vec3<F>;

    fn mul(self, rhs: Vec3<F>) -> Self::Output {
        return Vec3 {
            x: self.0[0] * rhs.x + self.0[3] * rhs.y + self.0[6] * rhs.z,
            y: self.0[1] * rhs.x + self.0[4] * rhs.y + self.0[7] * rhs.z,
            z: self.0[2] * rhs.x + self.0[5] * rhs.y + self.0[8] * rhs.z,
        };
    }
}

impl<F> std::ops::Mul for Mat3<F>
where
    F: Float,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return Mat3([
            // First row
            self.0[0] * rhs.0[0] + self.0[1] * rhs.0[3] + self.0[2] * rhs.0[6],
            self.0[0] * rhs.0[1] + self.0[1] * rhs.0[4] + self.0[2] * rhs.0[7],
            self.0[0] * rhs.0[2] + self.0[1] * rhs.0[5] + self.0[2] * rhs.0[8],
            // Second row
            self.0[3] * rhs.0[0] + self.0[4] * rhs.0[3] + self.0[5] * rhs.0[6],
            self.0[3] * rhs.0[1] + self.0[4] * rhs.0[4] + self.0[5] * rhs.0[7],
            self.0[3] * rhs.0[2] + self.0[4] * rhs.0[5] + self.0[5] * rhs.0[8],
            // Third row
            self.0[6] * rhs.0[0] + self.0[7] * rhs.0[3] + self.0[8] * rhs.0[6],
            self.0[6] * rhs.0[1] + self.0[7] * rhs.0[4] + self.0[8] * rhs.0[7],
            self.0[6] * rhs.0[2] + self.0[7] * rhs.0[5] + self.0[8] * rhs.0[8],
        ]);
    }
}

impl<F> std::ops::Div<F> for Mat3<F>
where
    F: Float,
{
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        return Mat3([
            self.0[0] / rhs,
            self.0[1] / rhs,
            self.0[2] / rhs,
            self.0[3] / rhs,
            self.0[4] / rhs,
            self.0[5] / rhs,
            self.0[6] / rhs,
            self.0[7] / rhs,
            self.0[8] / rhs,
        ]);
    }
}

//-----------------------------------------------------------------------------
