use super::model::{ColorFamily, Hsl, Rgb};

pub fn hex_to_rgb(hex: &str) -> Option<Rgb> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(Rgb { r, g, b })
}

pub fn rgb_to_hsl(rgb: &Rgb) -> Hsl {
    let r = rgb.r as f32 / 255.0;
    let g = rgb.g as f32 / 255.0;
    let b = rgb.b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;

    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };

    Hsl {
        h,
        s: s * 100.0,
        l: l * 100.0,
    }
}

pub fn classify_family(hsl: &Hsl) -> ColorFamily {
    // Handle neutrals (low saturation or extreme lightness)
    if hsl.s < 10.0 || hsl.l < 5.0 || hsl.l > 95.0 {
        return ColorFamily::Neutral;
    }

    // Handle browns (low saturation + orange-red hue range + darker)
    if hsl.s < 50.0 && hsl.l < 50.0 && (hsl.h < 40.0 || hsl.h > 340.0) {
        return ColorFamily::Brown;
    }

    // Classify by hue angle
    match hsl.h {
        h if h < 15.0 => ColorFamily::Red,
        h if h < 45.0 => ColorFamily::Orange,
        h if h < 70.0 => ColorFamily::Yellow,
        h if h < 150.0 => ColorFamily::Green,
        h if h < 190.0 => ColorFamily::Cyan,
        h if h < 260.0 => ColorFamily::Blue,
        h if h < 290.0 => ColorFamily::Purple,
        h if h < 340.0 => ColorFamily::Pink,
        _ => ColorFamily::Red,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#ff0000"), Some(Rgb { r: 255, g: 0, b: 0 }));
        assert_eq!(hex_to_rgb("00ff00"), Some(Rgb { r: 0, g: 255, b: 0 }));
        assert_eq!(hex_to_rgb("#fff"), None);
    }

    #[test]
    fn test_classify_family() {
        let red = Hsl {
            h: 0.0,
            s: 100.0,
            l: 50.0,
        };
        assert_eq!(classify_family(&red), ColorFamily::Red);

        let blue = Hsl {
            h: 220.0,
            s: 100.0,
            l: 50.0,
        };
        assert_eq!(classify_family(&blue), ColorFamily::Blue);

        let gray = Hsl {
            h: 0.0,
            s: 0.0,
            l: 50.0,
        };
        assert_eq!(classify_family(&gray), ColorFamily::Neutral);
    }
}
