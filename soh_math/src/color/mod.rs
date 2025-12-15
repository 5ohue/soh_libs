//-----------------------------------------------------------------------------
pub mod convert;
pub mod lerp;
//-----------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Hsluv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

//-----------------------------------------------------------------------------

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        return Rgb { r, g, b };
    }
}

impl Default for Rgb {
    fn default() -> Self {
        return Rgb { r: 0, g: 0, b: 0 };
    }
}

//-----------------------------------------------------------------------------

impl Hsv {
    pub const fn new(h: f64, s: f64, v: f64) -> Self {
        return Hsv { h, s, v };
    }
}

impl Default for Hsv {
    fn default() -> Self {
        return Hsv {
            h: 0.0,
            s: 0.0,
            v: 0.0,
        };
    }
}

//-----------------------------------------------------------------------------

impl Hsluv {
    pub const fn new(h: f64, s: f64, v: f64) -> Self {
        return Hsluv { h, s, v };
    }
}

impl Default for Hsluv {
    fn default() -> Self {
        return Hsluv {
            h: 0.0,
            s: 0.0,
            v: 0.0,
        };
    }
}

//-----------------------------------------------------------------------------
// Utility colors:
pub const BLACK: Rgb = Rgb::new(0, 0, 0);
pub const WHITE: Rgb = Rgb::new(255, 255, 255);
pub const GRAY: Rgb = Rgb::new(127, 127, 127);

pub const RED: Rgb = Rgb::new(237, 28, 36);
pub const ORANGE: Rgb = Rgb::new(255, 127, 39);
pub const YELLOW: Rgb = Rgb::new(255, 242, 0);
pub const GREEN: Rgb = Rgb::new(34, 177, 76);
pub const BLUE: Rgb = Rgb::new(0, 128, 255);

pub const LIGHT_RED: Rgb = Rgb::new(255, 128, 128);
pub const LIGHT_ORANGE: Rgb = Rgb::new(255, 201, 14);
pub const LIGHT_YELLOW: Rgb = Rgb::new(239, 228, 176);
pub const LIGHT_GREEN: Rgb = Rgb::new(181, 230, 29);
pub const LIGHT_BLUE: Rgb = Rgb::new(0, 162, 232);

pub const DARK_RED: Rgb = Rgb::new(136, 0, 21);
pub const DARK_ORANGE: Rgb = Rgb::new(128, 64, 0);
pub const DARK_YELLOW: Rgb = Rgb::new(128, 128, 0);
pub const DARK_GREEN: Rgb = Rgb::new(0, 128, 0);
pub const DARK_BLUE: Rgb = Rgb::new(63, 72, 204);
//-----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    static HEXES: [&str; 13] = [
        "#FFFFFF", "#000000", "#00FF00", "#FF0000", "#0000FF", "#7F7FFE", "#FE7F00", "#018d60",
        "#ec008c", "#2a3ab5", "#c0fe7f", "#3c8eb8", "#948096",
    ];
    static RGBS: [Rgb; 13] = [
        Rgb::new(255, 255, 255),
        Rgb::new(0, 0, 0),
        Rgb::new(0, 255, 0),
        Rgb::new(255, 0, 0),
        Rgb::new(0, 0, 255),
        Rgb::new(127, 127, 254),
        Rgb::new(254, 127, 0),
        Rgb::new(1, 141, 96),
        Rgb::new(236, 0, 140),
        Rgb::new(42, 58, 181),
        Rgb::new(192, 254, 127),
        Rgb::new(60, 142, 184),
        Rgb::new(148, 128, 150),
    ];
    static HSVS: [Hsv; 13] = [
        Hsv::new(0.0, 0.0, 1.0),
        Hsv::new(0.0, 0.0, 0.0),
        Hsv::new(120.0, 1.0, 1.0),
        Hsv::new(0.0, 1.0, 1.0),
        Hsv::new(240.0, 1.0, 1.0),
        Hsv::new(240.0, 0.5, 0.996),
        Hsv::new(30.0, 1.0, 0.996),
        Hsv::new(160.7, 0.993, 0.553),
        Hsv::new(324.4, 1.0, 0.925),
        Hsv::new(233.1, 0.768, 0.71),
        Hsv::new(89.3, 0.5, 0.996),
        Hsv::new(200.3, 0.674, 0.722),
        Hsv::new(294.5, 0.147, 0.588),
    ];

    //-----------------------------------------------------------------------------
    // Compare by three decimals
    fn eps_cmp(a: f64, b: f64, eps: f64) -> bool {
        let delta = a - b;
        return delta.abs() < eps;
    }

    //-----------------------------------------------------------------------------

    #[test]
    fn check_hex_rgb() {
        HEXES.iter().zip(RGBS.iter()).for_each(|(hex, rgb)| {
            assert_eq!(convert::hex_to_rgb(hex), *rgb);
            assert_eq!(convert::rgb_to_hex(rgb).to_lowercase(), hex.to_lowercase());
        });
    }

    #[test]
    fn check_rgb_hsv() {
        RGBS.iter().zip(HSVS.iter()).for_each(|(rgb, hsv)| {
            let rgb_1 = convert::hsv_to_rgb(hsv);
            let hsv_1 = convert::rgb_to_hsv(rgb);

            dbg!(hsv_1);
            dbg!(hsv);

            assert_eq!(*rgb, rgb_1);
            assert!(eps_cmp(hsv_1.h, hsv.h, 1e-1));
            assert!(eps_cmp(hsv_1.s, hsv.s, 1e-3));
            assert!(eps_cmp(hsv_1.v, hsv.v, 1e-3));
        });
    }

    #[test]
    fn check_hsv_hex() {
        HEXES.iter().zip(HSVS.iter()).for_each(|(hex, hsv)| {
            let hex_1 = convert::hsv_to_hex(hsv);
            let hsv_1 = convert::hex_to_hsv(hex);

            assert_eq!(hex.to_lowercase(), hex_1.to_lowercase());
            assert!(eps_cmp(hsv_1.h, hsv.h, 1e-1));
            assert!(eps_cmp(hsv_1.s, hsv.s, 1e-3));
            assert!(eps_cmp(hsv_1.v, hsv.v, 1e-3));
        });
    }

    #[test]
    fn check_hsluv() {
        HEXES
            .iter()
            .zip(RGBS.iter())
            .zip(HSVS.iter())
            .for_each(|((hex, rgb), hsv)| {
                let hsluv = convert::hex_to_hsluv(hex);
                let hex_1 = convert::hsluv_to_hex(&hsluv);
                assert_eq!(hex.to_lowercase(), hex_1.to_lowercase());
                let hsluv = convert::rgb_to_hsluv(rgb);
                let rgb_1 = convert::hsluv_to_rgb(&hsluv);
                assert_eq!(*rgb, rgb_1);
                let hsluv = convert::hsv_to_hsluv(hsv);
                let hsv_1 = convert::hsluv_to_hsv(&hsluv);
                assert!(eps_cmp(hsv_1.h, hsv.h, 1e-1));
                assert!(eps_cmp(hsv_1.s, hsv.s, 1e-3));
                assert!(eps_cmp(hsv_1.v, hsv.v, 1e-3));
            });
    }
}

//-----------------------------------------------------------------------------
