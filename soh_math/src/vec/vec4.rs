//-----------------------------------------------------------------------------

#[macro_impl_vec::impl_vec]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

//-----------------------------------------------------------------------------
// Basis vectors
impl<T> Vec4<T>
where
    T: crate::traits::WholeConsts + Copy,
{
    pub const X: Self = Vec4::new(T::ONE, T::ZERO, T::ZERO, T::ZERO);
    pub const Y: Self = Vec4::new(T::ZERO, T::ONE, T::ZERO, T::ZERO);
    pub const Z: Self = Vec4::new(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    pub const W: Self = Vec4::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
}

//-----------------------------------------------------------------------------
