use anyhow::Result;
use iced::{
    widget::{self, column, row, text},
    Alignment::Center,
    Element,
    Length::Fill,
    Theme,
};

mod container;
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
        widget::container(column((0..grid).map(|_| {
            row((0..grid).map(|_| {
                widget::text_input("aa", "e")
                    .align_x(Center)
                    .style(text_input::invisible_bordered)
                    .into()
                // widget::container(widget::text_input("aa", "e"))
                //     .align_x(Center)
                //     .align_y(Center)
                //     .padding(2)
                //     .height(Fill)
                //     .width(Fill)
                //     .style(container::bordered_box)
                //     .into()
            }))
            .into()
        })))
        .height(Fill),
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
