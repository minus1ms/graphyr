use iced::{
    advanced::{
        widget::{tree, Tree},
        Widget,
    },
    widget::{column, row},
    Element, Renderer, Theme,
};

use super::cell::Cell;

pub struct SpiralTable<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message> SpiralTable<'a, Message>
where
    Message: 'static + Clone,
{
    pub fn new() -> Self {
        Self {
            content: column![
                row![Cell::new(), Cell::new(), Cell::new()],
                row![Cell::new(), Cell::new(), Cell::new()],
                row![Cell::new(), Cell::new(), Cell::new()]
            ]
            .into(),
        }
    }
}

impl<'a, Message> Widget<Message, Theme, Renderer> for SpiralTable<'a, Message> {
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
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.content
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }
}

impl<'a, Message> From<SpiralTable<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(value: SpiralTable<'a, Message>) -> Self {
        Element::new(value)
    }
}
