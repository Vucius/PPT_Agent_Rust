use iced::Color;

#[allow(dead_code)]
pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub primary: Color,
    pub success: Color,
    pub danger: Color,
    pub text: Color,
    pub text_muted: Color,
    pub border: Color,
}

#[allow(dead_code)]
pub const DARK_PALETTE: Palette = Palette {
    background: Color::from_rgb(0.08, 0.08, 0.10),
    surface: Color::from_rgb(0.12, 0.12, 0.15),
    primary: Color::from_rgb(0.39, 0.58, 0.93),
    success: Color::from_rgb(0.18, 0.80, 0.44),
    danger: Color::from_rgb(0.90, 0.30, 0.26),
    text: Color::from_rgb(0.95, 0.95, 0.98),
    text_muted: Color::from_rgb(0.60, 0.60, 0.65),
    border: Color::from_rgb(0.20, 0.20, 0.25),
};

pub const BOLD_FONT: iced::Font = iced::Font {
    weight: iced::font::Weight::Bold,
    ..iced::Font::DEFAULT
};
