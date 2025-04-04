//-----------------------------------------------------------------------------
use crate::Vec3;
//-----------------------------------------------------------------------------
/// 3x3 matrix ( column major )
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mat3<T>(pub [T; 9]);

//-----------------------------------------------------------------------------

impl<T> Mat3<T>
where
    T: Copy,
{
    /// Construct a matrix from values ( column major )
    pub const fn new(m: [T; 9]) -> Self {
        return Mat3(m);
    }

    /// Construct a matrix from rows
    pub const fn from_rows(rows: [Vec3<T>; 3]) -> Self {
        return Mat3([
            rows[0].x, rows[1].x, rows[2].x,
            rows[0].y, rows[1].y, rows[2].y,
            rows[0].z, rows[1].z, rows[2].z,
        ]);
    }

    /// Construct a matrix from columns
    pub const fn from_cols(cols: [Vec3<T>; 3]) -> Self {
        return Mat3([
            cols[0].x, cols[0].y, cols[0].z,
            cols[1].x, cols[1].y, cols[1].z,
            cols[2].x, cols[2].y, cols[2].z,
        ]);
    }

    /// Get the row
    pub const fn row(&self, row: usize) -> Vec3<T> {
        return Vec3::new(
            self.at(row, 0),
            self.at(row, 1),
            self.at(row, 2),
        );
    }

    /// Get the column
    pub const fn col(&self, col: usize) -> Vec3<T> {
        return Vec3::new(
            self.at(0, col),
            self.at(1, col),
            self.at(2, col),
        );
    }

    /// Get the element at row `row` and column `col`
    #[inline(always)]
    pub const fn at(&self, row: usize, col: usize) -> T {
        return self.0[col * 3 + row];
    }

    /// Get mut reference to element at row `row` and column `col`
    /// (zero indexed)
    pub const fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        return &mut self.0[col * 3 + row]
    }
}

impl<T> Mat3<T>
where
    T: num_traits::Num + crate::traits::WholeConsts + std::ops::Neg<Output = T> + Copy,
{
    /// Get the identity matrix
    pub const fn identity() -> Self {
        return Mat3([
            T::ONE,  T::ZERO, T::ZERO,
            T::ZERO, T::ONE,  T::ZERO,
            T::ZERO, T::ZERO, T::ONE,
        ]);
    }

    /// Construct a scaling matrix
    pub const fn scale(factor: T) -> Self {
        return Mat3([
            factor,  T::ZERO, T::ZERO,
            T::ZERO, factor,  T::ZERO,
            T::ZERO, T::ZERO, factor,
        ]);
    }

    /// Get matrix determinant
    pub fn det(&self) -> T {
        return self.0[0] * (self.0[4] * self.0[8] - self.0[7] * self.0[5])
             + self.0[3] * (self.0[7] * self.0[2] - self.0[1] * self.0[8])
             + self.0[6] * (self.0[1] * self.0[5] - self.0[4] * self.0[2]);
    }

    /// Get the transposed matrix
    pub const fn t(&self) -> Self {
        return Mat3([
            self.0[0], self.0[3], self.0[6],
            self.0[1], self.0[4], self.0[7],
            self.0[2], self.0[5], self.0[8],
        ]);
    }

    /// Get an inverse of the `self`
    pub fn invert(&self) -> Self {
        let inv = self.invert_no_det();

        let det = self.0[0] * inv.0[0]
                + self.0[3] * inv.0[1]
                + self.0[6] * inv.0[2];

        return inv / det;
    }

    /// Get an inverse of `self` (but no devision by determinant)
    pub fn invert_no_det(&self) -> Self {
        return Mat3([
            // First column
            (self.0[4] * self.0[8] - self.0[7] * self.0[5]),
            (self.0[7] * self.0[2] - self.0[1] * self.0[8]),
            (self.0[1] * self.0[5] - self.0[4] * self.0[2]),
            // Second column
            (self.0[6] * self.0[5] - self.0[3] * self.0[8]),
            (self.0[0] * self.0[8] - self.0[6] * self.0[2]),
            (self.0[3] * self.0[2] - self.0[0] * self.0[5]),
            // Third column
            (self.0[3] * self.0[7] - self.0[6] * self.0[4]),
            (self.0[6] * self.0[1] - self.0[0] * self.0[7]),
            (self.0[0] * self.0[4] - self.0[3] * self.0[1]),
        ]);
    }
}

