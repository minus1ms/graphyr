use std::sync::RwLock;

use data::Data;
use floem::{kurbo::Stroke, prelude::*};
use theme::MyTheme;
use view_data::ViewData;

mod data;
mod theme;
mod view_data;

fn main() {
    floem::launch(graphyr_view);
}

fn graphyr_view() -> impl IntoView {
    let data = Data::new();

    // we want everything to react to changes of it and then get new values from Data
    let view_data = RwSignal::new(ViewData::new());

    let my_theme = MyTheme::default();

    h_stack((
        // the cell that we view
        dyn_container(move || view_data.get(), {
            let my_theme = my_theme.clone();
            move |view_data| data.get_cell(&view_data).into_view(my_theme.clone())
        })
        .style(|s| s.size_full()),
        create_configuration(),
    ))
    .style(move |s| theme::theme(s, &my_theme))
    .style(|s| s.width_full())
}

fn create_configuration() -> Container {
    container(text("Configuration:")).style(|s| s.padding(10).border(Stroke::new(2.0)))
}
