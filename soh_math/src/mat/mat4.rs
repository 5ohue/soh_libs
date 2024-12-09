//-----------------------------------------------------------------------------
use crate::Vec4;
//-----------------------------------------------------------------------------
/// 4x4 matrix ( column major )
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mat4<T>(pub [T; 16]);

//-----------------------------------------------------------------------------

impl<T> Mat4<T>
where
    T: Copy,
{
    /// Construct a matrix from values ( column major )
    pub const fn new(m: [T; 16]) -> Self {
        return Mat4(m);
    }

    /// Construct a matrix from rows
    pub const fn from_rows(rows: [Vec4<T>; 4]) -> Self {
        return Mat4([
            rows[0].x, rows[1].x, rows[2].x, rows[3].x,
            rows[0].y, rows[1].y, rows[2].y, rows[3].y,
            rows[0].z, rows[1].z, rows[2].z, rows[3].z,
            rows[0].w, rows[1].w, rows[2].w, rows[3].w,
        ]);
    }

    /// Construct a matrix from columns
    pub const fn from_cols(cols: [Vec4<T>; 4]) -> Self {
        return Mat4([
            cols[0].x, cols[0].y, cols[0].z, cols[0].w,
            cols[1].x, cols[1].y, cols[1].z, cols[1].w,
            cols[2].x, cols[2].y, cols[2].z, cols[2].w,
            cols[3].x, cols[3].y, cols[3].z, cols[3].w,
        ]);
    }

    /// Get the row
    pub const fn row(&self, row: usize) -> Vec4<T> {
        return Vec4::new(
            self.at(row, 0),
            self.at(row, 1),
            self.at(row, 2),
            self.at(row, 3),
        );
    }

    /// Get the column
    pub const fn col(&self, col: usize) -> Vec4<T> {
        return Vec4::new(
            self.at(0, col),
            self.at(1, col),
            self.at(2, col),
            self.at(3, col),
        );
    }

    /// Construct a 3x3 matrix and a vector from 4x4 matrix. 4x4 matrix looks like:
    ///
    /// | m_11 m_12 m_13 v.x |
    /// | m_21 m_22 m_23 v.y |
    /// | m_31 m_32 m_33 v.z |
    /// |    0    0    0   1 |
    pub const fn to_3x3_vec(&self) -> (crate::Mat3<T>, crate::Vec3<T>) {
        return ( self.m3x3(), self.translation());
    }

    /// Get the 3x3 matrix
    pub const fn m3x3(&self) -> crate::Mat3<T> {
        return crate::Mat3::new([
            self.at(0, 0), self.at(1, 0), self.at(2, 0),
            self.at(0, 1), self.at(1, 1), self.at(2, 1),
            self.at(0, 2), self.at(1, 2), self.at(2, 2),
        ]);
    }

    // Get the last translation column
    pub const fn translation(&self) -> crate::Vec3<T> {
        return crate::Vec3::new(
            self.at(0, 3),
            self.at(1, 3),
            self.at(2, 3),
        );
    }

    /// Get the element at row `row` and column `col`
    /// (zero indexed)
    #[inline(always)]
    pub const fn at(&self, row: usize, col: usize) -> T {
        return self.0[col * 4 + row];
    }

    /// Get mut reference to element at row `row` and column `col`
    /// (zero indexed)
    pub const fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        return &mut self.0[col * 4 + row];
    }
}

