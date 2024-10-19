/// Returns the high bits of the product of two 32-bit numbers.
///
/// This function is used to calculate the high bits of the product of two 32-bit numbers.
/// It works by casting the numbers to 64-bit, multiplying them, and then shifting the result right by 32 bits.
#[inline(always)]
fn get_hi_bits_for_u32_mul(a: u32, b: u32) -> u32 {
    return ((a as u64 * b as u64) >> 32) as u32;
}

/// Returns the high bits of the product of two 64-bit numbers.
///
/// This function is used to calculate the high bits of the product of two 64-bit numbers.
/// It works by splitting each number into high and low bits, multiplying the high and low bits separately,
/// and then combining the results.
#[inline(always)]
fn get_hi_bits_for_u64_mul(a: u64, b: u64) -> u64 {
    let a_hi = a >> 32;
    let a_lo = a & 0xFFFFFFFF;
    let b_hi = b >> 32;
    let b_lo = b & 0xFFFFFFFF;

    let res = (a_hi * b_lo >> 32) + (a_lo * b_hi >> 32) + a_hi * b_hi;
    return res;
}

/// A trait for types that can be generated randomly from a 32-bit number.
///
/// This trait provides methods for generating a value of the implementing type from a 32-bit random number.
pub trait RandomlyGenerated32
where
    Self: std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + Copy,
{
    /// Generates a value of the implementing type from a 32-bit random number.
    fn from_rand_32(rnum: u32) -> Self;

    /// Generates a value of the implementing type from a 32-bit random number, scaled to a maximum value.
    fn from_rand_32_to(rnum: u32, to: Self) -> Self;

    /// Generates a value of the implementing type from a 32-bit random number, within a specified range.
    fn from_rand_32_range(rnum: u32, from: Self, to: Self) -> Self {
        return from + Self::from_rand_32_to(rnum, to - from);
    }
}

/// A trait for types that can be generated randomly from a 64-bit number.
///
/// This trait provides methods for generating a value of the implementing type from a 64-bit random number.
pub trait RandomlyGenerated64
where
    Self: std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + Copy,
{
    /// Generates a value of the implementing type from a 64-bit random number.
    fn from_rand_64(rnum: u64) -> Self;

    /// Generates a value of the implementing type from a 64-bit random number, scaled to a maximum value.
    fn from_rand_64_to(rnum: u64, to: Self) -> Self;

    /// Generates a value of the implementing type from a 64-bit random number, within a specified range.
    fn from_rand_64_range(rnum: u64, from: Self, to: Self) -> Self {
        return from + Self::from_rand_64_to(rnum, to - from);
    }
}

macro_rules! impl_32 {
    ($uint:ty, $int:ty) => {
        impl RandomlyGenerated32 for $uint {
            #[inline(always)]
            fn from_rand_32(rnum: u32) -> $uint {
                // Take highest bits
                return (rnum >> (32 - size_of::<$uint>() * 8)) as $uint;
            }

            #[inline(always)]
            fn from_rand_32_to(rnum: u32, to: $uint) -> $uint {
                let mul = get_hi_bits_for_u32_mul(rnum, to as u32);
                return mul as $uint;
            }
        }

        impl RandomlyGenerated32 for $int {
            #[inline(always)]
            fn from_rand_32(rnum: u32) -> $int {
                return <$uint>::from_rand_32(rnum) as $int;
            }

            // Doesn't generate negative numbers
            #[inline(always)]
            fn from_rand_32_to(rnum: u32, to: $int) -> $int {
                if to < 0 {
                    return 0;
                }

                return <$uint>::from_rand_32_to(rnum, to as $uint) as $int;
            }
        }
    };
}

macro_rules! impl_64 {
    ($uint:ty, $int:ty) => {
        impl RandomlyGenerated64 for $uint {
            #[inline(always)]
            fn from_rand_64(rnum: u64) -> $uint {
                // Take highest bits
                return (rnum >> (64 - size_of::<$uint>() * 8)) as $uint;
            }

            #[inline(always)]
            fn from_rand_64_to(rnum: u64, to: $uint) -> $uint {
                let mul = get_hi_bits_for_u64_mul(rnum, to as u64);
                return mul as $uint;
            }
        }

        impl RandomlyGenerated64 for $int {
            #[inline(always)]
            fn from_rand_64(rnum: u64) -> $int {
                return <$uint>::from_rand_64(rnum) as $int;
            }

            // Doesn't generate negative numbers
            #[inline(always)]
            fn from_rand_64_to(rnum: u64, to: $int) -> $int {
                if to < 0 {
                    return 0;
                }

                return <$uint>::from_rand_64_to(rnum, to as $uint) as $int;
            }
        }
    };
}

impl_32!(u8, i8);
impl_32!(u16, i16);
impl_32!(u32, i32);

impl_64!(u8, i8);
impl_64!(u16, i16);
impl_64!(u32, i32);
impl_64!(u64, i64);
impl_64!(usize, isize);

impl RandomlyGenerated32 for f32 {
    fn from_rand_32(rnum: u32) -> f32 {
        return rnum as f32 / (u32::MAX as f32 + 1.0);
    }

    fn from_rand_32_to(rnum: u32, to: f32) -> f32 {
        return f32::from_rand_32(rnum) * to;
    }
}

impl RandomlyGenerated32 for f64 {
    fn from_rand_32(rnum: u32) -> f64 {
        return rnum as f64 / (u32::MAX as f64 + 1.0);
    }

    fn from_rand_32_to(rnum: u32, to: Self) -> Self {
        return f64::from_rand_32(rnum) * to;
    }
}

// Cast f64 numbers down to f32
impl RandomlyGenerated64 for f32 {
    fn from_rand_64(rnum: u64) -> f32 {
        return f64::from_rand_64(rnum) as f32;
    }

    fn from_rand_64_to(rnum: u64, to: Self) -> Self {
        return f64::from_rand_64_to(rnum, to as f64) as f32;
    }

    fn from_rand_64_range(rnum: u64, from: Self, to: Self) -> Self {
        return f64::from_rand_64_range(rnum, from as f64, to as f64) as f32;
    }
}

impl RandomlyGenerated64 for f64 {
    fn from_rand_64(rnum: u64) -> f64 {
        const TWO_TO_NEG_53: f64 = 1.0 / (1u64 << 53) as f64;

        return (rnum >> 11) as f64 * TWO_TO_NEG_53;
    }

    fn from_rand_64_to(rnum: u64, to: f64) -> f64 {
        return f64::from_rand_64(rnum) * to;
    }
}
