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

impl<T> Complex<T> {
    pub fn new(real: T, imaginary: T) -> Self {
        return Self {
            re: real,
            im: imaginary,
        };
    }
}

impl<T> Complex<T>
where
    T: Float + From<f64>,
{
    pub fn one() -> Self {
        return Self::new(T::one(), T::zero());
    }

    pub fn phi(&self) -> T {
        return self.im.atan2(self.re);
    }

    pub fn len2(&self) -> T {
        return self.re * self.re + self.im * self.im;
    }

    pub fn len(&self) -> T {
        return self.len2().sqrt();
    }

    pub fn ln(&self) -> Self {
        return Complex {
            re: (self.len2()).ln() * 0.5.into(),
            im: self.phi(),
        };
    }

    pub fn conjugate(&self) -> Self {
        return Complex {
            re: self.re,
            im: -self.im,
        };
    }

    pub fn from_param(length: T, angle: T) -> Self {
        return Complex {
            re: length * angle.cos(),
            im: length * angle.sin(),
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
