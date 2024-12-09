//-----------------------------------------------------------------------------

#[macro_impl_vec::impl_vec]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

//-----------------------------------------------------------------------------
// Basis vectors
impl<T> Vec2<T>
where
    T: crate::traits::WholeConsts + Copy,
{
    pub const X: Self = Vec2::new(T::ONE, T::ZERO);
    pub const Y: Self = Vec2::new(T::ZERO, T::ONE);
}

//-----------------------------------------------------------------------------
// Math functions
impl<T> Vec2<T>
where
    T: num_traits::Num + Copy,
{
    pub fn cross(vec1: Vec2<T>, vec2: Vec2<T>) -> T {
        return vec1.x * vec2.y - vec1.y * vec2.x;
    }
}

//-----------------------------------------------------------------------------
