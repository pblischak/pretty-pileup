use ansi_term::Color;
use colorsys::Rgb;

pub struct BaseColors {
    pub a: Color,
    pub c: Color,
    pub g: Color,
    pub t: Color,
    pub n: Color,
}

impl BaseColors {
    pub fn new(a: Color, c: Color, g: Color, t: Color, n: Color) -> Self {
        Self { a, c, g, t, n }
    }
}

pub struct QualityGradient {
    pub low: Rgb,
    pub high: Rgb,
}

impl QualityGradient {
    pub fn new(low: Rgb, high: Rgb) -> Self {
        Self { low, high }
    }

    pub fn calc_color_on_gradient(&self, qual_val: f64) -> Color {
        let qual_ratio = qual_val / 40.0;
        let new_red: u8 = (self.low.red() + qual_ratio * (self.high.red() - self.low.red())) as u8;
        let new_green: u8 =
            (self.low.green() + qual_ratio * (self.high.green() - self.low.green())) as u8;
        let new_blue: u8 =
            (self.low.blue() + qual_ratio * (self.high.blue() - self.low.blue())) as u8;
        Color::RGB(new_red, new_green, new_blue)
    }
}

pub enum BasePair {
    BaseA,
    BaseC,
    BaseG,
    BaseT,
    BaseN,
}

pub struct ColorTheme {
    pub base_colors: BaseColors,
    pub quality_gradient: QualityGradient,
}

impl ColorTheme {
    pub fn new(theme_name: Option<&str>) -> Self {
        if let Some(tn) = theme_name {
            set_color_theme(tn.to_uppercase().as_str())
        } else {
            set_color_theme("DARK")
        }
    }

    pub fn get_base_color(&self, base: &BasePair) -> Color {
        match base {
            BasePair::BaseA => self.base_colors.a,
            BasePair::BaseC => self.base_colors.c,
            BasePair::BaseG => self.base_colors.g,
            BasePair::BaseT => self.base_colors.t,
            BasePair::BaseN => self.base_colors.n,
        }
    }
}

fn set_color_theme(upper_theme_name: &str) -> ColorTheme {
    if upper_theme_name == "AYULIGHT" || upper_theme_name == "LIGHT" {
        let base_colors = BaseColors::new(
            rgb_from_hex("#D2BFFF"),
            rgb_from_hex("#D2BFFF"),
            rgb_from_hex("#D2BFFF"),
            rgb_from_hex("#D2BFFF"),
            Color::RGB(50, 50, 50),
        );
        let quality_gradient = QualityGradient::new(
            Rgb::from_hex_str("#F26D78").unwrap_or(Rgb::from((50.0_f32, 50.0_f32, 50.0_f32))),
            Rgb::from_hex_str("#AAD94C").unwrap_or(Rgb::from((50.0_f32, 50.0_f32, 50.0_f32))),
        );
        ColorTheme {
            base_colors,
            quality_gradient,
        }
    } else {
        let base_colors = BaseColors::new(
            rgb_from_hex("#F07178"),
            rgb_from_hex("#7FD962"),
            rgb_from_hex("#73B8FF"),
            rgb_from_hex("#E6B673"),
            Color::RGB(50, 50, 50),
        );
        let quality_gradient = QualityGradient::new(
            Rgb::from_hex_str("#F26D78").unwrap_or(Rgb::from((50.0_f32, 50.0_f32, 50.0_f32))),
            Rgb::from_hex_str("#AAD94C").unwrap_or(Rgb::from((50.0_f32, 50.0_f32, 50.0_f32))),
        );
        ColorTheme {
            base_colors,
            quality_gradient,
        }
    }
}

fn rgb_from_hex(hex_color: &str) -> Color {
    let rgb = Rgb::from_hex_str(hex_color).unwrap_or(Rgb::from((50.0_f32, 50.0_f32, 50.0_f32)));
    Color::RGB(rgb.red() as u8, rgb.green() as u8, rgb.blue() as u8)
}
