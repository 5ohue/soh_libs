//-----------------------------------------------------------------------------

#[macro_impl_vec::impl_vec]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

//-----------------------------------------------------------------------------
// Basis vectors
impl<T> Vec3<T>
where
    T: crate::traits::WholeConsts + Copy,
{
    pub const X: Self = Vec3::new(T::ONE, T::ZERO, T::ZERO);
    pub const Y: Self = Vec3::new(T::ZERO, T::ONE, T::ZERO);
    pub const Z: Self = Vec3::new(T::ZERO, T::ZERO, T::ONE);
}

//-----------------------------------------------------------------------------
// Math functions
impl<T> Vec3<T>
where
    T: num_traits::Num + Copy,
{
    #[inline]
    pub fn cross(vec1: &Vec3<T>, vec2: &Vec3<T>) -> Vec3<T> {
        return Vec3 {
            x: vec1.y * vec2.z - vec1.z * vec2.y,
            y: vec1.z * vec2.x - vec1.x * vec2.z,
            z: vec1.x * vec2.y - vec1.y * vec2.x,
        };
    }
}

//-----------------------------------------------------------------------------
