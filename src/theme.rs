use floem::{
    peniko::{Brush, Color},
    style::Style,
    views::TextInputClass,
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
            background: Color::rgb8(26, 27, 38),
            background_hovered: Color::rgb8(35, 37, 54),
            secondary_background: Color::rgb8(41, 46, 66),
            secondary_background_hovered: Color::rgb8(59, 66, 97),
            foreground: Color::rgb8(192, 202, 245),
        }
    }
}

pub fn theme(s: Style, my_theme: &MyTheme) -> Style {
    s.background(my_theme.background)
        .color(my_theme.foreground)
        .class(TextInputClass, move |s| {
            s.background(my_theme.secondary_background)
                .cursor_color(Brush::from({
                    let mut res = my_theme.background_hovered;
                    res.a = 150;
                    res
                }))
                .hover(move |s| s.background(my_theme.secondary_background_hovered))
                .focus(|s| s.hover(|s| s.background(my_theme.secondary_background_hovered)))
        })
}
