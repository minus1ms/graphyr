use data::Data;
use floem::{prelude::*, reactive::create_effect};
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
    // used for loading new data
    let temp_data: RwSignal<Option<Data>> = RwSignal::new(None);
    let data_signal = RwSignal::new(Data::new());

    create_effect({
        move |_| {
            if let Some(new_value) = temp_data.get().take() {
                data_signal.set(Data::new());
            }
        }
    });

    // we want everything to react to changes of view_data and then get new values from data
    // temporary settings
    let view_data = RwSignal::new(ViewData::new());

    let my_theme = MyTheme::default();

    dyn_container(move || data_signal.get(), {
        let my_theme = my_theme.clone();
        move |data: Data| {
            data.build_view(
                view_data,
                data_signal.clone(),
                temp_data.clone(),
                my_theme.clone(),
            )
            .style(|s| s.width_full())
        }
    })
    .style(move |s| theme::theme(s, &my_theme))
    .style(|s| s.width_full())
}
