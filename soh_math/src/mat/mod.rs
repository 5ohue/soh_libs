//-----------------------------------------------------------------------------
mod mat2;
mod mat3;
mod mat4;
//-----------------------------------------------------------------------------
pub use mat2::*;
pub use mat3::*;
pub use mat4::*;
//-----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use soh_rng::Engine64;

    use crate::*;

    #[test]
    fn test_mat2() {
        // Test that you can construct a matrix using f32
        let m = Mat2::<f32>::identity();
        assert_eq!(m.det(), 1.0);

        // Test multiplication (trivial)
        let identity = Mat2::<f64>::identity();
        assert_eq!(identity * identity, identity);

        // Test vector multiplication
        let v1 = Vec2::new(1.0, 0.0);
        let v2 = Vec2::new(-1.0, -1.0);

        let mat = Mat2::scale(2.0) * Mat2::rot(std::f64::consts::FRAC_PI_2);

        let mv1 = mat * v1;
        let mv2 = mat * v2;

        assert!((mv1 - Vec2::new(0.0, 2.0)).len() < 1.0e-10);
        assert!((mv2 - Vec2::new(2.0, -2.0)).len() < 1.0e-10);

        // Test matrix multiplication
        let m1 = Mat2::from_rows([Vec2::new(0.0, 1.0), Vec2::new(2.0, 3.0)]);
        let m2 = Mat2::from_rows([Vec2::new(3.0, 1.0), Vec2::new(4.0, 5.0)]);
        assert_eq!(
            m1 * m2,
            Mat2::from_rows([Vec2::new(4.0, 5.0), Vec2::new(18.0, 17.0)])
        );

        // Test matrix inversion
        let mut rng = soh_rng::RNG64::new(0xdeadbeef);
        for _ in 0..100_000 {
            let mat = Mat2::new([
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
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
        fn matrix_delta<T>(mat1: Mat3<T>, mat2: Mat3<T>) -> T
        where
            T: num_traits::Float + std::iter::Sum + From<f32>,
        {
            return (mat1 - mat2).norm();
        }

        // Test that you can construct a matrix using f32
        let m = Mat3::<f32>::identity();
        assert_eq!(m.det(), 1.0);

        // Test multiplication (trivial)
        let identity = Mat3::<f64>::identity();
        assert_eq!(identity * identity, identity);

        // Test vector multiplication
        let m1 = Mat3::from_rows([
            Vec3::new(0.0, 1.0, 2.0),
            Vec3::new(3.0, 4.0, 5.0),
            Vec3::new(6.0, 7.0, 8.0),
        ]);
        let m2 = Mat3::from_rows([
            Vec3::new(3.0, 1.0, 4.0),
            Vec3::new(1.0, 5.0, 9.0),
            Vec3::new(2.0, 6.0, 5.0),
        ]);

        let v = Vec3::new(1.0, 2.0, 3.0);
        let m1v = m1 * v;
        let m2v = m2 * v;

        assert_eq!(m1v, Vec3::new(8.0, 26.0, 44.0));
        assert_eq!(m2v, Vec3::new(17.0, 38.0, 29.0));

        // Test vector rotation
        let m_yaw = Mat3::yaw(std::f64::consts::FRAC_PI_2);
        let m_pitch = Mat3::pitch(std::f64::consts::FRAC_PI_2);
        let m_roll = Mat3::roll(std::f64::consts::FRAC_PI_2);
        let m_yaw_pitch_roll = Mat3::yaw_pitch_roll(
            std::f64::consts::FRAC_PI_2,
            std::f64::consts::FRAC_PI_2,
            std::f64::consts::FRAC_PI_2,
        );

        let v_orig = Vec3::new(1.0, 2.0, 3.0);
        let mut v = v_orig;
        v = m_roll * v;
        assert!((v - Vec3::new(1.0, -3.0, 2.0)).len() < 1.0e-10);
        v = m_pitch * v;
        assert!((v - Vec3::new(2.0, -3.0, -1.0)).len() < 1.0e-10);
        v = m_yaw * v;
        assert!((v - m_yaw_pitch_roll * v_orig).len() < 1.0e-10);

        // Test matrix multiplication
        assert_eq!(
            m1 * m2,
            Mat3::from_rows([
                Vec3::new(5.0, 17.0, 19.0),
                Vec3::new(23.0, 53.0, 73.0),
                Vec3::new(41.0, 89.0, 127.0)
            ])
        );

        // Test matrix inversion
        let mut rng = soh_rng::RNG64::new(0xdeadbeef);
        for _ in 0..100_000 {
            let mat = Mat3::new([
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
            ]);

            // Be careful here
            if mat.det().abs() < 1.0e-10 {
                continue;
            }

            let mm = mat * mat.invert() - identity;
            assert!(mm.norm() < 1.0e-10);

            let mm = mat * mat.invert_no_det() / mat.det() - identity;
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

            assert!(matrix_delta(m_yaw_pitch_roll, m_yaw_pitch_roll2) < 1.0e-3);

            assert!(matrix_delta(m_yaw * m_pitch * m_roll, m_yaw_pitch_roll) < 1.0e-10);
        }

        // Test LookAt matrixes
        let m = Mat3::look_at(
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::zero(),
            Vec3::new(0.0, 1.0, 0.0),
        );
        assert_eq!(m, Mat3::identity());

        // Test axis rotation
        for _ in 0..100_000 {
            let eps = 1.0e-10;
            let angle = rng.gen_to(std::f64::consts::TAU);

            // Test yaw
            let m_yaw_1 = Mat3::yaw(angle);
            let m_yaw_2 = Mat3::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle);
            assert!(matrix_delta(m_yaw_1, m_yaw_2) < eps);

            // Test pitch
            let m_pitch_1 = Mat3::pitch(angle);
            let m_pitch_2 = Mat3::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), angle);
            assert!(matrix_delta(m_pitch_1, m_pitch_2) < eps);

            // Test roll
            let m_roll_1 = Mat3::roll(angle);
            let m_roll_2 = Mat3::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), angle);
            assert!(matrix_delta(m_roll_1, m_roll_2) < eps);
        }

        for _ in 0..100_000 {
            let axis: Vec3<f64> = Vec3::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
            );

            let angle = rng.gen_to(std::f64::consts::TAU);

            let point: Vec3<f64> = Vec3::new(
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
                rng.gen_range(-1.0, 1.0),
            );

            let q_rotation = Quaternion::from_axis_angle(axis, angle);

            let m_rotation = Mat3::from_axis_angle(axis, angle);
            let m_rotation_quat = Mat3::from_quat(q_rotation);

            let p_1 = q_rotation.rotate(point);
            let p_2 = m_rotation * point;

            assert!((p_1 - p_2).len() < 1.0e-10);
            assert!(matrix_delta(m_rotation, m_rotation_quat) < 1.0e-10);
        }
    }

    #[test]
    fn test_mat4() {
        // Test that you can construct a matrix using f32
        let m = Mat4::<f32>::identity();
        assert_eq!(m.det(), 1.0);

        // Test multiplication (trivial)
        let identity = Mat4::<f64>::identity();
        assert_eq!(identity * identity, identity);

        let m1 = Mat4::from_rows([
            Vec4::new( 1, -2,  3, 8),
            Vec4::new( 7, -2,  9, 0),
            Vec4::new( 2,  4, -1, 5),
            Vec4::new(-6,  2,  0, 9),
        ]);
        let m2 = Mat4::from_rows([
            Vec4::new( 5,  0, -1, 2),
            Vec4::new( 0,  6,  2, 2),
            Vec4::new(-3, -3,  1, 2),
            Vec4::new( 0,  6,  1, 5),
        ]);
        let m1xm2 = Mat4::from_rows([
            Vec4::new( -4,  27,  6, 44),
            Vec4::new(  8, -39, -2, 28),
            Vec4::new( 13,  57, 10, 35),
            Vec4::new(-30,  66, 19, 37),
        ]);
        assert_eq!(m1 * m2, m1xm2);

        let mut rng = soh_rng::RNG64::new(0xdeadbeef);

        // Test determinants
        let m = Mat4::from_rows([
            Vec4::new(3, 1, 4, 1),
            Vec4::new(5, 9, 2, 6),
            Vec4::new(5, 3, 5, 8),
            Vec4::new(9, 7, 9, 3),
        ]);
        assert_eq!(m.det(), 98);

        for _ in 0..100_000 {
            let factor = rng.gen_range::<f32>(-10.0, 10.0);

            let m = Mat4::scale(factor);
            assert!((m.det() - factor.powi(4)).abs() < 1.0e-3);
        }

        // Test matrix inversion
        assert!(Mat4::<f32>::identity().invert() == Mat4::identity());
        assert!(Mat4::scale(2.0).invert() == Mat4::scale(0.5));

        for _ in 0..100_000 {
            let mat = Mat4::new([
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
                rng.gen_range::<f64>(-5.0, 5.0),
            ]);

            // Be careful here
            if mat.det().abs() < 1.0e-10 {
                continue;
            }

            let mm = mat * mat.invert() - identity;
            assert!(mm.norm() < 1.0e-10);

            let mm = mat * mat.invert_no_det() / mat.det() - identity;
            assert!(mm.norm() < 1.0e-10);
        }
    }
}

//-----------------------------------------------------------------------------
