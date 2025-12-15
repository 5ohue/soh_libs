//-----------------------------------------------------------------------------
use super::{Hsluv, Hsv, Rgb};
use std::cmp::{max, min};
//-----------------------------------------------------------------------------
// `hex_to` functions:
pub fn hex_to_rgb(hex: &str) -> Rgb {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[5..], 16).unwrap_or(0);

    return Rgb::new(r, g, b);
}

pub fn hex_to_hsv(hex: &str) -> Hsv {
    let rgb = hex_to_rgb(hex);
    return rgb_to_hsv(&rgb);
}

pub fn hex_to_hsluv(hex: &str) -> Hsluv {
    let (h, s, l) = hsluv::hex_to_hsluv(hex);
    return Hsluv::new(h, s, l);
}

//-----------------------------------------------------------------------------
// `rgb_to` functions:
pub fn rgb_to_hsv(rgb: &Rgb) -> Hsv {
    let (r, g, b) = (rgb.r as f64, rgb.g as f64, rgb.b as f64);

    let min = min(min(rgb.r, rgb.g), rgb.b) as f64;
    let max = max(max(rgb.r, rgb.g), rgb.b) as f64;

    let h;
    let s = (max - min) / max;
    let v = max / 255.0;

    if min == max {
        return Hsv::new(0.0, 0.0, v);
    }

    if r == max {
        h = (g - b) / (max - min);
    } else if g == max {
        h = 2.0 + (b - r) / (max - min);
    } else {
        h = 4.0 + (r - g) / (max - min);
    }
    let h = (h * 60.0).rem_euclid(360.0);

    return Hsv::new(h, s, v);
}

pub fn rgb_to_hex(rgb: &Rgb) -> String {
    return format!("#{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b);
}

pub fn rgb_to_hsluv(rgb: &Rgb) -> Hsluv {
    let r = rgb.r as f64 / 255.0;
    let g = rgb.g as f64 / 255.0;
    let b = rgb.b as f64 / 255.0;

    let (h, s, l) = hsluv::rgb_to_hsluv((r, g, b));

    return Hsluv::new(h, s, l);
}

//-----------------------------------------------------------------------------
// `hsv_to` functions:
fn hsv_to_rgb_float(hsv: &Hsv) -> (f64, f64, f64) {
    let r;
    let g;
    let b;

    let s = hsv.s.clamp(0.0, 1.0);
    let v = hsv.v.clamp(0.0, 1.0);

    let i = (hsv.h / 60.0).floor();
    let f = hsv.h / 60.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    let i = (i as u8) % 6;
    match i {
        0 => {
            r = v;
            g = t;
            b = p;
        }
        1 => {
            r = q;
            g = v;
            b = p;
        }
        2 => {
            r = p;
            g = v;
            b = t;
        }
        3 => {
            r = p;
            g = q;
            b = v;
        }
        4 => {
            r = t;
            g = p;
            b = v;
        }
        5 => {
            r = v;
            g = p;
            b = q;
        }
        _ => {
            unreachable!();
        }
    }

    let r = r.clamp(0.0, 1.0);
    let g = g.clamp(0.0, 1.0);
    let b = b.clamp(0.0, 1.0);

    return (r, g, b);
}

pub fn hsv_to_rgb(hsv: &Hsv) -> Rgb {
    let (r, g, b) = hsv_to_rgb_float(hsv);
    return Rgb::new(
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    );
}

pub fn hsv_to_hex(hsv: &Hsv) -> String {
    let rgb = hsv_to_rgb(hsv);
    return rgb_to_hex(&rgb);
}

pub fn hsv_to_hsluv(hsv: &Hsv) -> Hsluv {
    let rgb = hsv_to_rgb_float(hsv);
    let (h, s, v) = hsluv::rgb_to_hsluv(rgb);
    return Hsluv::new(h, s, v);
}

//-----------------------------------------------------------------------------
// `hsvluv_to` functions:
pub fn hsluv_to_rgb(hsluv: &Hsluv) -> Rgb {
    let (r, g, b) = hsluv::hsluv_to_rgb((hsluv.h, hsluv.s, hsluv.v));

    return Rgb::new(
        (r.clamp(0.0, 1.0) * 255.0).round() as u8,
        (g.clamp(0.0, 1.0) * 255.0).round() as u8,
        (b.clamp(0.0, 1.0) * 255.0).round() as u8,
    );
}

pub fn hsluv_to_hsv(hsluv: &Hsluv) -> Hsv {
    let rgb = hsluv_to_rgb(hsluv);
    return rgb_to_hsv(&rgb);
}

pub fn hsluv_to_hex(hsluv: &Hsluv) -> String {
    let rgb = hsluv_to_rgb(hsluv);
    return rgb_to_hex(&rgb);
}

//-----------------------------------------------------------------------------
