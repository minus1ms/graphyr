use std::rc::Rc;

use data::Data;
use floem::{kurbo::Stroke, prelude::*};
use theme::MyTheme;
use view_data::{IntoView as _, ViewData};

mod data;
mod theme;
mod view_data;

fn main() {
    floem::launch(graphyr_view);
}

fn graphyr_view() -> impl IntoView {
    let data = Rc::new(Data::new());

    // we want everything to react to changes of it and then get new values from Data
    let view_data = RwSignal::new(ViewData::new());

    let my_theme = MyTheme::default();

    h_stack((
        // the cell that we view
        dyn_container(move || view_data.get(), {
            let my_theme = my_theme.clone();
            let data = data.clone();
            move |view_data| data.get_cell(&view_data).into_view(my_theme.clone())
        })
        .style(|s| s.size_full()),
        create_configuration(data),
    ))
    .style(move |s| theme::theme(s, &my_theme))
    .style(|s| s.width_full())
}

fn create_configuration(data: Rc<Data>) -> Stack {
    let data = data.clone();
    v_stack((
        text("Configuration:"),
        h_stack((
            Checkbox::new_rw(data.configuration.show_border.clone()),
            text("Show borders"),
        ))
        .style(|s| s.items_center().gap(5)),
    ))
    .style(|s| {
        s.padding(10)
            .items_center()
            .border(Stroke::new(1.0))
            .gap(10)
    })
}
