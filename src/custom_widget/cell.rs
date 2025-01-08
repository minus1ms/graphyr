use iced::{
    advanced::{
        renderer,
        widget::{tree, Tree},
        Widget,
    },
    widget::{self, column, Container},
    Alignment::Center,
    Element,
    Length::Fill,
    Renderer, Theme,
};

use crate::{container, text_input};

pub trait Catalog: widget::container::Catalog {}

impl Catalog for Theme {}

pub struct Cell<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::Renderer,
    Theme: Catalog,
{
    content: Container<'a, Message, Theme, Renderer>,
}

impl<'a, Message> Cell<'a, Message>
where
    Message: 'static + Clone,
{
    pub fn new() -> Self {
        Self {
            content: widget::container(column![
                widget::container(
                    widget::text_input("", "a")
                        .align_x(Center)
                        .style(text_input::invisible),
                )
                .height(Fill)
                .align_y(Center) // optional table
            ])
            .style(container::bordered_box),
        }
    }
}

impl<'a, Message> Widget<Message, Theme, Renderer> for Cell<'a, Message> {
    fn tag(&self) -> tree::Tag {
        self.content.tag()
    }

    fn state(&self) -> tree::State {
        self.content.state()
    }

    fn children(&self) -> Vec<Tree> {
        self.content.children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.content.diff(tree);
    }

    fn size(&self) -> iced::Size<iced::Length> {
        self.content.size()
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        self.content.layout(tree, renderer, limits)
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.content
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }
}

impl<'a, Message> From<Cell<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(value: Cell<'a, Message>) -> Self {
        Element::new(value)
    }
}
