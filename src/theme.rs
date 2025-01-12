use std::time::Duration;

use floem::{
    peniko::{Brush, Color},
    style::{Background, CursorStyle, Foreground, Style, Transition},
    taffy::AlignItems,
    unit::UnitExt as _,
    views::{
        dropdown, scroll,
        slider::{self, SliderClass},
        ButtonClass, CheckboxClass, LabelClass, LabelCustomStyle, LabeledCheckboxClass,
        LabeledRadioButtonClass, ListClass, ListItemClass, PlaceholderTextClass, RadioButtonClass,
        RadioButtonDotClass, TextInputClass, ToggleButtonCircleRad, ToggleButtonClass,
        ToggleButtonInset, TooltipClass,
    },
};

#[derive(Clone)]
pub struct MyTheme {
    pub background: Color,
    pub background_hovered: Color,
    pub secondary_background: Color,
    pub secondary_background_hovered: Color,
    pub foreground: Color,
}

pub fn theme(s: Style, my_theme: &MyTheme) -> Style {
    s.background(my_theme.background)
        .color(my_theme.foreground)
        .class(TextInputClass, move |s| {
            s.background(my_theme.secondary_background)
                .cursor_color(Brush::from(my_theme.foreground))
                .hover(move |s| s.background(my_theme.secondary_background_hovered))
        })
}
