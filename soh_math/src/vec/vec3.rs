//-----------------------------------------------------------------------------
use macros::impl_vec;
//-----------------------------------------------------------------------------
#[repr(C)]
#[impl_vec]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: num_traits::Float,
{
    #[inline]
    pub fn cross(vec1: &Self, vec2: &Self) -> Vec3<T> {
        return Vec3 {
            x: vec1.y * vec2.z - vec1.z * vec2.y,
            y: vec1.z * vec2.x - vec1.x * vec2.z,
            z: vec1.x * vec2.y - vec1.y * vec2.x,
        };
    }
}
