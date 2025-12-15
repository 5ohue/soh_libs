//-----------------------------------------------------------------------------
// Different complex number based fractals implemented on CPU
//-----------------------------------------------------------------------------
use crate::Complex;
//-----------------------------------------------------------------------------
/// Sentinel value for when iteration didn't blow up
pub const QUALIFIED: f64 = -999.99;
//-----------------------------------------------------------------------------
// Generic trait for fractals
pub trait Fractal {
    /// Function that computes the starting position for z
    fn start_point(&self, pixel_coord: Complex<f64>) -> Complex<f64>;

    /// Function to iterate in the fractal
    fn iter_func(&self, z: &mut Complex<f64>, pixel_coord: Complex<f64>);

    /// Function that creates float value from iteration result
    fn iter_to_value(
        &self,
        z: Complex<f64>,
        pixel_coord: Complex<f64>,
        counter: u64,
        length_bound: f64,
    ) -> f64;

    /// Function that Iterates
    fn iterate(&self, pixel_coord: Complex<f64>, iteration_bound: u64, length_bound: f64) -> f64 {
        let mut counter = 0;
        let mut z = self.start_point(pixel_coord);

        while counter < iteration_bound && z.len2() < length_bound {
            self.iter_func(&mut z, pixel_coord);
            counter += 1;
        }

        if counter == iteration_bound {
            return QUALIFIED;
        } else {
            return self.iter_to_value(z, pixel_coord, counter, length_bound);
        }
    }
}

//-----------------------------------------------------------------------------
// Generic trait for types that a complex number can be raised to a power of
pub trait ComplexPower: Copy {
    /// Calculate `num^self`
    fn pow(self, num: Complex<f64>) -> Complex<f64>;

    /// Get the real part
    fn real(self) -> f64;
}

impl ComplexPower for u32 {
    fn pow(self, num: Complex<f64>) -> Complex<f64> {
        return num.powi(self);
    }

    fn real(self) -> f64 {
        return self as f64;
    }
}

impl ComplexPower for f64 {
    fn pow(self, num: Complex<f64>) -> Complex<f64> {
        return num.powf(self);
    }

    fn real(self) -> f64 {
        return self;
    }
}

impl ComplexPower for Complex<f64> {
    fn pow(self, num: Complex<f64>) -> Complex<f64> {
        return num.powc(self);
    }

    fn real(self) -> f64 {
        return self.re;
    }
}

//-----------------------------------------------------------------------------
// Multibrot fractals
#[derive(Clone, Copy)]
pub struct Multibrot<P>
where
    P: ComplexPower,
{
    pub start_point: Complex<f64>,
    pub pow: P,
}

#[derive(Default, Clone, Copy)]
pub struct MultibrotJulia<P>
where
    P: ComplexPower,
{
    pub center: Complex<f64>,
    pub pow: P,
}

impl<P> Fractal for Multibrot<P>
where
    P: ComplexPower,
{
    #[inline(always)]
    fn start_point(&self, _pixel_coord: Complex<f64>) -> Complex<f64> {
        return self.start_point;
    }

    #[inline(always)]
    fn iter_func(&self, z: &mut Complex<f64>, pixel_coord: Complex<f64>) {
        *z = self.pow.pow(*z) + pixel_coord;
    }

    #[inline(always)]
    fn iter_to_value(
        &self,
        z: Complex<f64>,
        _pixel_coord: Complex<f64>,
        counter: u64,
        length_bound: f64,
    ) -> f64 {
        return iter_to_value(self.pow.real(), z, counter, length_bound);
    }
}

impl<P> Fractal for MultibrotJulia<P>
where
    P: ComplexPower,
{
    #[inline(always)]
    fn start_point(&self, pixel_coord: Complex<f64>) -> Complex<f64> {
        return pixel_coord;
    }

    #[inline(always)]
    fn iter_func(&self, z: &mut Complex<f64>, _pixel_coord: Complex<f64>) {
        *z = self.pow.pow(*z) + self.center;
    }

    #[inline(always)]
    fn iter_to_value(
        &self,
        z: Complex<f64>,
        _pixel_coord: Complex<f64>,
        counter: u64,
        length_bound: f64,
    ) -> f64 {
        return iter_to_value(self.pow.real(), z, counter, length_bound);
    }
}

//-----------------------------------------------------------------------------
// Multicorn fractals
#[derive(Default, Clone, Copy)]
pub struct Multicorn<P>
where
    P: ComplexPower,
{
    pub start_point: Complex<f64>,
    pub pow: P,
}

