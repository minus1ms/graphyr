use iced::{
    widget::text_input::{Status, Style},
    Background, Border, Theme,
};

pub fn invisible_bordered(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Background::Color(palette.background.base.color),
        border: Border {
            width: 1.0,
            radius: 0.0.into(),
            color: palette.background.strong.color,
        },
        icon: palette.background.weak.text,
        placeholder: palette.background.strong.color,
        value: palette.background.base.text,
        selection: palette.primary.weak.color,
    }
}
