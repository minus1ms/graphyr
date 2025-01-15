use std::iter;

use floem::{
    kurbo::Stroke,
    menu::{Menu, MenuEntry, MenuItem},
    peniko::Color,
    prelude::{RwSignal, SignalGet as _, SignalUpdate as _},
    taffy::Position,
    text::{Attrs, AttrsList, TextLayout},
    views::{
        dyn_container, empty, h_stack_from_iter, text_input, v_stack, v_stack_from_iter,
        Decorators as _, Empty, TextInput,
    },
    IntoView, View,
};

use crate::{theme::MyTheme, view_data::ViewData};

#[derive(Clone)]
pub struct CellId(Vec<usize>);

impl CellId {
    pub fn new() -> Self {
        Self(vec![0])
    }

    pub fn current(&self) -> usize {
        self.0[0]
    }

    pub fn next(&self) -> CellIdRef {
        CellIdRef(&self.0[1..])
    }
}

pub struct CellIdRef<'a>(&'a [usize]);

impl<'a> CellIdRef<'a> {
    pub fn current(&self) -> usize {
        self.0[0]
    }

    pub fn next(&self) -> CellIdRef {
        CellIdRef(&self.0[1..])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone)]
pub struct Cell {
    title: RwSignal<String>,
    table: RwSignal<Option<Table>>,
    hierarchy_depth: usize,
}

impl Cell {
    pub fn new(table: Option<Table>, hierarchy_depth: usize) -> Self {
        Self {
            title: RwSignal::new(String::new()),
            table: RwSignal::new(table),
            hierarchy_depth,
        }
    }

    pub fn into_view(&self, my_theme: MyTheme) -> impl View {
        let hierarchy_depth = self.hierarchy_depth;
        let size_multiplier = (100. - (hierarchy_depth * 10) as f32) / 100.;

        let table = self.table.clone();
        v_stack((
            self.create_cell_title(my_theme.clone(), size_multiplier),
            dyn_container(move || table.get(), {
                let my_theme = my_theme.clone();
                move |table: Option<Table>| {
                    if let Some(table) = table {
                        table.into_view(my_theme.clone()).into_any()
                    } else {
                        empty().into_any()
                    }
                }
            })
            .style(move |s| {
                if table.get().is_some() {
                    s.size_full()
                } else {
                    s
                }
            }),
        ))
        .style(move |s| {
            if table.get().is_some() {
                s.gap(15. * size_multiplier)
            } else {
                s
            }
            .border(Stroke::new(1.0))
            .items_center()
            .justify_center()
            .hover(|s| s.background(my_theme.background_hovered))
            .size_full()
            .padding(15. * size_multiplier)
        })
        .context_menu({
            let hierarchy_depth = self.hierarchy_depth;
            move || {
                let res = Menu::new("");
                if table.get().is_none() {
                    res.entry(MenuEntry::Item(MenuItem::new("Create table").action(
                        move || {
                            table.set(Some(Table {
                                cells: vec![vec![Cell::new(None, hierarchy_depth + 1)]],
                            }))
                        },
                    )))
                } else {
                    res.entry(MenuEntry::Item(
                        MenuItem::new("Remove table").action(move || table.set(None)),
                    ))
                }
            }
        })
    }

    fn create_cell_title(&self, my_theme: MyTheme, size_multiplier: f32) -> TextInput {
        let text = self.title.clone();
        text_input(text).style(move |s| {
            let font_size = 12. * size_multiplier;
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

    pub fn get_inner_cell(&self, id_ref: CellIdRef) -> &Cell {
        if id_ref.is_empty() {
            return self;
        }

        let first = id_ref.current();

        let cells = &self.table.get_untracked().unwrap().cells;
        let rows = cells.len(); // single col size
        let cols = cells[0].len(); // single row size
        let row = first / cols;
        let col = rows - first * row;
        println!("{row} {col}");

        let rest = id_ref.next();

        None.unwrap()
    }
}

#[derive(Clone)]
pub struct Table {
    cells: Vec<Vec<Cell>>,
}

impl Table {
    pub fn into_view(&self, my_theme: MyTheme) -> impl View {
        v_stack_from_iter(
            iter::once(
                // this is a column pane, first of the rows
                Self::create_table_pane(my_theme.clone(), true).into_any(),
            )
            .chain((0..self.cells.len()).map(|i| {
                let row = &self.cells[i];
                h_stack_from_iter(
                    iter::once(
                        // this is a row pane, first of the columns
                        Self::create_table_pane(my_theme.clone(), false).into_any(),
                    )
                    .chain((0..row.len()).map(|i| row[i].into_view(my_theme.clone()).into_any())),
                )
                .style(|s| s.size_full())
                .into_any()
            })),
        )
        .style(|s| s.size_full())
    }

    fn create_table_pane(my_theme: MyTheme, column: bool) -> Empty {
        let size = 20;
        if column {
            empty().context_menu(|| {
                Menu::new("")
                    .entry(MenuEntry::SubMenu(
                        Menu::new("Add")
                            .entry(MenuEntry::Item(MenuItem::new("PRE")))
                            .entry(MenuEntry::Item(MenuItem::new("POST"))),
                    ))
                    .entry(MenuEntry::Item(MenuItem::new("Remove")))
            })
        } else {
            empty().context_menu(|| {
                Menu::new("")
                    .entry(MenuEntry::SubMenu(
                        Menu::new("Add")
                            .entry(MenuEntry::Item(MenuItem::new("PRE")))
                            .entry(MenuEntry::Item(MenuItem::new("POST"))),
                    ))
                    .entry(MenuEntry::Item(MenuItem::new("Remove")))
            })
        }
        .style(move |s| {
            if column {
                s.height(size).width_full()
            } else {
                s.width(size).height_full()
            }
            .background(my_theme.secondary_background)
            .border(Stroke::new(1.0))
        })
    }
}

// this is a main centralized storage
pub struct Data {
    // our hierarchy starts with a single cell
    cell: Cell,
}

impl Data {
    pub fn new() -> Self {
        Self {
            cell: Cell::new(None, 0),
        }
    }

    pub fn get_cell(&self, place: &ViewData) -> &Cell {
        let cell_id = place.get_cell();
        let first = cell_id.current();
        assert!(first == 0);
        self.cell.get_inner_cell(cell_id.next())
    }
}
