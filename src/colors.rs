//! Module for working with colors.

use ansi_term::Color;
use colorsys::Rgb;

use crate::config::Config;

const PHRED_MAX: f64 = 40.0;

/// Enumerate DNA base pairs for matching.
pub enum BasePair {
    BaseA,
    BaseC,
    BaseG,
    BaseT,
    BaseN,
}

#[derive(Debug)]
/// Store an `ansi_term::Color` value for each possible base pair (A, C, G, T, N).
pub struct BaseColors {
    pub a: Color,
    pub c: Color,
    pub g: Color,
    pub t: Color,
    pub n: Color,
}

impl BaseColors {
    /// Construct a new `BaseColor`.
    pub fn new(a: Color, c: Color, g: Color, t: Color, n: Color) -> Self {
        Self { a, c, g, t, n }
    }
}

impl Default for BaseColors {
    fn default() -> Self {
        Self {
            a: rgb_from_hex("#F07178"),
            c: rgb_from_hex("#7FD962"),
            g: rgb_from_hex("#73B8FF"),
            t: rgb_from_hex("#E6B673"),
            n: Color::RGB(50, 50, 50),
        }
    }
}

#[derive(Debug)]
/// Store values representing the start and end of a color gradient.
pub struct QualityGradient {
    pub low: Rgb,
    pub high: Rgb,
}

impl QualityGradient {
    /// Construct a new `ColorGradient`.
    pub fn new(low: Rgb, high: Rgb) -> Self {
        Self { low, high }
    }

    /// Calculate gradient color associated with given quality value.
    ///
    /// The passed value is normalized by the maximum PHRED value (`PHRED_MAX = 40.0`)
    /// and is clamped between 0.0 and 1.0 for values outside the range 0.0 to 40.0.
    pub fn calc_color_on_gradient(&self, qual_val: f64) -> Color {
        let qual_ratio = (qual_val / PHRED_MAX).clamp(0.0, 1.0);
        let new_red: u8 = (self.low.red() + qual_ratio * (self.high.red() - self.low.red())) as u8;
        let new_green: u8 =
            (self.low.green() + qual_ratio * (self.high.green() - self.low.green())) as u8;
        let new_blue: u8 =
            (self.low.blue() + qual_ratio * (self.high.blue() - self.low.blue())) as u8;
        Color::RGB(new_red, new_green, new_blue)
    }
}

impl Default for QualityGradient {
    fn default() -> Self {
        Self {
            low: Rgb::from_hex_str("#F26D78")
                .unwrap_or_else(|_| Rgb::from((50.0_f32, 50.0_f32, 50.0_f32))),
            high: Rgb::from_hex_str("#AAD94C")
                .unwrap_or_else(|_| Rgb::from((50.0_f32, 50.0_f32, 50.0_f32))),
        }
    }
}

#[derive(Debug, Default)]
pub struct ColorTheme {
    pub base_colors: BaseColors,
    pub quality_gradient: QualityGradient,
}

impl ColorTheme {
    pub fn get_base_color(&self, base: &BasePair) -> Color {
        match base {
            BasePair::BaseA => self.base_colors.a,
            BasePair::BaseC => self.base_colors.c,
            BasePair::BaseG => self.base_colors.g,
            BasePair::BaseT => self.base_colors.t,
            BasePair::BaseN => self.base_colors.n,
        }
    }

    pub(crate) fn from_config(config: Config) -> ColorTheme {
        let base_colors = BaseColors::new(
            rgb_from_hex(config.base_colors.base_a.as_str()),
            rgb_from_hex(config.base_colors.base_c.as_str()),
            rgb_from_hex(config.base_colors.base_g.as_str()),
            rgb_from_hex(config.base_colors.base_t.as_str()),
            rgb_from_hex(config.base_colors.base_n.as_str()),
        );
        let quality_gradient = QualityGradient::new(
            Rgb::from_hex_str(config.quality_gradient.low.as_str())
                .unwrap_or_else(|_| Rgb::from((50.0_f32, 50.0_f32, 50.0_f32))),
            Rgb::from_hex_str(config.quality_gradient.high.as_str())
                .unwrap_or_else(|_| Rgb::from((50.0_f32, 50.0_f32, 50.0_f32))),
        );
        ColorTheme {
            base_colors,
            quality_gradient,
        }
    }
}

fn rgb_from_hex(hex_color: &str) -> Color {
    let rgb =
        Rgb::from_hex_str(hex_color).unwrap_or_else(|_| Rgb::from((50.0_f32, 50.0_f32, 50.0_f32)));
    Color::RGB(rgb.red() as u8, rgb.green() as u8, rgb.blue() as u8)
}
