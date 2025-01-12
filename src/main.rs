use floem::{
    kurbo::Stroke,
    prelude::*,
    text::{Attrs, AttrsList, TextLayout},
};
use theme::MyTheme;

mod theme;

fn main() {
    floem::launch(graphyr_view);
}

fn graphyr_view() -> impl IntoView {
    let my_theme = MyTheme {
        background: Color::rgb8(26, 27, 38),
        background_hovered: Color::INDIAN_RED,
        secondary_background: Color::rgb8(41, 46, 66),
        secondary_background_hovered: Color::rgb8(59, 66, 97),
        foreground: Color::rgb8(192, 202, 245),
    };

    h_stack((
        v_stack((create_cell(my_theme.clone()), create_cell(my_theme.clone())))
            .style(|s| s.width_full()),
        create_configuration(),
    ))
    .style(move |s| theme::theme(s, &my_theme))
    .style(|s| s.width_full())
}

fn create_cell(my_theme: MyTheme) -> Container {
    let text = RwSignal::new(String::new());
    container(text_input(text).style(move |s| {
        let font_size = 12.;
        s.border_color(Color::TRANSPARENT)
            .background(my_theme.background)
            .max_width_full()
            .width({
                let mut text_layout = TextLayout::new();
                text_layout.set_text(
                    &text.get(),
                    AttrsList::new(Attrs::new().font_size(font_size)),
                );
                text_layout.size().width + 12.
            })
            .font_size(font_size)
    }))
    .style(move |s| {
        s.border(Stroke::new(1.0))
            .height_full()
            .items_center()
            .justify_center()
            .hover(|s| s.background(my_theme.background_hovered))
    })
}

fn create_configuration() -> Container {
    container(text("Configuration:")).style(|s| s.padding(10).border(Stroke::new(2.0)))
}
