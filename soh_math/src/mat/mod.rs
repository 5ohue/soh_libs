//-----------------------------------------------------------------------------
mod mat2;
mod mat3;
//-----------------------------------------------------------------------------
pub use mat2::*;
pub use mat3::*;
//-----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use soh_rng::Engine64;

    use crate::*;

    #[test]
    fn test_mat2() {
        // Test that you can construct a matrix using f32
        let m = Mat2::<f32>::identity();
        dbg!(m.det());

        let identity = Mat2::<f64>::identity();
        assert_eq!(identity * identity, identity);

        let v1 = Vec2 { x: 1.0, y: 0.0 };
        let v2 = Vec2 { x: -1.0, y: -1.0 };

        let mat = Mat2::scale(2.0) * Mat2::rot(std::f64::consts::FRAC_PI_2);

        let mv1 = mat * v1;
        let mv2 = mat * v2;

        assert!((mv1 - Vec2 { x: 0.0, y: 2.0 }).len() < 1.0e-10);
        assert!((mv2 - Vec2 { x: 2.0, y: -2.0 }).len() < 1.0e-10);

        let m1 = Mat2::from_rows([[0.0, 1.0], [2.0, 3.0]]);
        let m2 = Mat2::from_rows([[3.0, 1.0], [4.0, 5.0]]);
        assert_eq!(m1 * m2, Mat2::from_rows([[4.0, 5.0], [18.0, 17.0]]));

        let mut rng = soh_rng::RNG64::new(0xdeadbeef);
        for _ in 0..100_000 {
            let mat = Mat2::new([
                rng.gen_range::<f64>(-1.0, 1.0),
                rng.gen_range::<f64>(-1.0, 1.0),
                rng.gen_range::<f64>(-1.0, 1.0),
                rng.gen_range::<f64>(-1.0, 1.0),
            ]);

            // Be careful here
            if mat.det().abs() < 1.0e-10 {
                continue;
            }

            let mm = mat * mat.invert() - identity;
            assert!(mm.norm() < 1.0e-10);
        }
    }

    #[test]
    fn test_mat3() {
        // Test that you can construct a matrix using f32
        let m = Mat3::<f32>::identity();
        dbg!(m.det());

        let identity = Mat3::<f64>::identity();
        assert_eq!(identity * identity, identity);

        let m1 = Mat3::from_rows([[0.0, 1.0, 2.0], [3.0, 4.0, 5.0], [6.0, 7.0, 8.0]]);
        let m2 = Mat3::from_rows([[3.0, 1.0, 4.0], [1.0, 5.0, 9.0], [2.0, 6.0, 5.0]]);

        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let m1v = m1 * v;
        let m2v = m2 * v;

        assert_eq!(
            m1v,
            Vec3 {
                x: 8.0,
                y: 26.0,
                z: 44.0
            }
        );
        assert_eq!(
            m2v,
            Vec3 {
                x: 17.0,
                y: 38.0,
                z: 29.0
            }
        );

        assert_eq!(
            m1 * m2,
            Mat3::from_rows([[5.0, 17.0, 19.0], [23.0, 53.0, 73.0], [41.0, 89.0, 127.0]])
        );

        let mut rng = soh_rng::RNG64::new(0xdeadbeef);
        for _ in 0..100_000 {
            let mat = Mat3::new([
                rng.gen_range::<f64>(-1.0, 1.0),
                rng.gen_range::<f64>(-1.0, 1.0),
                rng.gen_range::<f64>(-1.0, 1.0),
                rng.gen_range::<f64>(-1.0, 1.0),
                rng.gen_range::<f64>(-1.0, 1.0),
                rng.gen_range::<f64>(-1.0, 1.0),
                rng.gen_range::<f64>(-1.0, 1.0),
                rng.gen_range::<f64>(-1.0, 1.0),
                rng.gen_range::<f64>(-1.0, 1.0),
            ]);

            // Be careful here
            if mat.det().abs() < 1.0e-10 {
                continue;
            }

            let mm = mat * mat.invert() - identity;
            assert!(mm.norm() < 1.0e-10);
        }

        // Test rotation matrixes
        for _ in 0..100_000 {
            let yaw = rng.gen_to::<f64>(std::f64::consts::TAU);
            let pitch = rng.gen_to::<f64>(std::f64::consts::TAU);
            let roll = rng.gen_to::<f64>(std::f64::consts::TAU);

            let m_yaw = Mat3::yaw(yaw);
            let m_pitch = Mat3::pitch(pitch);
            let m_roll = Mat3::roll(roll);

            let m_yaw_pitch_roll = Mat3::yaw_pitch_roll(yaw, pitch, roll);

            let (yaw2, pitch2, roll2) = m_yaw_pitch_roll.get_euler_angles();
            let m_yaw_pitch_roll2 = Mat3::yaw_pitch_roll(yaw2, pitch2, roll2);

            assert!((m_yaw_pitch_roll - m_yaw_pitch_roll2).norm() < 1.0e-3);

            assert!((m_yaw * m_pitch * m_roll - m_yaw_pitch_roll).norm() < 1.0e-10);
        }
    }
}

//-----------------------------------------------------------------------------
