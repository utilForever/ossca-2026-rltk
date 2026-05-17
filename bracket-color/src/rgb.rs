use crate::prelude::{XpColor, HSV, RGBA};
use std::convert::From;
use std::ops;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Copy, Clone, Default, Debug)]
/// Represents an R/G/B triplet, in the range 0..1 (32-bit float)
pub struct RGB {
    /// The red component (0..1)
    pub r: f32,
    /// The green components (0..1)
    pub g: f32,
    /// The blue component (0..1)
    pub b: f32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// Error message type when failing to convert a hex code to RGB.
pub enum HtmlColorConversionError {
    /// The HTML string was not a valid length. (Expects #AABBCC)
    InvalidStringLength,
    /// No # was included in the string.
    MissingHash,
    /// An unexpected character (not #, A-F) was detected in the color string.
    InvalidCharacter,
}

// Implement operator overloading

/// Support adding a float to a color. The result is clamped via the constructor.
impl ops::Add<f32> for RGB {
    type Output = Self;
    fn add(mut self, rhs: f32) -> Self {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
        self
    }
}

/// Support adding an RGB to a color. The result is clamped via the constructor.
impl ops::Add<RGB> for RGB {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self
    }
}

/// Support subtracting a float from a color. The result is clamped via the constructor.
impl ops::Sub<f32> for RGB {
    type Output = Self;
    fn sub(mut self, rhs: f32) -> Self {
        self.r -= rhs;
        self.g -= rhs;
        self.b -= rhs;
        self
    }
}

/// Support subtracting an RGB from a color. The result is clamped via the constructor.
impl ops::Sub<RGB> for RGB {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self
    }
}

/// Support multiplying a color by a float. The result is clamped via the constructor.
impl ops::Mul<f32> for RGB {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self
    }
}

/// Support multiplying a color by another color. The result is clamped via the constructor.
impl ops::Mul<RGB> for RGB {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self
    }
}

/// Support conversion from a color tuple
impl From<(u8, u8, u8)> for RGB {
    fn from(vals: (u8, u8, u8)) -> Self {
        Self::named(vals)
    }
}

/// Support conversion from HSV
impl From<HSV> for RGB {
    fn from(hsv: HSV) -> Self {
        hsv.to_rgb()
    }
}

/// Support conversion from RGBA
impl From<RGBA> for RGB {
    fn from(item: RGBA) -> Self {
        Self::from_f32(item.r, item.g, item.b)
    }
}

// Support conversion from Bevy
#[cfg(feature = "bevy")]
impl From<bevy::prelude::Color> for RGB {
    fn from(item: bevy::prelude::Color) -> Self {
        let srgba = item.to_srgba();
        Self::from_f32(srgba.red, srgba.green, srgba.blue)
    }
}

#[cfg(feature = "bevy")]
impl From<RGB> for bevy::prelude::Color {
    fn from(item: RGB) -> Self {
        Self::srgb(item.r, item.g, item.b)
    }
}

