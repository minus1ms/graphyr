use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Display,
    rc::Rc,
};

use floem::{
    kurbo::Stroke,
    menu::{Menu, MenuEntry, MenuItem},
    prelude::*,
    text::{Attrs, AttrsList, TextLayout},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::theme::MyTheme;
use crate::utils::signal_serde;

use super::{
    configuration::{arrow::Arrow, layer::Layer},
    table::Table,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
    #[serde(with = "signal_serde")]
    pub title: RwSignal<String>,
    pub id: CellId,
    #[serde(with = "signal_serde")]
    pub table: RwSignal<Option<Table>>,
    pub hierarchy_depth: usize,
}

impl Cell {
    pub fn new(table: Option<Table>, hierarchy_depth: usize) -> Self {
        Self {
            title: RwSignal::new(String::new()),
            id: CellId::new(),
            table: RwSignal::new(table),
            hierarchy_depth,
        }
    }

    pub fn build_view(
        &self,
        show_border: RwSignal<bool>,
        layers: RwSignal<Vec<Layer>>,
        arrow_start_id: RwSignal<Option<CellId>>,
        my_theme: MyTheme,
    ) -> Stack {
        let hierarchy_depth = self.hierarchy_depth;
        let size_multiplier = (100. - (hierarchy_depth * 10) as f32) / 100.;

        let table = self.table;
        v_stack((
            self.create_title(my_theme.clone(), size_multiplier),
            dyn_container(move || table.get(), {
                let my_theme = my_theme.clone();
                move |table: Option<Table>| {
                    if let Some(table) = table {
                        table
                            .build_view(show_border, layers, arrow_start_id, my_theme.clone())
                            .into_any()
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
            let s = if table.get().is_some() {
                s.gap(15. * size_multiplier)
            } else {
                s
            };
            if show_border.get() {
                s.border(Stroke::new(1.0))
            } else {
                s
            }
            .items_center()
            .justify_center()
            .hover(|s| s.background(my_theme.background_hovered))
            .size_full()
            .padding(15. * size_multiplier)
        })
        .context_menu({
            let id = self.id.clone();
            let hierarchy_depth = self.hierarchy_depth;
            move || {
                let id = id.clone();

                let res = Menu::new("");
                if table.get().is_none() {
                    let res = res.entry(MenuEntry::Item(MenuItem::new("Create table").action({
                        move || table.set(Some(Table::new(hierarchy_depth + 1)).into())
                    })));
                    if let Some(start_id) = arrow_start_id.get() {
                        let res = res.entry(MenuEntry::Item(
                            MenuItem::new("Cancel line start")
                                .action(move || arrow_start_id.set(None)),
                        ));
                        if start_id != id {
                            res.entry(MenuEntry::Item(MenuItem::new("End line").action(
                                move || {
                                    let arrow = Arrow {
                                        from: start_id.clone(),
                                        to: id.clone(),
                                    };
                                    layers.update(|layers| {
                                        for layer in layers
                                            .iter_mut()
                                            .filter(|layer| layer.enabled.get_untracked())
                                        {
                                            layer
                                                .arrows
                                                .update(|arrows| arrows.push(arrow.clone()));
                                        }
                                    });
                                    arrow_start_id.set(None)
                                },
                            )))
                        } else {
                            res
                        }
                    } else {
                        res.entry(MenuEntry::Item(MenuItem::new("Start line").action({
                            let id = id.clone();
                            move || arrow_start_id.set(Some(id.clone()))
                        })))
                    }
                } else {
                    res.entry(MenuEntry::Item(
                        MenuItem::new("Remove table").action(move || table.set(None.into())),
                    ))
                }
            }
        })
    }

    fn create_title(&self, my_theme: MyTheme, size_multiplier: f32) -> TextInput {
        let text = self.title.clone();
        text_input(text).style(move |s| {
            let font_size = 12. * size_multiplier;
            if text.get_untracked().is_empty() {
                s.background(my_theme.background)
            } else {
                s.background(my_theme.secondary_background)
            }
            .border_color(Color::TRANSPARENT)
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

    pub fn get_inner_cell(&self, id_ref: CellIdSlice) -> &Cell {
        if id_ref.is_empty() {
            return self;
        }

        let (row, col) = id_ref.top();

        let table = self.table.get_untracked();
        let cells: RawCells = table.as_ref().as_ref().unwrap().cells.get_untracked();
        println!("{row} {col}");

        let _rest = id_ref.lower();

        None.unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellId(Uuid);

impl CellId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for CellId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// points to a single position (col, row) in the entire hierarchy
#[derive(Clone)]
pub struct CellPos(Vec<(usize, usize)>);

impl CellPos {
    pub fn new() -> Self {
        Self(vec![(0, 0)])
    }

    // points to the highest cell
    pub fn top(&self) -> (usize, usize) {
        self.0[0]
    }

    // get the lower part of the id
    pub fn lower(&self) -> CellIdSlice {
        CellIdSlice(&self.0[1..])
    }
}

// a slice of CellId
pub struct CellIdSlice<'a>(&'a [(usize, usize)]);

impl<'a> CellIdSlice<'a> {
    pub fn top(&self) -> (usize, usize) {
        self.0[0]
    }

    pub fn lower(&self) -> CellIdSlice {
        CellIdSlice(&self.0[1..])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub type RowType = Vec<Cell>;
pub type RowsType = Vec<RowType>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawCells {
    pub data: Rc<RefCell<RowsType>>,
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

pub type Cells = RwSignal<RawCells>;
