use anyhow::Result;
use custom_widget::{cell::Cell, spiral_table::SpiralTable};
use iced::{
    widget::{self, row, text},
    Element,
    Length::Fill,
    Theme,
};

mod container;
mod custom_widget;
mod text_input;

#[derive(Default)]
struct Graphyr {}

fn main() -> Result<()> {
    Ok(iced::application("Graphyr", update, view)
        .theme(theme)
        .run()?)
}

fn update(counter: &mut Graphyr, message: ()) {
    todo!()
}

fn view(counter: &Graphyr) -> Element<()> {
    // starting grid is 5x5
    let grid = 5;
    row![
        Cell::new(),
        // widget::container(column((0..grid).map(|_| {
        //     row((0..grid).map(|_| {
        //         widget::container(widget::mouse_area(
        //             widget::text_input("", "e")
        //                 .align_x(Center)
        //                 .style(text_input::invisible),
        //         ))
        //         .align_y(Center)
        //         .padding(2)
        //         .height(Fill)
        //         .width(Fill)
        //         .style(container::bordered_box)
        //         .into()
        //     }))
        //     .into()
        // })))
        // .height(Fill),
        // configurational stuff
        widget::container(text("Configuration:"))
            .padding(20)
            .height(Fill)
            .style(container::thick_bordered_box),
    ]
    .into()
}

fn theme(_: &Graphyr) -> Theme {
    Theme::KanagawaDragon
}
