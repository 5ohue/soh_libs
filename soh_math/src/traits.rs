//-----------------------------------------------------------------------------

pub trait Convert<To> {
    fn convert(&self) -> To;
}

//-----------------------------------------------------------------------------
// Const traits

#[cfg(feature = "f128")]
use f128_num::{f128, f128_inner};

pub trait WholeConsts: Sized {
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
}

macro_rules! impl_whole_consts {
    ($zero:expr, $one:expr, $two:expr, $($t:ty)*) => {
        $(
            impl WholeConsts for $t {
                const ZERO: Self = $zero;
                const ONE: Self = $one;
                const TWO: Self = $two;
            }
        )*
    }
}

impl_whole_consts!(0, 1, 2, i8 u8
                            i16 u16
                            i32 u32
                            i64 u64);

impl_whole_consts!(0.0, 1.0, 2.0, f32 f64);

#[cfg(feature = "f128")]
impl_whole_consts!(f128::ZERO, f128::ONE, f128!(2.0), f128);

pub trait RealConsts {
    const ONE_HALF: Self;
}

impl RealConsts for f32 {
    const ONE_HALF: Self = 0.5;
}

impl RealConsts for f64 {
    const ONE_HALF: Self = 0.5;
}

#[cfg(feature = "f128")]
impl RealConsts for f128_num::f128 {
    const ONE_HALF: Self = f128_num::f128!(0.5);
}

//-----------------------------------------------------------------------------
