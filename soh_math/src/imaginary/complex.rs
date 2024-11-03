//-----------------------------------------------------------------------------
use num_traits::Float;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
//-----------------------------------------------------------------------------

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

//-----------------------------------------------------------------------------
// Constructors
impl<T> Complex<T>
where
    T: Float,
{
    pub fn new(real: T, imaginary: T) -> Self {
        return Self {
            re: real,
            im: imaginary,
        };
    }

    /// Complex number which equals to one
    pub fn one() -> Self {
        return T::one().into();
    }

    /// Complex number which equals to zero
    pub fn zero() -> Self {
        return T::zero().into();
    }

    /// Create a complex number from angle with unit length
    pub fn from_angle(angle: T) -> Self {
        return Complex {
            re: angle.cos(),
            im: angle.sin(),
        };
    }

    /// Create a complex number from length and angle
    pub fn from_param(length: T, angle: T) -> Self {
        return Complex {
            re: length * angle.cos(),
            im: length * angle.sin(),
        };
    }
}

// Math functions
impl<T> Complex<T>
where
    T: Float + From<f64>,
{
    /// Real part
    pub fn real(&self) -> T {
        return self.re;
    }

    /// Imaginary part
    pub fn imag(&self) -> T {
        return self.im;
    }

    /// Get the angle of the complex number
    pub fn phi(&self) -> T {
        return self.im.atan2(self.re);
    }

    /// Calculate the squared length
    pub fn len2(&self) -> T {
        return self.re * self.re + self.im * self.im;
    }

    /// Calculate the length (absolute value)
    pub fn len(&self) -> T {
        return T::hypot(self.re, self.im);
    }

    /// Calculate the natural logarithm
    pub fn ln(&self) -> Self {
        return Complex {
            re: self.ln_len(),
            im: self.phi(),
        };
    }

    /// Calculate the natural logarithm of the length
    pub fn ln_len(&self) -> T {
        return (self.len2()).ln() * 0.5.into();
    }

    /// Calculate the integer power of the number
    pub fn powi(&self, pow: u32) -> Self {
        let mut result: Complex<T> = Complex::one();
        let mut k: u32 = pow;
        let mut a: Complex<T> = *self;

        loop {
            let n = k / 2;
            if 2 * n < k {
                result = result * a;
            }
            k = n;
            a = a * a;
            if k == 0 {
                break;
            }
        }

        return result;
    }

    /// Calculate the float power of the number
    pub fn powf(&self, pow: T) -> Self {
        if *self == Complex::zero() {
            return pow.into();
        }

        let res_ln_len = pow * self.ln_len();
        return Complex::from_param(res_ln_len.exp(), pow * self.phi());
    }

    /// Calculate the complex power
    pub fn powc(&self, pow: Self) -> Self {
        if *self == Complex::zero() {
            return pow;
        }

        let c = pow * Complex::new(self.ln_len(), self.phi());
        return Complex::from_param(c.re.exp(), c.im);
    }

    /// Get the conjugate
    pub fn conjugate(&self) -> Self {
        return Complex {
            re: self.re,
            im: -self.im,
        };
    }
}

//-----------------------------------------------------------------------------
// Operator overloads
impl<T> std::ops::Add for Complex<T>
where
    T: Float + From<f64>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        return Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        };
    }
}

impl<T> std::ops::Sub for Complex<T>
where
    T: Float + From<f64>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        return Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        };
    }
}

impl<T> std::ops::Mul for Complex<T>
where
    T: Float + From<f64>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        return Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.im * rhs.re + self.re * rhs.im,
        };
    }
}

impl<T> std::ops::Mul<T> for Complex<T>
where
    T: Float + From<f64>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        return Complex {
            re: self.re * rhs,
            im: self.im * rhs,
        };
    }
}

impl<T> std::ops::Div for Complex<T>
where
    T: Float + From<f64>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let l = rhs.re * rhs.re + rhs.im * rhs.im;

        let re_div = (self.re * rhs.re + self.im * rhs.im) / l;
        let im_div = (self.im * rhs.re - self.re * rhs.im) / l;

        return Complex {
            re: re_div,
            im: im_div,
        };
    }
}

impl<T> std::ops::Div<T> for Complex<T>
where
    T: Float + From<f64>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        return Complex {
            re: self.re / rhs,
            im: self.im / rhs,
        };
    }
}

impl<T> std::ops::Neg for Complex<T>
where
    T: Float + From<f64>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Complex {
            re: -self.re,
            im: -self.im,
        };
    }
}

//-----------------------------------------------------------------------------
// From implementations
impl<T> From<T> for Complex<T>
where
    T: Float,
{
    fn from(value: T) -> Self {
        return Complex {
            re: value,
            im: T::zero(),
        };
    }
}

//-----------------------------------------------------------------------------

impl<T> std::fmt::Display for Complex<T>
where
    T: Float + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.im >= T::zero() {
            write!(f, "{} + {} * i", self.re, self.im)
        } else {
            write!(f, "{} - {} * i", self.re, -self.im)
        }
    }
}

//-----------------------------------------------------------------------------
