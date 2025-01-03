use iced::{
    widget::text_input::{Status, Style},
    Background, Border, Color, Theme,
};

pub fn invisible(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border::default(),
        icon: palette.background.weak.text,
        placeholder: palette.background.strong.color,
        value: palette.background.base.text,
        selection: palette.primary.weak.color,
    }
}
