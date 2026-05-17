use crate::prelude::{HtmlColorConversionError, HSV, RGB};
use std::convert::From;
use std::ops;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Copy, Clone, Default, Debug)]
/// Represents an R/G/B triplet, in the range 0..1 (32-bit float)
pub struct RGBA {
    /// The red component (0..1)
    pub r: f32,
    /// The green component (0..1)
    pub g: f32,
    /// The blue component (0..1)
    pub b: f32,
    /// The alpha component (0..1), 0 is transparent, 1 is solid
    pub a: f32,
}

// Implement operator overloading

/// Support adding a float to a color. The result is clamped via the constructor.
impl ops::Add<f32> for RGBA {
    type Output = Self;
    fn add(mut self, rhs: f32) -> Self {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
        self.a += rhs;
        self
    }
}

/// Support adding an RGB to a color. The result is clamped via the constructor.
impl ops::Add<RGBA> for RGBA {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
        self
    }
}

/// Support subtracting a float from a color. The result is clamped via the constructor.
impl ops::Sub<f32> for RGBA {
    type Output = Self;
    fn sub(mut self, rhs: f32) -> Self {
        self.r -= rhs;
        self.g -= rhs;
        self.b -= rhs;
        self.a -= rhs;
        self
    }
}

/// Support subtracting an RGB from a color. The result is clamped via the constructor.
impl ops::Sub<RGBA> for RGBA {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
        self
    }
}

/// Support multiplying a color by a float. The result is clamped via the constructor.
impl ops::Mul<f32> for RGBA {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
        self
    }
}

/// Support multiplying a color by another color. The result is clamped via the constructor.
impl ops::Mul<RGBA> for RGBA {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
        self
    }
}