impl<T> Mat3<T>
where
    T: num_traits::Float + std::iter::Sum + From<f32>,
{
    /// Get a rotation matrix for yaw `phi`
    /// ( Rotation around the z-axis )
    pub fn yaw(phi: T) -> Self {
        let phi_cos = phi.cos();
        let phi_sin = phi.sin();

        return Mat3([
             phi_cos,   phi_sin,   T::zero(),
            -phi_sin,   phi_cos,   T::zero(),
             T::zero(), T::zero(), T::one(),
        ]);
    }

    /// Get a rotation matrix for pitch `theta`
    /// ( Rotation around the y-axis )
    pub fn pitch(theta: T) -> Self {
        let theta_cos = theta.cos();
        let theta_sin = theta.sin();

        return Mat3([
            theta_cos, T::zero(), -theta_sin,
            T::zero(), T::one(),   T::zero(),
            theta_sin, T::zero(),  theta_cos,
        ]);
    }

    /// Get a rotation matrix for roll `psi`
    /// ( Rotation around the x-axis )
    pub fn roll(psi: T) -> Self {
        let psi_cos = psi.cos();
        let psi_sin = psi.sin();

        return Mat3([
            T::one(),   T::zero(), T::zero(),
            T::zero(),  psi_cos,   psi_sin,
            T::zero(), -psi_sin,   psi_cos,
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
            // First column
            yaw_cos * pitch_cos,
            yaw_sin * pitch_cos,
            -pitch_sin,
            // Second column
            yaw_cos * pitch_sin * roll_sin - yaw_sin * roll_cos,
            yaw_sin * pitch_sin * roll_sin + yaw_cos * roll_cos,
            pitch_cos * roll_sin,
            // Third column
            yaw_cos * pitch_sin * roll_cos + yaw_sin * roll_sin,
            yaw_sin * pitch_sin * roll_cos - yaw_cos * roll_sin,
            pitch_cos * roll_cos,
        ]);
    }

    /// Get euler angles ( yaw, pitch, roll )
    ///
    /// source:
    /// <https://learnopencv.com/rotation-matrix-to-euler-angles/>
    pub fn get_euler_angles(&self) -> (T, T, T) {
        let sy = T::hypot(self.at(0, 0), self.at(1, 0));

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

    /// Create a rotation matrix from rotation axis and angle
    ///
    /// source:
    /// <https://songho.ca/opengl/gl_rotate.html>
    pub fn from_axis_angle(axis: Vec3<T>, angle: T) -> Self {
        // Angle related values
        let cos = angle.cos();
        let sin = angle.sin();

        let one_minus_cos = T::one() - cos;

        // Unpack axis
        let Vec3 { x, y, z } = axis.normalized();

        let xx = x * x;
        let xy = x * y;
        let xz = x * z;

        let yy = y * y;
        let yz = y * z;

        let zz = z * z;

        return Mat3([
            // First column
            one_minus_cos * xx + cos,
            one_minus_cos * xy + sin * z,
            one_minus_cos * xz - sin * y,
            // Second column
            one_minus_cos * xy - sin * z,
            one_minus_cos * yy + cos,
            one_minus_cos * yz + sin * x,
            // Third column
            one_minus_cos * xz + sin * y,
            one_minus_cos * yz - sin * x,
            one_minus_cos * zz + cos,
        ]);
    }

    /// Create a rotation matrix from a unit quaternion
    ///
    /// source:
    /// <https://songho.ca/opengl/gl_quaternion.html>
    pub fn from_quat(quat: crate::Quaternion<T>) -> Self {
        // Unpack quaternion
        let crate::Quaternion {
            scalar: s,
            vector: Vec3 { x, y, z },
        } = quat;

        let sx = s * x;
        let sy = s * y;
        let sz = s * z;

        let xx = x * x;
        let xy = x * y;
        let xz = x * z;

        let yy = y * y;
        let yz = y * z;

        let zz = z * z;

        let one = T::one();
        let two = one + one;

        return Mat3([
            // First column
            one - two*yy - two*zz,
            two*xy + two*sz,
            two*xz - two*sy,
            // Second column
            two*xy - two*sz,
            one - two*xx - two*zz,
            two*yz + two*sx,
            // Third column
            two*xz + two*sy,
            two*yz - two*sx,
            one - two*xx - two*yy,
        ]);
    }

    /// LookAt matrix:
    ///
    /// The direction from `pos` to `target` becomes the Z direction
    /// Orthogonalized `up` becomes Y direction
    /// (Y x Z) becomes X direction
    pub fn look_at(pos: Vec3<T>, target: Vec3<T>, up: Vec3<T>) -> Self {
        let z = (target - pos).normalized();
        let x = Vec3::cross(&up, &z).normalized();
        let y = Vec3::cross(&z, &x);

        return Self::from_cols([x, y, z]);
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
    T: num_traits::Num + Copy,
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
    T: num_traits::Num + Copy,
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
    T: num_traits::Num + Copy,
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
    T: num_traits::Num + Copy,
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
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return Mat3([
            // First column
            self.0[0] * rhs.0[0] + self.0[3] * rhs.0[1] + self.0[6] * rhs.0[2],
            self.0[1] * rhs.0[0] + self.0[4] * rhs.0[1] + self.0[7] * rhs.0[2],
            self.0[2] * rhs.0[0] + self.0[5] * rhs.0[1] + self.0[8] * rhs.0[2],
            // Second column
            self.0[0] * rhs.0[3] + self.0[3] * rhs.0[4] + self.0[6] * rhs.0[5],
            self.0[1] * rhs.0[3] + self.0[4] * rhs.0[4] + self.0[7] * rhs.0[5],
            self.0[2] * rhs.0[3] + self.0[5] * rhs.0[4] + self.0[8] * rhs.0[5],
            // Third column
            self.0[0] * rhs.0[6] + self.0[3] * rhs.0[7] + self.0[6] * rhs.0[8],
            self.0[1] * rhs.0[6] + self.0[4] * rhs.0[7] + self.0[7] * rhs.0[8],
            self.0[2] * rhs.0[6] + self.0[5] * rhs.0[7] + self.0[8] * rhs.0[8],
        ]);
    }
}

impl<T> std::ops::Div<T> for Mat3<T>
where
    T: num_traits::Num + Copy,
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

impl<T> std::ops::Neg for Mat3<T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Mat3([
            -self.0[0],
            -self.0[1],
            -self.0[2],
            -self.0[3],
            -self.0[4],
            -self.0[5],
            -self.0[6],
            -self.0[7],
            -self.0[8],
        ]);
    }
}

//-----------------------------------------------------------------------------
