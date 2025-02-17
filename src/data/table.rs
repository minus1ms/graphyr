use std::iter;

use crate::utils::signal_serde;
use floem::{
    kurbo::Stroke,
    menu::{Menu, MenuEntry, MenuItem},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::theme::MyTheme;

use super::{
    cell::{CellId, Cells, RawCells, RowType},
    configuration::layer::Layer,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Table {
    #[serde(with = "signal_serde")]
    pub cells: Cells,
}

impl Table {
    pub fn new(hierarchy_depth: usize) -> Self {
        Self {
            cells: RwSignal::new(RawCells::new(hierarchy_depth)),
        }
    }

    pub fn build_view(
        &self,
        show_border_signal: RwSignal<bool>,
        layers: RwSignal<Vec<Layer>>,
        arrow_start_id: RwSignal<Option<CellId>>,
        my_theme: MyTheme,
    ) -> DynamicContainer<(RawCells, bool)> {
        let cells = self.cells;
        dyn_container(
            move || (cells.get(), show_border_signal.get()),
            move |(raw_cells, show_border): (RawCells, _)| {
                v_stack_from_iter(
                    iter::once(if show_border {
                        let row: &RowType = &raw_cells.borrow_row(0);
                        h_stack_from_iter((0..row.len()).map(|i| {
                            // this is a column pane, in the first of the rows
                            Self::create_pane(
                                my_theme.clone(),
                                cells,
                                true,
                                i,
                                raw_cells.rows(),
                                raw_cells.cols(),
                            )
                        }))
                        .into_any()
                    } else {
                        empty().into_any()
                    })
                    .chain((0..raw_cells.rows()).map(|i| {
                        let row: &RowType = &raw_cells.borrow_row(i);
                        h_stack_from_iter(
                            iter::once(if show_border {
                                // this is a row pane, first of the columns
                                Self::create_pane(
                                    my_theme.clone(),
                                    cells,
                                    false,
                                    i,
                                    raw_cells.rows(),
                                    raw_cells.cols(),
                                )
                                .into_any()
                            } else {
                                empty().into_any()
                            })
                            .chain((0..raw_cells.cols()).map(|i| {
                                row[i]
                                    .build_view(
                                        show_border_signal,
                                        layers,
                                        arrow_start_id,
                                        my_theme.clone(),
                                    )
                                    .into_any()
                            })),
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

    fn create_pane(
        my_theme: MyTheme,
        cells: Cells,
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
