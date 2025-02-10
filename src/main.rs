use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};

use data::Data;
use floem::{kurbo::Stroke, prelude::*};
use main_view::Main;
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
    let data = Rc::new(RefCell::new(Data::new()));

    // we want everything to react to changes of view_data and then get new values from data
    // temporary settings
    let view_data = RwSignal::new(ViewData::new());

    let my_theme = MyTheme::default();

    h_stack((
        Main::new(view_data, data.clone(), my_theme.clone()).style(|s| s.size_full()),
        {
            let data_ref = data.borrow();
            create_configuration(&data_ref)
        },
    ))
    .style(move |s| theme::theme(s, &my_theme))
    .style(|s| s.width_full())
}

fn create_configuration(data: &Data) -> Stack {
    let layers = data.configuration.layers;
    let unique_atomic = AtomicU32::new(0);
    v_stack((
        "Configuration:".style(|s| s.font_bold().font_size(15)),
        empty(),
        h_stack((
            Checkbox::new_rw(data.configuration.show_border.clone()),
            "Show borders",
        ))
        .style(|s| s.items_center().gap(5)),
        empty(),
        "Layers:",
        dyn_stack(
            move || layers.get(),
            move |_| unique_atomic.fetch_add(1, Ordering::Relaxed),
            |layer| {
                h_stack((Checkbox::new_rw(layer.enabled.clone()), layer.name)).style(|s| s.gap(5))
            },
        ),
        empty(),
    ))
    .style(|s| {
        s.padding(10)
            .items_center()
            .border(Stroke::new(1.0))
            .gap(10)
    })
}
