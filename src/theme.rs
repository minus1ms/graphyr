use floem::{
    peniko::{Brush, Color},
    style::Style,
    views::{ButtonClass, CheckboxClass, TextInputClass},
};

#[derive(Clone)]
pub struct MyTheme {
    pub background: Color,
    pub background_hovered: Color,
    pub secondary_background: Color,
    pub secondary_background_hovered: Color,
    pub foreground: Color,
}

impl Default for MyTheme {
    fn default() -> Self {
        MyTheme {
            background: Color::from_rgb8(26, 27, 38),
            background_hovered: Color::from_rgb8(35, 37, 54),
            secondary_background: Color::from_rgb8(41, 46, 66),
            secondary_background_hovered: Color::from_rgb8(59, 66, 97),
            foreground: Color::from_rgb8(192, 202, 245),
        }
    }
}

pub fn theme(s: Style, my_theme: &MyTheme) -> Style {
    s.background(my_theme.background)
        .color(my_theme.foreground)
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