impl RGB {
    /// Constructs a new, zeroed (black) RGB triplet.
    #[must_use]
    pub fn new() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    /// Constructs a new RGB color, from 3 32-bit floats in the range 0..1
    ///
    /// # Arguments
    ///
    /// * `r` - the red component (0..1)
    /// * `g` - the green component (0..1)
    /// * `b` - the blue component (0..1)
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// let red = RGB::from_f32(1.0, 0.0, 0.0);
    /// let green = RGB::from_f32(0.0, 1.0, 0.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_f32(r: f32, g: f32, b: f32) -> Self {
        let r_clamped = r.clamp(0.0, 1.0);
        let g_clamped = g.clamp(0.0, 1.0);
        let b_clamped = b.clamp(0.0, 1.0);
        Self {
            r: r_clamped,
            g: g_clamped,
            b: b_clamped,
        }
    }

    /// Constructs a new RGB color, from 3 bytes in the range 0..255
    ///
    /// # Arguments
    ///
    /// * `r` - the red component, ranged from 0 to 255
    /// * `g` - the green component, ranged from 0 to 255
    /// * `b` - the blue component, ranged from 0 to 255
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// let red = RGB::from_u8(255, 0, 0);
    /// let green = RGB::from_u8(0, 255, 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
        }
    }

    /// Construct an RGB color from a tuple of u8, or a named constant
    ///
    /// # Arguments
    ///
    /// * `col` a tuple of three `u8` values. See `from_u8`. These are usually provided from the `named` colors list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// let red = RGB::named(RED);
    /// let green = RGB::named((0, 255, 0));
    /// ```
    #[inline]
    #[must_use]
    pub fn named(col: (u8, u8, u8)) -> Self {
        Self::from_u8(col.0, col.1, col.2)
    }

    /// Constructs from an HTML color code (e.g. "#eeffee")
    ///
    /// # Arguments
    ///
    /// * `code` - an HTML color notation (e.g. "#ffeeff")
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// let red = RGB::from_hex("#FF0000");
    /// let green = RGB::from_hex("#00FF00");
    /// ```
    ///
    /// # Errors
    ///
    /// See `HtmlColorConversionError`
    #[allow(clippy::cast_precision_loss)]
    pub fn from_hex<S: AsRef<str>>(code: S) -> Result<Self, HtmlColorConversionError> {
        let mut full_code = code.as_ref().chars();

        if let Some(hash) = full_code.next() {
            if hash != '#' {
                return Err(HtmlColorConversionError::MissingHash);
            }
        } else {
            return Err(HtmlColorConversionError::InvalidStringLength);
        }

        let red1 = match full_code.next() {
            Some(red) => match red.to_digit(16) {
                Some(red) => red * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let red2 = match full_code.next() {
            Some(red) => match red.to_digit(16) {
                Some(red) => red,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };

        let green1 = match full_code.next() {
            Some(green) => match green.to_digit(16) {
                Some(green) => green * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let green2 = match full_code.next() {
            Some(green) => match green.to_digit(16) {
                Some(green) => green,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };

        let blue1 = match full_code.next() {
            Some(blue) => match blue.to_digit(16) {
                Some(blue) => blue * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let blue2 = match full_code.next() {
            Some(blue) => match blue.to_digit(16) {
                Some(blue) => blue,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };

        if full_code.next().is_some() {
            return Err(HtmlColorConversionError::InvalidStringLength);
        }

        Ok(Self {
            r: (red1 + red2) as f32 / 255.0,
            g: (green1 + green2) as f32 / 255.0,
            b: (blue1 + blue2) as f32 / 255.0,
        })
    }

    #[inline]
    #[must_use]
    pub fn from_xp(xp: XpColor) -> Self {
        Self::from_u8(xp.r, xp.g, xp.b)
    }

    /// Converts an RGB triple to an HSV triple.
    #[allow(clippy::many_single_char_names)]
    #[must_use]
    pub fn to_hsv(&self) -> HSV {
        let r = self.r;
        let g = self.g;
        let b = self.b;

        let max = f32::max(f32::max(r, g), b);
        let min = f32::min(f32::min(r, g), b);

        let mut h: f32 = max;
        let v: f32 = max;

        let d = max - min;
        let s = if max == 0.0 { 0.0 } else { d / max };

        if (max - min).abs() < f32::EPSILON {
            h = 0.0; // Achromatic
        } else {
            if (max - r).abs() < f32::EPSILON {
                if g < b {
                    h = (g - b) / d + 6.0;
                } else {
                    h = (g - b) / d;
                }
            } else if (max - g).abs() < f32::EPSILON {
                h = (b - r) / d + 2.0;
            } else if (max - b).abs() < f32::EPSILON {
                h = (r - g) / d + 4.0;
            }

            h /= 6.0;
        }

        HSV::from_f32(h, s, v)
    }

    /// Converts an RGB to an RGBA
    #[inline]
    #[must_use]
    pub fn to_rgba(&self, alpha: f32) -> RGBA {
        RGBA::from_f32(self.r, self.g, self.b, alpha)
    }

    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn to_xp(&self) -> XpColor {
        XpColor::new(
            (self.r.clamp(0.0, 1.0) * 255.0) as u8,
            (self.g.clamp(0.0, 1.0) * 255.0) as u8,
            (self.b.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }

    /// Applies a quick grayscale conversion to the color
    #[inline]
    #[must_use]
    pub fn to_greyscale(&self) -> Self {
        let linear = (self.r * 0.2126) + (self.g * 0.7152) + (self.b * 0.0722);
        Self::from_f32(linear, linear, linear)
    }

    /// Applies a lengthier desaturate (via HSV) to the color
    #[inline]
    #[must_use]
    pub fn desaturate(&self) -> Self {
        let mut hsv = self.to_hsv();
        hsv.s = 0.0;
        hsv.to_rgb()
    }

    /// Lerps by a specified percentage (from 0 to 1) between this color and another
    #[inline]
    #[must_use]
    pub fn lerp(&self, color: Self, percent: f32) -> Self {
        let range = (color.r - self.r, color.g - self.g, color.b - self.b);
        Self {
            r: self.r + range.0 * percent,
            g: self.g + range.1 * percent,
            b: self.b + range.2 * percent,
        }
    }
}

#[cfg(feature = "crossterm")]
mod crossterm_features {
    use super::RGB;
    use crossterm::style::Color;
    use std::convert::TryFrom;

    impl TryFrom<RGB> for Color {
        type Error = &'static str;

        fn try_from(rgb: RGB) -> Result<Self, Self::Error> {
            let (r, g, b) = (rgb.r, rgb.g, rgb.b);
            for c in [r, g, b].iter() {
                if *c < 0.0 {
                    return Err("Value < 0.0 found!");
                }
                if *c > 1.0 {
                    return Err("Value > 1.0 found!");
                }
            }
            let (r, g, b) = ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8);
            let rgb = Color::Rgb { r, g, b };
            Ok(rgb)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::prelude::RGB;
        use crossterm::style::Color;
        use std::convert::TryInto;

        #[test]
        fn basic_conversion() {
            let rgb = RGB {
                r: 0.0,
                g: 0.5,
                b: 1.0,
            };
            let rgb: Color = rgb.try_into().unwrap();
            match rgb {
                Color::Rgb { r, g, b } => {
                    assert_eq!(r, 0);
                    assert_eq!(g, 127);
                    assert_eq!(b, 255);
                }
                _ => unreachable!(),
            }
        }

        #[test]
        fn negative_rgb() {
            let rgb = RGB {
                r: 0.0,
                g: 0.5,
                b: -1.0,
            };
            let rgb: Result<Color, _> = rgb.try_into();
            assert!(rgb.is_err());
        }

        #[test]
        fn too_large_rgb() {
            let rgb = RGB {
                r: 0.0,
                g: 0.5,
                b: 1.1,
            };
            let rgb: Result<Color, _> = rgb.try_into();
            assert!(rgb.is_err());
        }
    }
}

// Unit tests for the color system

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_utils::*;
    use rstest::rstest;

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    fn make_rgb_minimal() {
        let black = RGB::new();
        assert!(black.r < f32::EPSILON);
        assert!(black.g < f32::EPSILON);
        assert!(black.b < f32::EPSILON);
    }

    #[test]
    fn new_rgb_is_black() {
        assert_rgb_eq(RGB::new(), 0.0, 0.0, 0.0);
    }

    #[test]
    fn from_f32_clamps_components() {
        let rgb = RGB::from_f32(-1.0, 0.5, 2.0);
        assert_rgb_eq(rgb, 0.0, 0.5, 1.0);
    }

    #[test]
    fn from_u8_converts_components() {
        let rgb = RGB::from_u8(255, 128, 0);
        assert_rgb_eq(rgb, 1.0, 128.0 / 255.0, 0.0);
    }

    #[test]
    fn tuple_conversion_uses_u8_components() {
        let rgb = RGB::from((64, 128, 255));
        assert_rgb_eq(rgb, 64.0 / 255.0, 128.0 / 255.0, 1.0);
    }

    #[test]
    fn rgba_conversion_drops_alpha() {
        let rgb = RGB::from(RGBA::from_f32(0.25, 0.5, 0.75, 0.125));
        assert_rgb_eq(rgb, 0.25, 0.5, 0.75);
    }

    #[test]
    fn hsv_conversion_delegates_to_hsv_to_rgb() {
        let rgb = RGB::from(HSV::from_f32(120.0 / 360.0, 1.0, 1.0));
        assert_rgb_eq(rgb, 0.0, 1.0, 0.0);
    }

    #[rstest]
    #[case(RGB::from_f32(0.25, 0.5, 0.75) + 0.125, 0.375, 0.625, 0.875)]
    #[case(
        RGB::from_f32(0.25, 0.5, 0.75) + RGB::from_f32(0.125, 0.25, 0.125),
        0.375,
        0.75,
        0.875
    )]
    #[case(RGB::from_f32(0.25, 0.5, 0.75) - 0.125, 0.125, 0.375, 0.625)]
    #[case(
        RGB::from_f32(0.25, 0.5, 0.75) - RGB::from_f32(0.125, 0.25, 0.125),
        0.125,
        0.25,
        0.625
    )]
    #[case(RGB::from_f32(0.25, 0.5, 0.75) * 0.5, 0.125, 0.25, 0.375)]
    #[case(
        RGB::from_f32(0.25, 0.5, 0.75) * RGB::from_f32(0.125, 0.25, 0.125),
        0.03125,
        0.125,
        0.09375
    )]
    fn arithmetic_operators_apply_component_wise(
        #[case] rgb: RGB,
        #[case] r: f32,
        #[case] g: f32,
        #[case] b: f32,
    ) {
        assert_rgb_eq(rgb, r, g, b);
    }

    #[rstest]
    #[case(RGB::from_f32(1.0, 0.0, 0.0), 0.0, 1.0, 1.0)]
    #[case(RGB::from_f32(0.0, 1.0, 0.0), 120.0 / 360.0, 1.0, 1.0)]
    #[case(RGB::from_f32(0.0, 0.0, 1.0), 240.0 / 360.0, 1.0, 1.0)]
    fn convert_primary_colors_to_hsv(
        #[case] rgb: RGB,
        #[case] h: f32,
        #[case] s: f32,
        #[case] v: f32,
    ) {
        assert_hsv_eq(rgb.to_hsv(), h, s, v);
    }

    #[rstest]
    #[case(RGB::from_f32(0.0, 0.0, 0.0), 0.0, 0.0, 0.0)]
    #[case(RGB::from_f32(1.0, 1.0, 1.0), 0.0, 0.0, 1.0)]
    #[case(RGB::from_f32(0.5, 0.5, 0.5), 0.0, 0.0, 0.5)]
    fn convert_grayscale_to_hsv(#[case] rgb: RGB, #[case] h: f32, #[case] s: f32, #[case] v: f32) {
        assert_hsv_eq(rgb.to_hsv(), h, s, v);
    }

    #[test]
    fn convert_olive_to_hsv() {
        let rgb = RGB::from_u8(128, 128, 0);
        let hsv = rgb.to_hsv();

        assert_hsv_eq(hsv, 60.0 / 360.0, 1.0, 128.0 / 255.0);
    }

    #[test]
    fn convert_magenta_to_hsv_wraps_red_hue() {
        let rgb = RGB::from_f32(1.0, 0.0, 0.5);
        let hsv = rgb.to_hsv();

        assert_hsv_eq(hsv, 330.0 / 360.0, 1.0, 1.0);
    }

    #[rstest]
    #[case(0.0)]
    #[case(0.5)]
    #[case(1.0)]
    fn convert_rgb_to_rgba_preserves_rgb_and_sets_alpha(#[case] alpha: f32) {
        let rgba = RGB::from_f32(1.0, 0.0, 0.0).to_rgba(alpha);

        assert_rgba_eq(rgba, 1.0, 0.0, 0.0, alpha);
    }

    #[test]
    fn xp_color_round_trip_preserves_bytes() {
        let xp = XpColor::new(17, 128, 255);
        let rgb = RGB::from_xp(xp);
        assert_rgb_eq(rgb, 17.0 / 255.0, 128.0 / 255.0, 1.0);

        let converted = rgb.to_xp();
        assert_eq!(converted.r, 17);
        assert_eq!(converted.g, 128);
        assert_eq!(converted.b, 255);
    }

    #[test]
    fn to_xp_clamps_out_of_range_components() {
        let rgb = RGB {
            r: -0.5,
            g: 0.5,
            b: 1.5,
        };
        let xp = rgb.to_xp();

        assert_eq!(xp.r, 0);
        assert_eq!(xp.g, 127);
        assert_eq!(xp.b, 255);
    }

    #[rstest]
    #[case("#FF0000", 1.0, 0.0, 0.0)]
    #[case("#00FF00", 0.0, 1.0, 0.0)]
    #[case("#0000FF", 0.0, 0.0, 1.0)]
    #[case("#808000", 128.0 / 255.0, 128.0 / 255.0, 0.0)]
    fn parse_hex_colors(#[case] hex: &str, #[case] r: f32, #[case] g: f32, #[case] b: f32) {
        let rgb = RGB::from_hex(hex).expect("valid hex color");
        assert_rgb_eq(rgb, r, g, b);
    }

    #[test]
    fn parse_hex_rejects_missing_hash() {
        let err = RGB::from_hex("FF0000").unwrap_err();
        assert_eq!(err, HtmlColorConversionError::MissingHash);
    }

    #[rstest]
    #[case("")]
    #[case("#")]
    #[case("#F")]
    #[case("#FF")]
    #[case("#FFF")]
    #[case("#FFFF")]
    #[case("#FFFFF")]
    #[case("#FFFFFF00")]
    fn parse_hex_rejects_invalid_length(#[case] hex: &str) {
        let err = RGB::from_hex(hex).unwrap_err();
        assert_eq!(err, HtmlColorConversionError::InvalidStringLength);
    }

    #[rstest]
    #[case("#GG0000")]
    #[case("#FG0000")]
    #[case("#FFG000")]
    #[case("#FF0G00")]
    #[case("#FF00G0")]
    #[case("#FF000G")]
    fn parse_hex_rejects_invalid_character(#[case] hex: &str) {
        let err = RGB::from_hex(hex).unwrap_err();
        assert_eq!(err, HtmlColorConversionError::InvalidCharacter);
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn bevy_color_conversion_preserves_rgb_channels() {
        let rgb = RGB::from(bevy::prelude::Color::srgb(0.25, 0.5, 0.75));
        assert_rgb_eq(rgb, 0.25, 0.5, 0.75);
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn rgb_conversion_to_bevy_color_preserves_rgb_channels() {
        let color = bevy::prelude::Color::from(RGB::from_f32(0.25, 0.5, 0.75));
        let srgba = color.to_srgba();

        assert_approx_eq(srgba.red, 0.25);
        assert_approx_eq(srgba.green, 0.5);
        assert_approx_eq(srgba.blue, 0.75);
    }

    #[test]
    fn test_blue_named() {
        let rgb = RGB::named(BLUE);

        assert_rgb_eq(rgb, 0.0, 0.0, 1.0);
    }

    #[test]
    fn to_greyscale_uses_luminance_weights() {
        let grey = RGB::from_f32(1.0, 0.0, 0.0).to_greyscale();
        assert_rgb_eq(grey, 0.2126, 0.2126, 0.2126);
    }

    #[test]
    fn desaturate_keeps_value_and_removes_saturation() {
        let desaturated = RGB::from_f32(1.0, 0.0, 0.0).desaturate();
        assert_rgb_eq(desaturated, 1.0, 1.0, 1.0);
    }

    #[rstest]
    #[case(0.0, 0.0, 0.0, 0.0)]
    #[case(0.5, 0.5, 0.5, 0.5)]
    #[case(1.0, 1.0, 1.0, 1.0)]
    fn lerp_interpolates_between_colors(
        #[case] percent: f32,
        #[case] r: f32,
        #[case] g: f32,
        #[case] b: f32,
    ) {
        let black = RGB::named(BLACK);
        let white = RGB::named(WHITE);

        assert_rgb_eq(black.lerp(white, percent), r, g, b);
    }

    #[test]
    // Test the lerp function
    fn test_lerp() {
        let black = RGB::named(BLACK);
        let white = RGB::named(WHITE);
        assert!(black.lerp(white, 0.0) == black);
        assert!(black.lerp(white, 1.0) == white);
    }
}
