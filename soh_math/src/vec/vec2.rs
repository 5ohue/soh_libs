//-----------------------------------------------------------------------------

#[macro_impl_vec::impl_vec]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

//-----------------------------------------------------------------------------
// Math functions
impl<T> Vec2<T>
where
    T: num_traits::Float,
{
    pub fn cross(vec1: &Self, vec2: &Self) -> T {
        return vec1.x * vec2.y - vec1.y * vec2.x;
    }
}

//-----------------------------------------------------------------------------
