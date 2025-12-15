//-----------------------------------------------------------------------------
// Formatting makes matrix code disgusting
#[rustfmt::skip]
pub mod mat;
pub mod color;
pub mod fractal;
pub mod imaginary;
pub mod vec;
//-----------------------------------------------------------------------------
pub use imaginary::*;
pub use mat::*;
pub use vec::*;
//-----------------------------------------------------------------------------
pub mod traits;
pub use traits::Convert;
//-----------------------------------------------------------------------------
/// Linear interpolation
pub fn lerp<V, T>(a: V, b: V, t: T) -> V
where
    V: std::ops::Add<Output = V> + std::ops::Sub<Output = V> + std::ops::Mul<T, Output = V> + Copy,
{
    return a + (b - a) * t;
}

/// Find coordinate y of a point (x, y) that lies on a line that goes through points (x0, y0) and (x1, y1)
pub fn linear_func<T>(x0: T, y0: T, x1: T, y1: T, x: T) -> T
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + Copy,
{
    return y0 + (y1 - y0) * (x - x0) / (x1 - x0);
}

//-----------------------------------------------------------------------------

pub mod consts {
    pub const DEG_TO_RAD_F64: f64 = core::f64::consts::PI / 180.0;
    pub const DEG_TO_RAD_F32: f32 = core::f32::consts::PI / 180.0;
    pub const RAD_TO_DEG_F64: f64 = 180.0 / core::f64::consts::PI;
    pub const RAD_TO_DEG_F32: f32 = 180.0 / core::f32::consts::PI;
}

//-----------------------------------------------------------------------------
