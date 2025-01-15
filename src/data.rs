use std::{
    cell::{Ref, RefCell, RefMut},
    iter,
    ops::Deref,
    rc::Rc,
};

use floem::{
    kurbo::Stroke,
    menu::{Menu, MenuEntry, MenuItem},
    peniko::Color,
    prelude::{RwSignal, SignalGet as _, SignalUpdate as _},
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

pub struct Cell {
    title: RwSignal<String>,
    table: RwSignal<Rc<Option<Table>>>,
    hierarchy_depth: usize,
}

impl Cell {
    pub fn new(table: Option<Table>, hierarchy_depth: usize) -> Self {
        Self {
            title: RwSignal::new(String::new()),
            table: RwSignal::new(Rc::new(table)),
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
                move |table: Rc<Option<Table>>| {
                    if let Some(table) = table.deref() {
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
                        move || table.set(Some(Table::new(hierarchy_depth + 1)).into()),
                    )))
                } else {
                    res.entry(MenuEntry::Item(
                        MenuItem::new("Remove table").action(move || table.set(None.into())),
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

        let table = self.table.get_untracked();
        let cells: RawCells = table.as_ref().as_ref().unwrap().cells.get_untracked();
        let rows = cells.rows(); // single col size
        let cols = cells.cols(); // single row size
        let row = first / cols;
        let col = rows - first * row;
        println!("{row} {col}");

        let rest = id_ref.next();

        None.unwrap()
    }
}

type RowType = Vec<Cell>;
type RowsType = Vec<RowType>;

#[derive(Clone)]
struct RawCells {
    data: Rc<RefCell<RowsType>>,
    hierarchy_depth: usize,
}

impl RawCells {
    pub fn new(hierarchy_depth: usize) -> Self {
        Self {
            data: Rc::new(RefCell::new(vec![vec![Cell::new(None, hierarchy_depth)]])),
            hierarchy_depth,
        }
    }

    pub fn add_row(&self, index: usize) {
        let cols = self.cols();
        self.borrow_mut_rows().insert(
            index,
            (0..cols)
                .map(|_| Cell::new(None, self.hierarchy_depth))
                .collect(),
        );
    }

    pub fn remove_row(&self, index: usize) {
        self.borrow_mut_rows().remove(index);
    }

    pub fn add_col(&self, index: usize) {
        for row in self.borrow_mut_rows().iter_mut() {
            row.insert(index, Cell::new(None, self.hierarchy_depth));
        }
    }

    pub fn remove_col(&self, index: usize) {
        for row in self.borrow_mut_rows().iter_mut() {
            row.remove(index);
        }
    }

    pub fn rows(&self) -> usize {
        self.data.borrow().len()
    }

    pub fn cols(&self) -> usize {
        self.data.borrow()[0].len()
    }

    pub fn borrow_mut_rows(&self) -> RefMut<RowsType> {
        self.data.borrow_mut()
    }

    pub fn borrow_row(&self, index: usize) -> Ref<RowType> {
        Ref::map(self.data.borrow(), |cells| &cells[index])
    }

    pub fn borrow_mut_row(&self, index: usize) -> RefMut<RowType> {
        RefMut::map(self.data.borrow_mut(), |cells| &mut cells[index])
    }
}

type CellsType = RwSignal<RawCells>;

pub struct Table {
    cells: CellsType,
}

impl Table {
    pub fn new(hierarchy_depth: usize) -> Self {
        Self {
            cells: RwSignal::new(RawCells::new(hierarchy_depth)),
        }
    }

    pub fn into_view(&self, my_theme: MyTheme) -> impl View {
        let cells = self.cells.clone();
        dyn_container(
            move || cells.get(),
            move |raw_cells: RawCells| {
                v_stack_from_iter(
                    iter::once({
                        let row: &RowType = &raw_cells.borrow_row(0);
                        h_stack_from_iter((0..row.len()).map(|i| {
                            // this is a column pane, in the first of the rows
                            Self::create_table_pane(
                                my_theme.clone(),
                                cells,
                                true,
                                i,
                                raw_cells.rows(),
                                raw_cells.cols(),
                            )
                        }))
                        .into_any()
                    })
                    .chain((0..raw_cells.rows()).map(|i| {
                        let row: &RowType = &raw_cells.borrow_row(i);
                        h_stack_from_iter(
                            iter::once(
                                // this is a row pane, first of the columns
                                Self::create_table_pane(
                                    my_theme.clone(),
                                    cells,
                                    false,
                                    i,
                                    raw_cells.rows(),
                                    raw_cells.cols(),
                                )
                                .into_any(),
                            )
                            .chain(
                                (0..raw_cells.cols())
                                    .map(|i| row[i].into_view(my_theme.clone()).into_any()),
                            ),
                        )
                        .style(|s| s.size_full())
                        .into_any()
                    })),
                )
                .style(|s| s.size_full())
            },
        )
        .style(|s| s.size_full())
    }

    fn create_table_pane(
        my_theme: MyTheme,
        cells: CellsType,
        column: bool,
        index: usize,
        rows: usize,
        cols: usize,
    ) -> Empty {
        let size = 20;
        empty()
            .context_menu(move || {
                let res = Menu::new("").entry(MenuEntry::SubMenu(
                    Menu::new("Add")
                        .entry(MenuEntry::Item(MenuItem::new("PRE").action(move || {
                            cells.update(|cells| {
                                if column {
                                    cells.add_col(index);
                                } else {
                                    cells.add_row(index);
                                }
                            });
                        })))
                        .entry(MenuEntry::Item(MenuItem::new("POST").action(move || {
                            cells.update(|cells| {
                                if column {
                                    cells.add_col(index + 1);
                                } else {
                                    cells.add_row(index + 1);
                                }
                            });
                        }))),
                ));
                if if column { cols } else { rows } > 1 {
                    res.entry(MenuEntry::Item(MenuItem::new("Remove").action(move || {
                        cells.update(|cells| {
                            if column {
                                cells.remove_col(index);
                            } else {
                                cells.remove_row(index);
                            }
                        });
                    })))
                } else {
                    res
                }
            })
            .style(move |s| {
                if column {
                    if index == 0 { s.margin_left(size) } else { s }
                        .min_height(size)
                        .width_full()
                } else {
                    s.min_width(size).height_full()
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
