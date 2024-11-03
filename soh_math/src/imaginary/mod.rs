//-----------------------------------------------------------------------------
mod complex;
//-----------------------------------------------------------------------------
pub use complex::*;
//-----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex() {
        let c = Complex::new(0.0, 1.0);
        assert_eq!(c * c, Complex::new(-1.0, 0.0));

        let c1 = Complex::new(1.0, 1.0);
        let c2 = Complex::new(-2.5, 1.0);

        assert_eq!(c1 * c2, Complex::new(-3.5, -1.5));

        // Test powers
        let c = Complex::new(1.0, 1.0);
        let c_squared = Complex::new(0.0, 2.0);

        assert_eq!(c.powi(2), c_squared);
        assert!((c.powf(2.0) - c_squared).len() < 1.0e-10);
        assert!((c.powc(Complex::new(2.0, 0.0)) - c_squared).len() < 1.0e-10);
    }
}

//-----------------------------------------------------------------------------
