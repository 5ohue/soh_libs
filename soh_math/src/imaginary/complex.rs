//-----------------------------------------------------------------------------

#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

//-----------------------------------------------------------------------------
// Constructors
impl<T> Complex<T> {
    pub const fn new(real: T, imaginary: T) -> Self {
        return Self {
            re: real,
            im: imaginary,
        };
    }
}

impl<T> Complex<T>
where
    T: num_traits::Num,
{
    /// Complex number which equals to zero
    pub fn zero() -> Self {
        return T::zero().into();
    }

    /// Complex number which equals to one
    pub fn one() -> Self {
        return T::one().into();
    }
}

impl<T> Complex<T>
where
    T: num_traits::Float,
{
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

//-----------------------------------------------------------------------------
// Math functions
impl<T> Complex<T>
where
    T: Copy,
{
    /// Real part
    pub const fn real(&self) -> T {
        return self.re;
    }

    /// Imaginary part
    pub const fn imag(&self) -> T {
        return self.im;
    }
}

impl<T> Complex<T>
where
    T: num_traits::Num + Copy,
{
    /// Calculate the squared length
    pub fn len2(&self) -> T {
        return self.re * self.re + self.im * self.im;
    }

    /// Calculate the integer power of the number
    pub fn powi(&self, pow: u32) -> Self {
        let mut result: Complex<T> = Complex::one();
        let mut k: u32 = pow;
        let mut a: Complex<T> = *self;

        loop {
            let n = k / 2;
            if 2 * n < k {
                result *= a;
            }
            k = n;
            a *= a;
            if k == 0 {
                break;
            }
        }

        return result;
    }
}

impl<T> Complex<T>
where
    T: num_traits::Float + From<f32>,
{
    /// Get the angle of the complex number
    pub fn phi(&self) -> T {
        return self.im.atan2(self.re);
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
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        return Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        };
    }
}

impl<T> std::ops::AddAssign for Complex<T>
where
    T: std::ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> std::ops::Sub for Complex<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        return Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        };
    }
}

impl<T> std::ops::SubAssign for Complex<T>
where
    T: std::ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl<T> std::ops::Mul<T> for Complex<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        return Complex {
            re: self.re * rhs,
            im: self.im * rhs,
        };
    }
}

impl<T> std::ops::MulAssign<T> for Complex<T>
where
    T: std::ops::MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.re *= rhs;
        self.im *= rhs;
    }
}

impl<T> std::ops::Mul for Complex<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        return Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.im * rhs.re + self.re * rhs.im,
        };
    }
}

impl<T> std::ops::MulAssign for Complex<T>
where
    T: num_traits::Num + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T> std::ops::Div<T> for Complex<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        return Complex {
            re: self.re / rhs,
            im: self.im / rhs,
        };
    }
}

impl<T> std::ops::DivAssign<T> for Complex<T>
where
    T: std::ops::DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.re /= rhs;
        self.im /= rhs;
    }
}

impl<T> std::ops::Div for Complex<T>
where
    T: num_traits::Num + Copy,
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

impl<T> std::ops::DivAssign for Complex<T>
where
    T: num_traits::Num + Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T> std::ops::Neg for Complex<T>
where
    T: std::ops::Neg<Output = T>,
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
    T: num_traits::Zero,
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
    T: num_traits::Num + std::ops::Neg<Output = T> + PartialOrd + std::fmt::Display + Copy,
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
