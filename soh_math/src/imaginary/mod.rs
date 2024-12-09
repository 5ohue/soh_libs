//-----------------------------------------------------------------------------
mod complex;
mod quat;
//-----------------------------------------------------------------------------
pub use complex::*;
pub use quat::*;
//-----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #[cfg(feature = "f128")]
    use f128_num::{f128, f128_inner};
    use soh_rng::Engine64;

    use crate::*;

    #[test]
    fn test_complex() {
        // Test that you can construct a complex number using f32
        let c = Complex::<f32>::new(0.0, 0.0);
        assert_eq!(c.len(), 0.0);

        // Test sqrt(-1)
        let c = Complex::new(0.0, 1.0);
        assert_eq!(c * c, Complex::new(-1.0, 0.0));

        // Test multiplication
        let c1 = Complex::new(1.0, 1.0);
        let c2 = Complex::new(-2.5, 1.0);
        assert_eq!(c1 * c2, Complex::new(-3.5, -1.5));

        // Test powers
        let c = Complex::new(1.0, 1.0);
        let c_squared = Complex::new(0.0, 2.0);

        assert_eq!(c.powi(2), c_squared);
        assert!((c.powf(2.0) - c_squared).len() < 1.0e-10);
        assert!((c.powc(Complex::new(2.0, 0.0)) - c_squared).len() < 1.0e-10);

        // Test exp and ln
        let mut rng = soh_rng::RNG64::new(0xdeadbeef);

        for _ in 0..100_000 {
            let c = Complex::from_param(rng.gen_range(0.5, 1.5), rng.gen_to(std::f64::consts::TAU));
            let c_1 = c.exp().ln();

            assert!((c - c_1).len() < 1.0e-10);
        }

        // Test f128
        #[cfg(feature = "f128")]
        {
            let c = Complex::new(f128!(1.0), f128!(1.0));
            let c_squared = Complex::new(0.0, 2.0);

            let c1 = c.powi(2);
            let c2 = c.powf(f128!(2.0));
            let c3 = c.powc(Complex::new(f128!(2.0), f128!(0.0)));

            let c1: Complex<f64> = Complex::new(c1.re.into(), c1.im.into());
            let c2: Complex<f64> = Complex::new(c2.re.into(), c2.im.into());
            let c3: Complex<f64> = Complex::new(c3.re.into(), c3.im.into());

            assert_eq!(c1, c_squared);
            assert!((c2 - c_squared).len() < 1.0e-10);
            assert!((c3 - c_squared).len() < 1.0e-10);
        }
    }

    #[test]
    fn test_quat() {
        // Test that rotation quaternions are unit length
        let mut rng = soh_rng::RNG64::new(0xdeadbeef);

        for _ in 0..100_000 {
            let axis: Vec3<f64> = Vec3::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
            );

            let angle = rng.gen_to(std::f64::consts::TAU);

            let rotation_quat = Quaternion::from_axis_angle(axis, angle);

            let delta = (rotation_quat.len() - 1.0).abs();

            if delta > 1.0e-10 {
                dbg!(delta, rotation_quat.len(), rotation_quat);
            }

            assert!(delta < 1.0e-10);
        }

        // Test simple rotations
        for _ in 0..100_000 {
            let eps = 1.0e-10;
            let angle = rng.gen_to(std::f64::consts::TAU);
            let point: Vec3<f64> = Vec3::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
            );

            // Test yaw
            let m_yaw = Mat3::yaw(angle);
            let q_yaw = Quaternion::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle);
            let p_1 = m_yaw * point;
            let p_2 = q_yaw.rotate(point);
            assert!((p_1 - p_2).len() < eps);

            // Test pitch
            let m_pitch = Mat3::pitch(angle);
            let q_pitch = Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), angle);
            let p_1 = m_pitch * point;
            let p_2 = q_pitch.rotate(point);
            assert!((p_1 - p_2).len() < eps);

            // Test roll
            let m_roll = Mat3::roll(angle);
            let q_roll = Quaternion::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), angle);
            let p_1 = m_roll * point;
            let p_2 = q_roll.rotate(point);
            assert!((p_1 - p_2).len() < eps);
        }

        // Test exp and ln
        for _ in 0..100_000 {
            let q = Quaternion::new(
                rng.gen_range(-1.0, 1.0),
                Vec3::new(
                    rng.gen_range(-1.0, 1.0),
                    rng.gen_range(-1.0, 1.0),
                    rng.gen_range(-1.0, 1.0),
                ),
            );
            let q_1 = q.exp().ln();

            assert!((q - q_1).len() < 1.0e-10);
        }
    }
}

//-----------------------------------------------------------------------------