impl<T> Mat4<T>
where
    T: num_traits::Num + crate::traits::WholeConsts + std::ops::Neg<Output = T> + Copy,
{
    /// Get the identity matrix
    pub const fn identity() -> Self {
        return Mat4([
            T::ONE,  T::ZERO, T::ZERO, T::ZERO,
            T::ZERO, T::ONE,  T::ZERO, T::ZERO,
            T::ZERO, T::ZERO, T::ONE,  T::ZERO,
            T::ZERO, T::ZERO, T::ZERO, T::ONE,
        ]);
    }

    /// Construct a scaling matrix
    pub const fn scale(factor: T) -> Self {
        return Mat4([
            factor,  T::ZERO, T::ZERO, T::ZERO,
            T::ZERO, factor,  T::ZERO, T::ZERO,
            T::ZERO, T::ZERO, factor,  T::ZERO,
            T::ZERO, T::ZERO, T::ZERO, factor,
        ]);
    }

    /// Get matrix determinant
    pub fn det(&self) -> T {
        return self.0[0]  * (self.0[5]  * (self.0[10] * self.0[15] - self.0[14] * self.0[11])
                           + self.0[9]  * (self.0[14] * self.0[7]  - self.0[6]  * self.0[15])
                           + self.0[13] * (self.0[6]  * self.0[11] - self.0[10] * self.0[7]))

             + self.0[4]  * (self.0[1]  * (self.0[14] * self.0[11] - self.0[10] * self.0[15])
                           + self.0[9]  * (self.0[2]  * self.0[15] - self.0[14] * self.0[3])
                           + self.0[13] * (self.0[10] * self.0[3]  - self.0[2]  * self.0[11]))

             + self.0[8]  * (self.0[1]  * (self.0[6]  * self.0[15] - self.0[14] * self.0[7])
                           + self.0[5]  * (self.0[14] * self.0[3]  - self.0[2]  * self.0[15])
                           + self.0[13] * (self.0[2]  * self.0[7]  - self.0[6]  * self.0[3]))

             + self.0[12] * (self.0[1]  * (self.0[10] * self.0[7]  - self.0[6]  * self.0[11])
                           + self.0[5]  * (self.0[2]  * self.0[11] - self.0[10] * self.0[3])
                           + self.0[9]  * (self.0[6]  * self.0[3]  - self.0[2]  * self.0[7]));
    }

    /// Get the transposed matrix
    pub const fn t(&self) -> Self {
        return Mat4([
            self.0[0], self.0[4], self.0[8],  self.0[12],
            self.0[1], self.0[5], self.0[9],  self.0[13],
            self.0[2], self.0[6], self.0[10], self.0[14],
            self.0[3], self.0[7], self.0[11], self.0[15],
        ]);
    }

    /// Get an inverse of the `self`
    pub fn invert(&self) -> Self {
        let inv = self.invert_no_det();

        let det = self.0[0]  * inv.0[0]
                + self.0[4]  * inv.0[1]
                + self.0[8]  * inv.0[2]
                + self.0[12] * inv.0[3];

        return inv / det;
    }

    /// Get an inverse of `self` (but no devision by determinant)
    pub fn invert_no_det(&self) -> Self {
        return Mat4([
            /*
             * First column
             */
            self.0[5]  * (self.0[10] * self.0[15] - self.0[14] * self.0[11])
          + self.0[9]  * (self.0[14] * self.0[7]  - self.0[6]  * self.0[15])
          + self.0[13] * (self.0[6]  * self.0[11] - self.0[10] * self.0[7]),

            self.0[1]  * (self.0[14] * self.0[11] - self.0[10] * self.0[15])
          + self.0[9]  * (self.0[2]  * self.0[15] - self.0[14] * self.0[3])
          + self.0[13] * (self.0[10] * self.0[3]  - self.0[2]  * self.0[11]),

            self.0[1]  * (self.0[6]  * self.0[15] - self.0[14] * self.0[7])
          + self.0[5]  * (self.0[14] * self.0[3]  - self.0[2]  * self.0[15])
          + self.0[13] * (self.0[2]  * self.0[7]  - self.0[6]  * self.0[3]),

            self.0[1]  * (self.0[10] * self.0[7]  - self.0[6]  * self.0[11])
          + self.0[5]  * (self.0[2]  * self.0[11] - self.0[10] * self.0[3])
          + self.0[9]  * (self.0[6]  * self.0[3]  - self.0[2]  * self.0[7]),

            /*
             * Second column
             */
            self.0[4]  * (self.0[14] * self.0[11] - self.0[10] * self.0[15])
          + self.0[8]  * (self.0[6]  * self.0[15] - self.0[14] * self.0[7])
          + self.0[12] * (self.0[10] * self.0[7]  - self.0[6]  * self.0[11]),

            self.0[0]  * (self.0[10] * self.0[15] - self.0[14] * self.0[11])
          + self.0[8]  * (self.0[14] * self.0[3]  - self.0[2]  * self.0[15])
          + self.0[12] * (self.0[2]  * self.0[11] - self.0[10] * self.0[3]),

            self.0[0]  * (self.0[14] * self.0[7]  - self.0[6]  * self.0[15])
          + self.0[4]  * (self.0[2]  * self.0[15] - self.0[14] * self.0[3])
          + self.0[12] * (self.0[6]  * self.0[3]  - self.0[2]  * self.0[7]),

            self.0[0]  * (self.0[6]  * self.0[11] - self.0[10] * self.0[7])
          + self.0[4]  * (self.0[10] * self.0[3]  - self.0[2]  * self.0[11])
          + self.0[8]  * (self.0[2]  * self.0[7]  - self.0[6]  * self.0[3]),

            /*
             * Third column
             */
            self.0[4]  * (self.0[9]  * self.0[15] - self.0[13] * self.0[11])
          + self.0[8]  * (self.0[13] * self.0[7]  - self.0[5]  * self.0[15])
          + self.0[12] * (self.0[5]  * self.0[11] - self.0[9]  * self.0[7]),

            self.0[0]  * (self.0[13] * self.0[11] - self.0[9]  * self.0[15])
          + self.0[8]  * (self.0[1]  * self.0[15] - self.0[13] * self.0[3])
          + self.0[12] * (self.0[9]  * self.0[3]  - self.0[1]  * self.0[11]),

            self.0[0]  * (self.0[5]  * self.0[15] - self.0[13] * self.0[7])
          + self.0[4]  * (self.0[13] * self.0[3]  - self.0[1]  * self.0[15])
          + self.0[12] * (self.0[1]  * self.0[7]  - self.0[5]  * self.0[3]),

            self.0[0]  * (self.0[9]  * self.0[7]  - self.0[5]  * self.0[11])
          + self.0[4]  * (self.0[1]  * self.0[11] - self.0[9]  * self.0[3])
          + self.0[8]  * (self.0[5]  * self.0[3]  - self.0[1]  * self.0[7]),

            /*
             * Fourth column
             */
            self.0[4]  * (self.0[13] * self.0[10] - self.0[9]  * self.0[14])
          + self.0[8]  * (self.0[5]  * self.0[14] - self.0[13] * self.0[6])
          + self.0[12] * (self.0[9]  * self.0[6]  - self.0[5]  * self.0[10]),

            self.0[0]  * (self.0[9]  * self.0[14] - self.0[13] * self.0[10])
          + self.0[8]  * (self.0[13] * self.0[2]  - self.0[1]  * self.0[14])
          + self.0[12] * (self.0[1]  * self.0[10] - self.0[9]  * self.0[2]),

            self.0[0]  * (self.0[13] * self.0[6]  - self.0[5]  * self.0[14])
          + self.0[4]  * (self.0[1]  * self.0[14] - self.0[13] * self.0[2])
          + self.0[12] * (self.0[5]  * self.0[2]  - self.0[1]  * self.0[6]),

            self.0[0]  * (self.0[5]  * self.0[10] - self.0[9]  * self.0[6])
          + self.0[4]  * (self.0[9]  * self.0[2]  - self.0[1]  * self.0[10])
          + self.0[8]  * (self.0[1]  * self.0[6]  - self.0[5]  * self.0[2]),
        ]);
    }

    /// Construct a 4x4 matrix from a 3x3 matrix and a vector. It will look like:
    ///
    /// | m_11 m_12 m_13 v.x |
    /// | m_21 m_22 m_23 v.y |
    /// | m_31 m_32 m_33 v.z |
    /// |    0    0    0   1 |
    pub const fn from_3x3_vec(mat3: crate::Mat3<T>, vec: crate::Vec3<T>) -> Self {
        return Mat4([
            mat3.at(0, 0), mat3.at(1, 0), mat3.at(2, 0), T::ZERO,
            mat3.at(0, 1), mat3.at(1, 1), mat3.at(2, 1), T::ZERO,
            mat3.at(0, 2), mat3.at(1, 2), mat3.at(2, 2), T::ZERO,
            vec.x,         vec.y,         vec.z,         T::ONE,
        ]);
    }
}

