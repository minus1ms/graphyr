use floem::{
    kurbo::Stroke,
    prelude::*,
    text::{Attrs, AttrsList, TextLayout},
};
use theme::MyTheme;

mod theme;

struct Cell {
    title: RwSignal<String>,
    table: Option<Table>,
}

impl Cell {
    fn new(table: Option<Table>) -> Self {
        Self {
            title: RwSignal::new(String::new()),
            table,
        }
    }

    fn into_view(&self, my_theme: MyTheme) -> impl View {
        let has_table = self.table.is_some();
        v_stack((self.create_cell_title(my_theme.clone()), {
            let res: Box<dyn View> = if let Some(table) = &self.table {
                Box::new(table.into_view(my_theme.clone()))
            } else {
                Box::new(empty())
            };
            res
        }))
        .style(move |s| {
            if has_table { s.gap(15) } else { s }
                .border(Stroke::new(1.0))
                .items_center()
                .justify_center()
                .hover(|s| s.background(my_theme.background_hovered))
                .size_full()
                .padding(15)
        })
    }

    fn create_cell_title(&self, my_theme: MyTheme) -> TextInput {
        let text = self.title.clone();
        text_input(text).style(move |s| {
            let font_size = 12.;
            s.border_color(Color::TRANSPARENT)
                .background(my_theme.background)
                .max_width_full()
                .width({
                    let mut text_layout = TextLayout::new();
                    text_layout.set_text(
                        &text.get(),
                        AttrsList::new(Attrs::new().font_size(font_size)),
                    );
                    text_layout.size().width + 12.
                })
                .font_size(font_size)
        })
    }
}

struct Table {
    cells: Vec<Vec<Cell>>,
}

impl Table {
    fn into_view(&self, my_theme: MyTheme) -> impl View {
        v_stack_from_iter((0..self.cells.len()).map(|i| {
            let row = &self.cells[i];
            h_stack_from_iter((0..row.len()).map(|i| row[i].into_view(my_theme.clone())))
                .style(|s| s.size_full())
        }))
        .style(|s| s.size_full())
    }
}

fn main() {
    floem::launch(graphyr_view);
}

fn graphyr_view() -> impl IntoView {
    let my_theme = MyTheme::default();

    h_stack((
        Cell::new(Some(Table {
            cells: vec![vec![
                Cell::new(Some(Table {
                    cells: vec![vec![Cell::new(None)]],
                })),
                Cell::new(Some(Table {
                    cells: vec![vec![Cell::new(None)]],
                })),
            ]],
        }))
        .into_view(my_theme.clone()),
        create_configuration(),
    ))
    .style(move |s| theme::theme(s, &my_theme))
    .style(|s| s.width_full())
}

fn create_configuration() -> Container {
    container(text("Configuration:")).style(|s| s.padding(10).border(Stroke::new(2.0)))
}
