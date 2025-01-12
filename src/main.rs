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
    let my_theme = MyTheme::default();

    h_stack((
        create_table(my_theme.clone(), true).style(|s| s.width_full()),
        create_configuration(),
    ))
    .style(move |s| theme::theme(s, &my_theme))
    .style(|s| s.width_full())
}

fn create_table(my_theme: MyTheme, inner: bool) -> Stack {
    let grid = 5;

    v_stack_from_iter((0..grid).map(|_| {
        h_stack_from_iter(
            (0..grid).map(|_| create_cell(my_theme.clone(), inner).style(|s| s.width_full())),
        )
        .style(|s| s.height_full())
    }))
}

fn create_cell(my_theme: MyTheme, table: bool) -> Stack {
    v_stack((create_cell_title(my_theme.clone()), {
        let res: Box<dyn View> = if table {
            Box::new(create_table(my_theme.clone(), false))
        } else {
            Box::new(empty())
        };
        res
    }))
    .style(move |s| {
        s.border(Stroke::new(1.0))
            .items_center()
            .justify_center()
            .hover(|s| s.background(my_theme.background_hovered))
    })
}

fn create_cell_title(my_theme: MyTheme) -> TextInput {
    let text = RwSignal::new(String::new());
    text_input(text).style(move |s| {
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
    })
}

fn create_configuration() -> Container {
    container(text("Configuration:")).style(|s| s.padding(10).border(Stroke::new(2.0)))
}
