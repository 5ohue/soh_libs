//-----------------------------------------------------------------------------
use macros::impl_vec;
use num_traits::Float;
//-----------------------------------------------------------------------------
#[repr(C)]
#[impl_vec]
pub struct Vec2<F> {
    pub x: F,
    pub y: F,
}

//-----------------------------------------------------------------------------
// Math functions
impl<F> Vec2<F>
where
    F: Float,
{
    pub fn cross(vec1: &Self, vec2: &Self) -> F {
        return vec1.x * vec2.y - vec1.y * vec2.x;
    }
}

//-----------------------------------------------------------------------------
// From implementations
impl<F> From<[F; 2]> for Vec2<F>
where
    F: Copy,
{
    fn from(value: [F; 2]) -> Self {
        return Vec2 {
            x: value[0],
            y: value[1],
        };
    }
}
impl<F> From<Vec2<F>> for [F; 2]
where
    F: Copy,
{
    fn from(value: Vec2<F>) -> Self {
        return [value.x, value.y];
    }
}

impl<F> From<(F, F)> for Vec2<F>
where
    F: Copy,
{
    fn from(value: (F, F)) -> Self {
        return Vec2 {
            x: value.0,
            y: value.1,
        };
    }
}
impl<F> From<Vec2<F>> for (F, F)
where
    F: Copy,
{
    fn from(value: Vec2<F>) -> Self {
        return (value.x, value.y);
    }
}

//-----------------------------------------------------------------------------