impl RGBA {
    /// Constructs a new, zeroed (black) RGB triplet.
    #[must_use]
    pub fn new() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }

    /// Constructs a new RGB color, from 3 32-bit floats in the range 0..1
    ///
    /// # Arguments
    ///
    /// * `r` - the red component (0..1)
    /// * `g` - the green component (0..1)
    /// * `b` - the blue component (0..1)
    /// * `a` - the alpha component (0..1). 0 is transparent, 1 is solid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// let red = RGBA::from_f32(1.0, 0.0, 0.0, 1.0);
    /// let green = RGBA::from_f32(0.0, 1.0, 0.0, 1.0);
    /// let invisible = RGBA::from_f32(0.0, 0.0, 0.0, 0.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        let r_clamped = r.clamp(0.0, 1.0);
        let g_clamped = g.clamp(0.0, 1.0);
        let b_clamped = b.clamp(0.0, 1.0);
        let a_clamped = a.clamp(0.0, 1.0);
        Self {
            r: r_clamped,
            g: g_clamped,
            b: b_clamped,
            a: a_clamped,
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
    /// let red = RGBA::from_u8(255, 0, 0, 255);
    /// let green = RGBA::from_u8(0, 255, 0, 255);
    /// let invisible = RGBA::from_u8(0, 0, 0, 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
            a: f32::from(a) / 255.0,
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
    /// let red = RGBA::named(RED);
    /// let green = RGBA::named((0, 255, 0));
    /// ```
    #[inline]
    #[must_use]
    pub fn named(col: (u8, u8, u8)) -> Self {
        Self::from_u8(col.0, col.1, col.2, 255)
    }

    /// Constructs from an HTML color code (e.g. "#eeffeeff")
    ///
    /// # Arguments
    ///
    /// * `code` - an HTML color notation (e.g. "#ffeeffff")
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// let red = RGBA::from_hex("#FF0000FF");
    /// let green = RGBA::from_hex("#00FF00FF");
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

        let alpha1 = match full_code.next() {
            Some(alpha) => match alpha.to_digit(16) {
                Some(alpha) => alpha * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let alpha2 = match full_code.next() {
            Some(alpha) => match alpha.to_digit(16) {
                Some(alpha) => alpha,
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
            a: (alpha1 + alpha2) as f32 / 255.0,
        })
    }

    /// Converts to an RGB, dropping the alpha component
    #[inline]
    #[must_use]
    pub fn to_rgb(&self) -> RGB {
        RGB::from_f32(self.r, self.g, self.b)
    }

    /// Converts an RGBA to an array of `f32`, useful in Bevy.
    #[cfg(feature = "bevy")]
    #[must_use]
    pub fn as_rgba_f32(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Applies a quick grayscale conversion to the color
    #[inline]
    #[must_use]
    pub fn to_greyscale(&self) -> Self {
        let linear = (self.r * 0.2126) + (self.g * 0.7152) + (self.b * 0.0722);
        Self::from_f32(linear, linear, linear, self.a)
    }

    /// Applies a lengthier desaturate (via HSV) to the color
    #[inline]
    #[must_use]
    pub fn desaturate(&self) -> Self {
        let mut hsv = self.to_rgb().to_hsv();
        hsv.s = 0.0;
        hsv.to_rgb().to_rgba(self.a)
    }

    /// Lerps by a specified percentage (from 0 to 1) between this color and another
    #[inline]
    #[must_use]
    pub fn lerp(&self, color: Self, percent: f32) -> Self {
        let range = (
            color.r - self.r,
            color.g - self.g,
            color.b - self.b,
            color.a - self.a,
        );
        Self {
            r: self.r + range.0 * percent,
            g: self.g + range.1 * percent,
            b: self.b + range.2 * percent,
            a: self.a + range.3 * percent,
        }
    }

    /// Lerps only the alpha channel, by a specified percentage (from 0 to 1) between this color and another
    #[inline]
    #[must_use]
    pub fn lerp_alpha(&self, color: Self, percent: f32) -> Self {
        let range = color.a - self.a;
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a + range * percent,
        }
    }
}

/// Support conversion from RGB
impl From<RGB> for RGBA {
    fn from(item: RGB) -> Self {
        Self::from_f32(item.r, item.g, item.b, 1.0)
    }
}

/// Support conversion from HSV
impl From<HSV> for RGBA {
    fn from(item: HSV) -> Self {
        item.to_rgba(1.0)
    }
}

/// Support conversion from a color tuple
impl From<(u8, u8, u8, u8)> for RGBA {
    fn from(vals: (u8, u8, u8, u8)) -> Self {
        Self::from_u8(vals.0, vals.1, vals.2, vals.3)
    }
}

/// Support conversion from a color tuple
impl From<(u8, u8, u8)> for RGBA {
    fn from(vals: (u8, u8, u8)) -> Self {
        Self::from_u8(vals.0, vals.1, vals.2, 255)
    }
}

impl From<[f32; 4]> for RGBA {
    fn from(item: [f32; 4]) -> Self {
        Self::from_f32(item[0], item[1], item[2], item[3])
    }
}

// Support conversion from Bevy
#[cfg(feature = "bevy")]
impl From<bevy::prelude::Color> for RGBA {
    fn from(item: bevy::prelude::Color) -> Self {
        let srgba = item.to_srgba();
        Self::from_f32(srgba.red, srgba.green, srgba.blue, srgba.alpha)
    }
}

#[cfg(feature = "bevy")]
impl From<RGBA> for bevy::prelude::Color {
    fn from(item: RGBA) -> Self {
        Self::srgba(item.r, item.g, item.b, item.a)
    }
}

#[cfg(feature = "crossterm")]
mod crossterm_features {
    use super::RGBA;
    use crossterm::style::Color;
    use std::convert::TryFrom;

    impl TryFrom<RGBA> for Color {
        type Error = &'static str;

        fn try_from(rgb: RGBA) -> Result<Self, Self::Error> {
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
        use crate::prelude::RGBA;
        use crossterm::style::Color;
        use std::convert::TryInto;

        #[test]
        fn basic_conversion() {
            let rgb = RGBA {
                r: 0.0,
                g: 0.5,
                b: 1.0,
                a: 1.0,
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
            let rgb = RGBA {
                r: 0.0,
                g: 0.5,
                b: -1.0,
                a: 1.0,
            };
            let rgb: Result<Color, _> = rgb.try_into();
            assert!(rgb.is_err());
        }

        #[test]
        fn too_large_rgb() {
            let rgb = RGBA {
                r: 0.0,
                g: 0.5,
                b: 1.1,
                a: 1.0,
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
    fn make_rgba_minimal() {
        let black = RGBA::new();
        assert!(black.r < f32::EPSILON);
        assert!(black.g < f32::EPSILON);
        assert!(black.b < f32::EPSILON);
        assert!(black.a < f32::EPSILON);
    }

    #[test]
    fn new_rgba_is_transparent_black() {
        assert_rgba_eq(RGBA::new(), 0.0, 0.0, 0.0, 0.0);
    }

    #[test]
    fn from_f32_clamps_components() {
        let rgba = RGBA::from_f32(-1.0, 0.5, 2.0, 1.5);
        assert_rgba_eq(rgba, 0.0, 0.5, 1.0, 1.0);
    }

    #[test]
    fn from_u8_converts_components() {
        let rgba = RGBA::from_u8(255, 128, 0, 64);
        assert_rgba_eq(rgba, 1.0, 128.0 / 255.0, 0.0, 64.0 / 255.0);
    }

    #[rstest]
    #[case(RGBA::from(RGB::from_f32(0.25, 0.5, 0.75)), 0.25, 0.5, 0.75, 1.0)]
    #[case(
        RGBA::from((64, 128, 255, 32)),
        64.0 / 255.0,
        128.0 / 255.0,
        1.0,
        32.0 / 255.0
    )]
    #[case(RGBA::from((64, 128, 255)), 64.0 / 255.0, 128.0 / 255.0, 1.0, 1.0)]
    #[case(RGBA::from([-1.0, 0.5, 2.0, 0.25]), 0.0, 0.5, 1.0, 0.25)]
    fn conversions_fill_or_preserve_alpha(
        #[case] rgba: RGBA,
        #[case] r: f32,
        #[case] g: f32,
        #[case] b: f32,
        #[case] a: f32,
    ) {
        assert_rgba_eq(rgba, r, g, b, a);
    }

    #[test]
    fn hsv_conversion_uses_opaque_alpha() {
        let rgba = RGBA::from(HSV::from_f32(240.0 / 360.0, 1.0, 1.0));
        assert_rgba_eq(rgba, 0.0, 0.0, 1.0, 1.0);
    }

    #[rstest]
    #[case(RGBA::from_f32(0.25, 0.5, 0.75, 0.5) + 0.125, 0.375, 0.625, 0.875, 0.625)]
    #[case(
        RGBA::from_f32(0.25, 0.5, 0.75, 0.5) + RGBA::from_f32(0.125, 0.25, 0.125, 0.25),
        0.375,
        0.75,
        0.875,
        0.75
    )]
    #[case(RGBA::from_f32(0.25, 0.5, 0.75, 0.5) - 0.125, 0.125, 0.375, 0.625, 0.375)]
    #[case(
        RGBA::from_f32(0.25, 0.5, 0.75, 0.5) - RGBA::from_f32(0.125, 0.25, 0.125, 0.25),
        0.125,
        0.25,
        0.625,
        0.25
    )]
    #[case(RGBA::from_f32(0.25, 0.5, 0.75, 0.5) * 0.5, 0.125, 0.25, 0.375, 0.25)]
    #[case(
        RGBA::from_f32(0.25, 0.5, 0.75, 0.5) * RGBA::from_f32(0.125, 0.25, 0.125, 0.25),
        0.03125,
        0.125,
        0.09375,
        0.125
    )]
    fn arithmetic_operators_apply_component_wise(
        #[case] rgba: RGBA,
        #[case] r: f32,
        #[case] g: f32,
        #[case] b: f32,
        #[case] a: f32,
    ) {
        assert_rgba_eq(rgba, r, g, b, a);
    }

    #[rstest]
    #[case("#FF0000FF", 1.0, 0.0, 0.0, 1.0)]
    #[case("#00FF00FF", 0.0, 1.0, 0.0, 1.0)]
    #[case("#0000FFFF", 0.0, 0.0, 1.0, 1.0)]
    #[case("#80800080", 128.0 / 255.0, 128.0 / 255.0, 0.0, 128.0 / 255.0)]
    fn parse_hex_colors(
        #[case] hex: &str,
        #[case] r: f32,
        #[case] g: f32,
        #[case] b: f32,
        #[case] a: f32,
    ) {
        let rgba = RGBA::from_hex(hex).expect("valid RGBA hex color");
        assert_rgba_eq(rgba, r, g, b, a);
    }

    #[test]
    fn parse_hex_rejects_missing_hash() {
        let err = RGBA::from_hex("FF0000FF").unwrap_err();
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
    #[case("#FFFFFF")]
    #[case("#FFFFFFF")]
    #[case("#FFFFFFFF00")]
    fn parse_hex_rejects_invalid_length(#[case] hex: &str) {
        let err = RGBA::from_hex(hex).unwrap_err();
        assert_eq!(err, HtmlColorConversionError::InvalidStringLength);
    }

    #[rstest]
    #[case("#GG0000FF")]
    #[case("#FG0000FF")]
    #[case("#FFG000FF")]
    #[case("#FF0G00FF")]
    #[case("#FF00G0FF")]
    #[case("#FF000GFF")]
    #[case("#FF0000GF")]
    #[case("#FF0000FG")]
    fn parse_hex_rejects_invalid_character(#[case] hex: &str) {
        let err = RGBA::from_hex(hex).unwrap_err();
        assert_eq!(err, HtmlColorConversionError::InvalidCharacter);
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn as_rgba_f32_returns_components_for_bevy() {
        let rgba = RGBA::from_f32(0.25, 0.5, 0.75, 0.125);
        let [r, g, b, a] = rgba.as_rgba_f32();

        assert_approx_eq(r, 0.25);
        assert_approx_eq(g, 0.5);
        assert_approx_eq(b, 0.75);
        assert_approx_eq(a, 0.125);
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn bevy_color_conversion_preserves_rgba_channels() {
        let rgba = RGBA::from(bevy::prelude::Color::srgba(0.25, 0.5, 0.75, 0.125));
        assert_rgba_eq(rgba, 0.25, 0.5, 0.75, 0.125);
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn rgba_conversion_to_bevy_color_preserves_rgba_channels() {
        let color = bevy::prelude::Color::from(RGBA::from_f32(0.25, 0.5, 0.75, 0.125));
        let srgba = color.to_srgba();

        assert_approx_eq(srgba.red, 0.25);
        assert_approx_eq(srgba.green, 0.5);
        assert_approx_eq(srgba.blue, 0.75);
        assert_approx_eq(srgba.alpha, 0.125);
    }

    #[test]
    fn to_rgb_drops_alpha() {
        let rgba = RGBA::from_f32(1.0, 0.5, 0.0, 0.25);
        let rgb = rgba.to_rgb();

        assert_rgb_eq(rgb, 1.0, 0.5, 0.0);
    }

    #[test]
    fn to_greyscale_preserves_alpha() {
        let rgba = RGBA::from_f32(1.0, 0.0, 0.0, 0.25);
        let grey = rgba.to_greyscale();

        assert_rgba_eq(grey, 0.2126, 0.2126, 0.2126, 0.25);
    }

    #[test]
    fn desaturate_preserves_alpha() {
        let rgba = RGBA::from_f32(1.0, 0.0, 0.0, 0.25);
        let desaturated = rgba.desaturate();

        assert_rgba_eq(desaturated, 1.0, 1.0, 1.0, 0.25);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn test_blue_named() {
        let rgba = RGBA::named(BLUE);
        assert_rgba_eq(rgba, 0.0, 0.0, 1.0, 1.0);
    }

    #[test]
    // Test the lerp function
    fn test_lerp() {
        let black = RGBA::named(BLACK);
        let white = RGBA::named(WHITE);
        assert!(black.lerp(white, 0.0) == black);
        assert!(black.lerp(white, 1.0) == white);

        assert_rgba_eq(black.lerp(white, 0.5), 0.5, 0.5, 0.5, 1.0);
    }

    #[test]
    // Test the lerp function
    fn test_lerp_alpha() {
        let black = RGB::named(BLACK).to_rgba(0.0);
        let white = RGB::named(WHITE).to_rgba(1.0);

        let l0 = black.lerp_alpha(white, 0.0);
        let l1 = black.lerp_alpha(white, 1.0);

        assert_rgba_eq(l0, 0.0, 0.0, 0.0, 0.0);
        assert_rgba_eq(l1, 0.0, 0.0, 0.0, 1.0);
    }
}
