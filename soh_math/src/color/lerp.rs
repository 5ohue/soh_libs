//-----------------------------------------------------------------------------
use super::convert::{hsluv_to_rgb, hsv_to_rgb, rgb_to_hsluv, rgb_to_hsv};
use super::{Hsluv, Hsv, Rgb};
//-----------------------------------------------------------------------------

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    return a + (b - a) * t;
}

//-----------------------------------------------------------------------------

pub fn lerp_rgb(a: &Rgb, b: &Rgb, t: f64) -> Rgb {
    let r = lerp(a.r as f64, b.r as f64, t).round();
    let g = lerp(a.g as f64, b.g as f64, t).round();
    let b = lerp(a.b as f64, b.b as f64, t).round();

    return Rgb::new(r as u8, g as u8, b as u8);
}

//-----------------------------------------------------------------------------

pub fn lerp_hsv(a: &Hsv, b: &Hsv, t: f64, clockwise: bool, closest: bool) -> Hsv {
    let mut h = lerp(a.h, b.h, t);

    if closest {
        let d1 = (a.h - b.h).abs();
        let d2 = (a.h + 360.0 - b.h).abs();
        let d3 = (a.h - 360.0 - b.h).abs();

        /* Find min */
        if d1 <= d2 && d1 <= d3 {
            h = lerp(a.h, b.h, t);
        } else if d2 <= d1 && d2 <= d3 {
            h = lerp(a.h + 360.0, b.h, t).rem_euclid(360.0);
        } else {
            h = lerp(a.h - 360.0, b.h, t).rem_euclid(360.0);
        }
    } else {
        if clockwise && a.h > b.h {
            h = lerp(a.h, b.h + 360.0, t).rem_euclid(360.0);
        }
        if !clockwise && a.h < b.h {
            h = lerp(a.h + 360.0, b.h, t).rem_euclid(360.0);
        }
    }

    return Hsv::new(h, lerp(a.s, b.s, t), lerp(a.v, b.v, t));
}

pub fn lerp_hsluv(a: &Hsluv, b: &Hsluv, t: f64, clockwise: bool, closest: bool) -> Hsluv {
    let mut h = lerp(a.h, b.h, t);

    if closest {
        let d1 = (a.h - b.h).abs();
        let d2 = (a.h + 360.0 - b.h).abs();
        let d3 = (a.h - 360.0 - b.h).abs();

        /* Find min */
        if d1 <= d2 && d1 <= d3 {
            h = lerp(a.h, b.h, t);
        } else if d2 <= d1 && d2 <= d3 {
            h = lerp(a.h + 360.0, b.h, t).rem_euclid(360.0);
        } else {
            h = lerp(a.h - 360.0, b.h, t).rem_euclid(360.0);
        }
    } else {
        if clockwise && a.h > b.h {
            h = lerp(a.h, b.h + 360.0, t).rem_euclid(360.0);
        }
        if !clockwise && a.h < b.h {
            h = lerp(a.h + 360.0, b.h, t).rem_euclid(360.0);
        }
    }

    return Hsluv::new(h, lerp(a.s, b.s, t), lerp(a.v, b.v, t));
}

//-----------------------------------------------------------------------------
// Linear interpolation by converting to a different color format in between
pub fn lerp_rgb_hsv(a: &Rgb, b: &Rgb, t: f64, clockwise: bool, closest: bool) -> Rgb {
    return hsv_to_rgb(&lerp_hsv(
        &rgb_to_hsv(a),
        &rgb_to_hsv(b),
        t,
        clockwise,
        closest,
    ));
}

pub fn lerp_rgb_hsluv(a: &Rgb, b: &Rgb, t: f64, clockwise: bool, closest: bool) -> Rgb {
    return hsluv_to_rgb(&lerp_hsluv(
        &rgb_to_hsluv(a),
        &rgb_to_hsluv(b),
        t,
        clockwise,
        closest,
    ));
}

//-----------------------------------------------------------------------------
