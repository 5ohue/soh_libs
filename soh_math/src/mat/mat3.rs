//-----------------------------------------------------------------------------
use crate::Vec3;
use num_traits::Float;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
//-----------------------------------------------------------------------------
/// 3x3 matrix ( column major )
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mat3<T>(pub [T; 9]);

//-----------------------------------------------------------------------------

impl<T> Mat3<T>
where
    T: Float + std::iter::Sum + From<f32>,
{
    /// Construct a matrix from values ( column major )
    pub fn new(m: [T; 9]) -> Self {
        return Mat3(m);
    }

    /// Construct a matrix from rows
    pub fn from_rows(rows: [[T; 3]; 3]) -> Self {
        return Mat3([
            rows[0][0], rows[1][0], rows[2][0], rows[0][1], rows[1][1], rows[2][1], rows[0][2],
            rows[1][2], rows[2][2],
        ]);
    }

    /// Construct a matrix from columns
    pub fn from_cols(cols: [[T; 3]; 3]) -> Self {
        return Mat3([
            cols[0][0], cols[0][1], cols[0][2], cols[1][0], cols[1][1], cols[1][2], cols[2][0],
            cols[2][1], cols[2][2],
        ]);
    }

    /// Get the identity matrix
    pub fn identity() -> Self {
        return Mat3([
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        ]);
    }

    /// Get a rotation matrix for yaw `phi`
    /// ( Rotation around the z-axis )
    pub fn yaw(phi: T) -> Self {
        let phi_cos = phi.cos();
        let phi_sin = phi.sin();

        return Mat3([
            phi_cos,
            phi_sin,
            T::zero(),
            -phi_sin,
            phi_cos,
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        ]);
    }

    /// Get a rotation matrix for pitch `theta`
    /// ( Rotation around the y-axis )
    pub fn pitch(theta: T) -> Self {
        let theta_cos = theta.cos();
        let theta_sin = theta.sin();

        return Mat3([
            theta_cos,
            T::zero(),
            -theta_sin,
            T::zero(),
            T::one(),
            T::zero(),
            theta_sin,
            T::zero(),
            theta_cos,
        ]);
    }

    /// Get a rotation matrix for roll `psi`
    /// ( Rotation around the x-axis )
    pub fn roll(psi: T) -> Self {
        let psi_cos = psi.cos();
        let psi_sin = psi.sin();

        return Mat3([
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            psi_cos,
            psi_sin,
            T::zero(),
            -psi_sin,
            psi_cos,
        ]);
    }

    /// Get a rotation matrix for euler angles yaw pitch and roll.
    /// Identical to multiplying yaw * pitch * roll matrices separately
    /// ( First rotating around x-axis, then rotating around y-axis and finally around z-axis )
    pub fn yaw_pitch_roll(yaw: T, pitch: T, roll: T) -> Self {
        let yaw_cos = yaw.cos();
        let yaw_sin = yaw.sin();
        let pitch_cos = pitch.cos();
        let pitch_sin = pitch.sin();
        let roll_cos = roll.cos();
        let roll_sin = roll.sin();

        return Mat3([
            yaw_cos * pitch_cos,
            yaw_sin * pitch_cos,
            -pitch_sin,
            yaw_cos * pitch_sin * roll_sin - yaw_sin * roll_cos,
            yaw_sin * pitch_sin * roll_sin + yaw_cos * roll_cos,
            pitch_cos * roll_sin,
            yaw_cos * pitch_sin * roll_cos + yaw_sin * roll_sin,
            yaw_sin * pitch_sin * roll_cos - yaw_cos * roll_sin,
            pitch_cos * roll_cos,
        ]);
    }

    /// Construct a scaling matrix
    pub fn scale(factor: T) -> Self {
        return Mat3([
            factor,
            T::zero(),
            T::zero(),
            T::zero(),
            factor,
            T::zero(),
            T::zero(),
            T::zero(),
            factor,
        ]);
    }

    /// Get the element at row `row` and column `col`
    #[inline(always)]
    pub fn at(&self, row: usize, col: usize) -> T {
        return self.0[col * 3 + row];
    }

    /// Get euler angles ( yaw, pitch, roll )
    pub fn get_euler_angles(&self) -> (T, T, T) {
        let sy = T::sqrt(self.at(0, 0).powi(2) + self.at(1, 0).powi(2));

        let singular = sy < 1.0e-6.into();

        if !singular {
            return (
                T::atan2(self.at(1, 0), self.at(0, 0)),
                T::atan2(-self.at(2, 0), sy),
                T::atan2(self.at(2, 1), self.at(2, 2)),
            );
        } else {
            return (
                T::zero(),
                T::atan2(-self.at(2, 0), sy),
                T::atan2(-self.at(1, 2), self.at(1, 1)),
            );
        }
    }

    /// Get matrix determinant
    pub fn det(&self) -> T {
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

    /// Get the norm
    pub fn norm(&self) -> T {
        return self.0.iter().map(|&x| x * x).sum::<T>().sqrt();
    }
}

//-----------------------------------------------------------------------------
// Operator overloads
impl<T> std::ops::Add for Mat3<T>
where
    T: Float,
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

impl<T> std::ops::Sub for Mat3<T>
where
    T: Float,
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

impl<T> std::ops::Mul<T> for Mat3<T>
where
    T: Float,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
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

impl<T> std::ops::Mul<Vec3<T>> for Mat3<T>
where
    T: Float,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        return Vec3 {
            x: self.0[0] * rhs.x + self.0[3] * rhs.y + self.0[6] * rhs.z,
            y: self.0[1] * rhs.x + self.0[4] * rhs.y + self.0[7] * rhs.z,
            z: self.0[2] * rhs.x + self.0[5] * rhs.y + self.0[8] * rhs.z,
        };
    }
}

impl<T> std::ops::Mul for Mat3<T>
where
    T: Float,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return Mat3([
            // First row
            self.0[0] * rhs.0[0] + self.0[3] * rhs.0[1] + self.0[6] * rhs.0[2],
            self.0[1] * rhs.0[0] + self.0[4] * rhs.0[1] + self.0[7] * rhs.0[2],
            self.0[2] * rhs.0[0] + self.0[5] * rhs.0[1] + self.0[8] * rhs.0[2],
            // Second row
            self.0[0] * rhs.0[3] + self.0[3] * rhs.0[4] + self.0[6] * rhs.0[5],
            self.0[1] * rhs.0[3] + self.0[4] * rhs.0[4] + self.0[7] * rhs.0[5],
            self.0[2] * rhs.0[3] + self.0[5] * rhs.0[4] + self.0[8] * rhs.0[5],
            // Third row
            self.0[0] * rhs.0[6] + self.0[3] * rhs.0[7] + self.0[6] * rhs.0[8],
            self.0[1] * rhs.0[6] + self.0[4] * rhs.0[7] + self.0[7] * rhs.0[8],
            self.0[2] * rhs.0[6] + self.0[5] * rhs.0[7] + self.0[8] * rhs.0[8],
        ]);
    }
}

impl<T> std::ops::Div<T> for Mat3<T>
where
    T: Float,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
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
