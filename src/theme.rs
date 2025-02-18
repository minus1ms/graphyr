use std::sync::Arc;

use floem::{
    peniko::{Brush, Color},
    prelude::palette::css,
    style::Style,
    text::{fontdb::Source, FONT_SYSTEM},
    views::{ButtonClass, CheckboxClass, TextInputClass},
};

const INTER_REGULAR: &[u8] = include_bytes!("../Inter-VariableFont_opsz,wght.ttf");

#[derive(Clone)]
pub struct MyTheme {
    pub background: Color,
    pub background_hovered: Color,
    pub secondary_background: Color,
    pub secondary_background_hovered: Color,
    pub foreground: Color,
    pub border: Color,
}

impl Default for MyTheme {
    fn default() -> Self {
        MyTheme {
            background: Color::from_rgb8(15, 16, 23),
            background_hovered: Color::from_rgb8(26, 28, 40),
            secondary_background: Color::from_rgb8(32, 35, 51),
            secondary_background_hovered: Color::from_rgb8(47, 52, 76),
            foreground: css::WHITE,
            border: css::WHITE,
        }
    }
}

pub fn theme(s: Style, my_theme: &MyTheme) -> Style {
    let mut system = FONT_SYSTEM.lock();
    system
        .db_mut()
        .load_font_source(Source::Binary(Arc::new(INTER_REGULAR)));

    system.db_mut().set_sans_serif_family("Inter");

    s.background(my_theme.background)
        .color(my_theme.foreground)
        .border_color(my_theme.border)
        .class(TextInputClass, |s| {
            s.background(my_theme.secondary_background)
                .cursor_color(Brush::from(my_theme.background_hovered.with_alpha(0.6)))
                .hover(|s| s.background(my_theme.secondary_background_hovered))
                .focus(|s| s.hover(|s| s.background(my_theme.secondary_background_hovered)))
        })
        .class(CheckboxClass, |s| {
            s.background(my_theme.background)
                .hover(|s| s.background(my_theme.secondary_background))
                .focus(|s| s.hover(|s| s.background(my_theme.secondary_background)))
                .size(15, 15)
        })
        .class(ButtonClass, |s| {
            s.background(my_theme.background)
                .color(my_theme.foreground)
                .hover(|s| s.background(my_theme.secondary_background))
                .focus(|s| s.hover(|s| s.background(my_theme.secondary_background)))
        })
}
