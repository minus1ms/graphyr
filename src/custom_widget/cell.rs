use iced::{
    advanced::{
        renderer,
        widget::{tree, Tree},
        Widget,
    },
    font::Stretch,
    widget::{self, column},
    Alignment::Center,
    Element, Font,
    Length::Fill,
    Renderer, Theme,
};

use crate::{container, text_input};

pub enum Status {
    Default,
    Hovered,
}

pub struct Cell<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
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
                        .font(Font {
                            stretch: Stretch::SemiCondensed,
                            ..Font::MONOSPACE
                        })
                        .align_x(Center)
                        .style(text_input::invisible),
                )
                .height(Fill)
                .align_y(Center) // optional table
            ])
            .style(container::bordered_box)
            .into(),
        }
    }
}

impl<'a, Message> Widget<Message, Theme, Renderer> for Cell<'a, Message> {
    fn tag(&self) -> tree::Tag {
        self.content.as_widget().tag()
    }

    fn state(&self) -> tree::State {
        self.content.as_widget().state()
    }

    fn children(&self) -> Vec<Tree> {
        self.content.as_widget().children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.content.as_widget().diff(tree);
    }

    fn size(&self) -> iced::Size<iced::Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        self.content.as_widget().layout(tree, renderer, limits)
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
        let is_mouse_over = cursor.is_over(layout.bounds());
        let status = if is_mouse_over {
            Status::Hovered
        } else {
            Status::Default
        };

        self.content
            .as_widget()
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