#[derive(Default, Clone, Copy)]
pub struct MulticornJulia<P>
where
    P: ComplexPower,
{
    pub center: Complex<f64>,
    pub pow: P,
}

impl<P> Fractal for Multicorn<P>
where
    P: ComplexPower,
{
    #[inline(always)]
    fn start_point(&self, _pixel_coord: Complex<f64>) -> Complex<f64> {
        return self.start_point;
    }

    #[inline(always)]
    fn iter_func(&self, z: &mut Complex<f64>, pixel_coord: Complex<f64>) {
        *z = self.pow.pow(z.conjugate()) + pixel_coord;
    }

    #[inline(always)]
    fn iter_to_value(
        &self,
        z: Complex<f64>,
        _pixel_coord: Complex<f64>,
        counter: u64,
        length_bound: f64,
    ) -> f64 {
        return iter_to_value(self.pow.real(), z, counter, length_bound);
    }
}

impl<P> Fractal for MulticornJulia<P>
where
    P: ComplexPower,
{
    #[inline(always)]
    fn start_point(&self, pixel_coord: Complex<f64>) -> Complex<f64> {
        return pixel_coord;
    }

    #[inline(always)]
    fn iter_func(&self, z: &mut Complex<f64>, _pixel_coord: Complex<f64>) {
        *z = self.pow.pow(z.conjugate()) + self.center;
    }

    #[inline(always)]
    fn iter_to_value(
        &self,
        z: Complex<f64>,
        _pixel_coord: Complex<f64>,
        counter: u64,
        length_bound: f64,
    ) -> f64 {
        return iter_to_value(self.pow.real(), z, counter, length_bound);
    }
}

//-----------------------------------------------------------------------------
// Lambda fractals
#[derive(Default, Clone, Copy)]
pub struct Lambda<P>
where
    P: ComplexPower,
{
    pub start_point: Complex<f64>,
    pub pow: P,
}

#[derive(Default, Clone, Copy)]
pub struct LambdaJulia<P>
where
    P: ComplexPower,
{
    pub center: Complex<f64>,
    pub pow: P,
}

impl<P> Fractal for Lambda<P>
where
    P: ComplexPower,
{
    #[inline(always)]
    fn start_point(&self, _pixel_coord: Complex<f64>) -> Complex<f64> {
        return self.start_point;
    }

    #[inline(always)]
    fn iter_func(&self, z: &mut Complex<f64>, pixel_coord: Complex<f64>) {
        *z = pixel_coord * (*z - self.pow.pow(*z));
    }

    #[inline(always)]
    fn iter_to_value(
        &self,
        z: Complex<f64>,
        pixel_coord: Complex<f64>,
        counter: u64,
        length_bound: f64,
    ) -> f64 {
        return iter_to_value_lambda(self.pow.real(), z, counter, length_bound, pixel_coord);
    }
}

impl<P> Fractal for LambdaJulia<P>
where
    P: ComplexPower,
{
    #[inline(always)]
    fn start_point(&self, pixel_coord: Complex<f64>) -> Complex<f64> {
        return pixel_coord;
    }

    #[inline(always)]
    fn iter_func(&self, z: &mut Complex<f64>, _pixel_coord: Complex<f64>) {
        *z = self.center * (*z - self.pow.pow(*z));
    }

    #[inline(always)]
    fn iter_to_value(
        &self,
        z: Complex<f64>,
        _pixel_coord: Complex<f64>,
        counter: u64,
        length_bound: f64,
    ) -> f64 {
        return iter_to_value_lambda(self.pow.real(), z, counter, length_bound, self.center);
    }
}

//-----------------------------------------------------------------------------
// Helper functions:
#[inline(always)]
fn iter_to_value(pow: f64, z: Complex<f64>, counter: u64, length_bound: f64) -> f64 {
    let ln_sub = (z.len2().ln() / length_bound.ln()).ln() / pow.ln();
    let smooth_n = (counter + 1) as f64 - ln_sub;
    return smooth_n;
}

#[inline(always)]
fn iter_to_value_lambda(
    pow: f64,
    z: Complex<f64>,
    counter: u64,
    length_bound: f64,
    lambda: Complex<f64>,
) -> f64 {
    let mod_lambda = lambda.len();
    let b = mod_lambda * length_bound;
    let mod_l_z = z.len() * mod_lambda;

    let ln_sub = (mod_l_z.ln() / b.ln()).ln() / pow.ln();
    let smooth_n = (counter + 1) as f64 - ln_sub;
    return smooth_n;
}

//-----------------------------------------------------------------------------
