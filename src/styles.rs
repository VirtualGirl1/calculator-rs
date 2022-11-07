use iced::{Background, Color, text_input};
use iced_style::button;
use iced_style::text_input::Style;

// dark mode

pub struct ButtonStyleDark;
impl button::StyleSheet for ButtonStyleDark {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Default::default(),
            background: Option::from(Background::Color(Color::from_rgb8(0x7b, 0x7b, 0x7b))),
            border_radius: 1.0,
            border_width: 1.0,
            border_color: Default::default(),
            text_color: Color::BLACK,
        }
    }
}

pub struct InputStyleDark;
impl text_input::StyleSheet for InputStyleDark {
    fn active(&self) -> Style {
        Style {
            background: Background::from(Color::from_rgb8(0x7b, 0x7b, 0x7b)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default()
        }
    }

    fn focused(&self) -> Style {
        Style {
            background: Background::from(Color::from_rgb8(0x7b, 0x7b, 0x7b)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default()
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb8(0xaf, 0xaf, 0xaf)
    }

    fn value_color(&self) -> Color {
        Color::from_rgb8(0xff, 0xff, 0xff)
    }

    fn selection_color(&self) -> Color {
        Color::from_rgb8(0xff, 0xff, 0xff)
    }
}

