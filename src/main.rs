use data::Data;
use floem::prelude::*;
use theme::MyTheme;
use view_data::ViewData;

mod data;
mod main_view;
mod theme;
mod utils;
mod view_data;

fn main() {
    floem::launch(graphyr_view);
}

fn graphyr_view() -> impl IntoView {
    let data_signal = RwSignal::new(Data::new());

    // we want everything to react to changes of view_data and then get new values from data
    // temporary settings
    let view_data = RwSignal::new(ViewData::new());

    let my_theme = MyTheme::default();

    dyn_container(move || data_signal.get(), {
        let my_theme = my_theme.clone();
        move |data: Data| {
            data.build_view(view_data, data_signal.clone(), my_theme.clone())
                .style(|s| s.width_full())
        }
    })
    .style(move |s| theme::theme(s, &my_theme))
    .style(|s| s.width_full())
}
