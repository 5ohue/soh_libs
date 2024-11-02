//-----------------------------------------------------------------------------
mod vec2;
mod vec3;
mod vec4;
//-----------------------------------------------------------------------------
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
//-----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2() {
        let v1 = Vec2 { x: 11.0, y: 30.0 };
        let v2 = Vec2 { x: 19.5, y: -9.5 };

        assert_eq!(v1 + v2, Vec2 { x: 30.5, y: 20.5 });
        assert_eq!(v1 - v2, Vec2 { x: -8.5, y: 39.5 });
        assert_eq!(v1 * 2.0, Vec2 { x: 22.0, y: 60.0 });
        assert_eq!(v2 * 2.0, Vec2 { x: 39.0, y: -19.0 });
        assert_eq!(v1 / 2.0, Vec2 { x: 5.5, y: 15.0 });
        assert_eq!(v2 / 2.0, Vec2 { x: 9.75, y: -4.75 });
    }

    #[test]
    fn test_vec3() {
        let v1 = Vec3 { x: 11.0, y: 30.0, z: -13.0 };
        let v2 = Vec3 { x: 19.5, y: -9.5, z: 9.5 };

        assert_eq!(v1 + v2, Vec3 { x: 30.5, y: 20.5, z: -3.5 });
        assert_eq!(v1 - v2, Vec3 { x: -8.5, y: 39.5, z: -22.5 });
        assert_eq!(v1 * 2.0, Vec3 { x: 22.0, y: 60.0, z: -26.0 });
        assert_eq!(v2 * 2.0, Vec3 { x: 39.0, y: -19.0, z: 19.0 });
        assert_eq!(v1 / 2.0, Vec3 { x: 5.5, y: 15.0, z: -6.5 });
        assert_eq!(v2 / 2.0, Vec3 { x: 9.75, y: -4.75, z: 4.75 });
    }

    #[test]
    fn test_vec4() {
        let v1 = Vec4 { x: 11.0, y: 30.0, z: -13.0, w: 2.5 };
        let v2 = Vec4 { x: 19.5, y: -9.5, z: 9.5, w: -2.5 };

        assert_eq!(v1 + v2, Vec4 { x: 30.5, y: 20.5, z: -3.5, w: 0.0 });
        assert_eq!(v1 - v2, Vec4 { x: -8.5, y: 39.5, z: -22.5, w: 5.0 });
        assert_eq!(v1 * 2.0, Vec4 { x: 22.0, y: 60.0, z: -26.0, w: 5.0 });
        assert_eq!(v2 * 2.0, Vec4 { x: 39.0, y: -19.0, z: 19.0, w: -5.0 });
        assert_eq!(v1 / 2.0, Vec4 { x: 5.5, y: 15.0, z: -6.5, w: 1.25 });
        assert_eq!(v2 / 2.0, Vec4 { x: 9.75, y: -4.75, z: 4.75, w: -1.25 });
    }
}