impl<T> Mat4<T>
where
    T: num_traits::Float + crate::traits::WholeConsts + std::iter::Sum,
{
    /// Construct a perspective projection matrix
    ///
    /// * `fov`: - FOV in degrees
    /// * `aspect`: viewport aspect ratio: width / height
    /// * `near`: near plane
    /// * `far`: far plane
    pub fn perspective(fov: T, aspect: T, near: T, far: T) -> Self {
        let cot = T::ONE / T::tan(fov.to_radians() / T::TWO);
        let far_near = T::ONE / (far - near);

        return Mat4([
            cot / aspect, T::ZERO, T::ZERO,          T::ZERO,
            T::ZERO,      cot,     T::ZERO,          T::ZERO,
            T::ZERO,      T::ZERO, far_near,         T::ONE,
            T::ZERO,      T::ZERO, -near * far_near, T::ZERO,
        ]);
    }

    /// Get the norm
    pub fn norm(&self) -> T {
        return self.0.iter().map(|&x| x * x).sum::<T>().sqrt();
    }
}

//-----------------------------------------------------------------------------
// Operator overloads
impl<T> std::ops::Add for Mat4<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Mat4([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
            self.0[4] + rhs.0[4],
            self.0[5] + rhs.0[5],
            self.0[6] + rhs.0[6],
            self.0[7] + rhs.0[7],
            self.0[8] + rhs.0[8],
            self.0[9] + rhs.0[9],
            self.0[10] + rhs.0[10],
            self.0[11] + rhs.0[11],
            self.0[12] + rhs.0[12],
            self.0[13] + rhs.0[13],
            self.0[14] + rhs.0[14],
            self.0[15] + rhs.0[15],
        ]);
    }
}

