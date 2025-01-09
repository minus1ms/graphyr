use floem::{kurbo::Stroke, prelude::*};

fn main() {
    floem::launch(graphyr_view);
}

fn graphyr_view() -> impl IntoView {
    h_stack((
        v_stack((create_cell(), create_cell())).style(|s| s.width_full()),
        create_configuration(),
    ))
    .style(|s| s.width_full())
}

fn create_cell() -> Container {
    container(text("a")).style(|s| {
        s.border(Stroke::new(1.0))
            .height_full()
            .items_center()
            .justify_center()
    })
}

fn create_configuration() -> Container {
    container(text("Configuration:")).style(|s| s.padding(10).border(Stroke::new(2.0)))
}
