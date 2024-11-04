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
    F: Float + std::iter::Sum + From<f32>,
{
    /// Construct a matrix:
    /// ```notrust
    /// | a b c |
    /// | d e f |
    /// | g h i |
    /// ```
    pub fn new(m: [F; 9]) -> Self {
        return Mat3(m);
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

    /// Get a rotation matrix for yaw `phi`
    /// ( Rotation around the z-axis )
    pub fn yaw(phi: F) -> Self {
        let phi_cos = phi.cos();
        let phi_sin = phi.sin();

        return Mat3([
            phi_cos,
            -phi_sin,
            F::zero(),
            phi_sin,
            phi_cos,
            F::zero(),
            F::zero(),
            F::zero(),
            F::one(),
        ]);
    }

    /// Get a rotation matrix for pitch `theta`
    /// ( Rotation around the y-axis )
    pub fn pitch(theta: F) -> Self {
        let theta_cos = theta.cos();
        let theta_sin = theta.sin();

        return Mat3([
            theta_cos,
            F::zero(),
            theta_sin,
            F::zero(),
            F::one(),
            F::zero(),
            -theta_sin,
            F::zero(),
            theta_cos,
        ]);
    }

    /// Get a rotation matrix for roll `psi`
    /// ( Rotation around the x-axis )
    pub fn roll(psi: F) -> Self {
        let psi_cos = psi.cos();
        let psi_sin = psi.sin();

        return Mat3([
            F::one(),
            F::zero(),
            F::zero(),
            F::zero(),
            psi_cos,
            -psi_sin,
            F::zero(),
            psi_sin,
            psi_cos,
        ]);
    }

    /// Get a rotation matrix for euler angles yaw pitch and roll.
    /// Identical to multiplying yaw * pitch * roll matrices separately
    /// ( First rotating around x-axis, then rotating around y-axis and finally around z-axis )
    pub fn yaw_pitch_roll(yaw: F, pitch: F, roll: F) -> Self {
        let yaw_cos = yaw.cos();
        let yaw_sin = yaw.sin();
        let pitch_cos = pitch.cos();
        let pitch_sin = pitch.sin();
        let roll_cos = roll.cos();
        let roll_sin = roll.sin();

        return Mat3([
            yaw_cos * pitch_cos,
            yaw_cos * pitch_sin * roll_sin - yaw_sin * roll_cos,
            yaw_cos * pitch_sin * roll_cos + yaw_sin * roll_sin,
            yaw_sin * pitch_cos,
            yaw_sin * pitch_sin * roll_sin + yaw_cos * roll_cos,
            yaw_sin * pitch_sin * roll_cos - yaw_cos * roll_sin,
            -pitch_sin,
            pitch_cos * roll_sin,
            pitch_cos * roll_cos,
        ]);
    }

    /// Get euler angles ( yaw, pitch, roll )
    pub fn get_euler_angles(&self) -> (F, F, F) {
        let sy = F::sqrt(self.0[0].powi(2) + self.0[3].powi(2));

        let singular = sy < 1.0e-6.into();

        if !singular {
            return (
                F::atan2(self.0[3], self.0[0]),
                F::atan2(-self.0[6], sy),
                F::atan2(self.0[7], self.0[8]),
            );
        } else {
            return (
                F::zero(),
                F::atan2(-self.0[6], sy),
                F::atan2(-self.0[5], self.0[4]),
            );
        }
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

    /// Get the norm
    pub fn norm(&self) -> F {
        return self.0.iter().map(|&x| x * x).sum::<F>().sqrt();
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