impl<T> std::ops::Sub for Mat4<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Mat4([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
            self.0[4] - rhs.0[4],
            self.0[5] - rhs.0[5],
            self.0[6] - rhs.0[6],
            self.0[7] - rhs.0[7],
            self.0[8] - rhs.0[8],
            self.0[9] - rhs.0[9],
            self.0[10] - rhs.0[10],
            self.0[11] - rhs.0[11],
            self.0[12] - rhs.0[12],
            self.0[13] - rhs.0[13],
            self.0[14] - rhs.0[14],
            self.0[15] - rhs.0[15],
        ]);
    }
}

impl<T> std::ops::Mul<T> for Mat4<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        return Mat4([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
            self.0[4] * rhs,
            self.0[5] * rhs,
            self.0[6] * rhs,
            self.0[7] * rhs,
            self.0[8] * rhs,
            self.0[9] * rhs,
            self.0[10] * rhs,
            self.0[11] * rhs,
            self.0[12] * rhs,
            self.0[13] * rhs,
            self.0[14] * rhs,
            self.0[15] * rhs,
        ]);
    }
}

impl<T> std::ops::Mul<Vec4<T>> for Mat4<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        return Vec4 {
            x: self.0[0] * rhs.x + self.0[4] * rhs.y + self.0[8] * rhs.z + self.0[12] * rhs.w,
            y: self.0[1] * rhs.x + self.0[5] * rhs.y + self.0[9] * rhs.z + self.0[13] * rhs.w,
            z: self.0[2] * rhs.x + self.0[6] * rhs.y + self.0[10] * rhs.z + self.0[14] * rhs.w,
            w: self.0[3] * rhs.x + self.0[7] * rhs.y + self.0[11] * rhs.z + self.0[15] * rhs.w,
        };
    }
}

impl<T> std::ops::Mul for Mat4<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return Mat4([
            /*
             * First column
             */
            self.0[0] * rhs.0[0] + self.0[4] * rhs.0[1] + self.0[8]  * rhs.0[2] + self.0[12] * rhs.0[3],
            self.0[1] * rhs.0[0] + self.0[5] * rhs.0[1] + self.0[9]  * rhs.0[2] + self.0[13] * rhs.0[3],
            self.0[2] * rhs.0[0] + self.0[6] * rhs.0[1] + self.0[10] * rhs.0[2] + self.0[14] * rhs.0[3],
            self.0[3] * rhs.0[0] + self.0[7] * rhs.0[1] + self.0[11] * rhs.0[2] + self.0[15] * rhs.0[3],
            /*
             * Second column
             */
            self.0[0] * rhs.0[4] + self.0[4] * rhs.0[5] + self.0[8]  * rhs.0[6] + self.0[12] * rhs.0[7],
            self.0[1] * rhs.0[4] + self.0[5] * rhs.0[5] + self.0[9]  * rhs.0[6] + self.0[13] * rhs.0[7],
            self.0[2] * rhs.0[4] + self.0[6] * rhs.0[5] + self.0[10] * rhs.0[6] + self.0[14] * rhs.0[7],
            self.0[3] * rhs.0[4] + self.0[7] * rhs.0[5] + self.0[11] * rhs.0[6] + self.0[15] * rhs.0[7],
            /*
             * Third column
             */
            self.0[0] * rhs.0[8] + self.0[4] * rhs.0[9] + self.0[8]  * rhs.0[10] + self.0[12] * rhs.0[11],
            self.0[1] * rhs.0[8] + self.0[5] * rhs.0[9] + self.0[9]  * rhs.0[10] + self.0[13] * rhs.0[11],
            self.0[2] * rhs.0[8] + self.0[6] * rhs.0[9] + self.0[10] * rhs.0[10] + self.0[14] * rhs.0[11],
            self.0[3] * rhs.0[8] + self.0[7] * rhs.0[9] + self.0[11] * rhs.0[10] + self.0[15] * rhs.0[11],
            /*
             * Fourth column
             */
            self.0[0] * rhs.0[12] + self.0[4] * rhs.0[13] + self.0[8]  * rhs.0[14] + self.0[12] * rhs.0[15],
            self.0[1] * rhs.0[12] + self.0[5] * rhs.0[13] + self.0[9]  * rhs.0[14] + self.0[13] * rhs.0[15],
            self.0[2] * rhs.0[12] + self.0[6] * rhs.0[13] + self.0[10] * rhs.0[14] + self.0[14] * rhs.0[15],
            self.0[3] * rhs.0[12] + self.0[7] * rhs.0[13] + self.0[11] * rhs.0[14] + self.0[15] * rhs.0[15],
        ]);
    }
}

impl<T> std::ops::Div<T> for Mat4<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        return Mat4([
            self.0[0] / rhs,
            self.0[1] / rhs,
            self.0[2] / rhs,
            self.0[3] / rhs,
            self.0[4] / rhs,
            self.0[5] / rhs,
            self.0[6] / rhs,
            self.0[7] / rhs,
            self.0[8] / rhs,
            self.0[9] / rhs,
            self.0[10] / rhs,
            self.0[11] / rhs,
            self.0[12] / rhs,
            self.0[13] / rhs,
            self.0[14] / rhs,
            self.0[15] / rhs,
        ]);
    }
}

impl<T> std::ops::Neg for Mat4<T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Mat4([
            -self.0[0],
            -self.0[1],
            -self.0[2],
            -self.0[3],
            -self.0[4],
            -self.0[5],
            -self.0[6],
            -self.0[7],
            -self.0[8],
            -self.0[9],
            -self.0[10],
            -self.0[11],
            -self.0[12],
            -self.0[13],
            -self.0[14],
            -self.0[15],
        ]);
    }
}

//-----------------------------------------------------------------------------
