use std::{iter, ops::Deref, rc::Rc};

use floem::{
    kurbo::Stroke,
    menu::{Menu, MenuEntry, MenuItem},
    peniko::Color,
    prelude::{RwSignal, SignalGet, SignalUpdate},
    text::{Attrs, AttrsList, TextLayout},
    views::{
        dyn_container, empty, h_stack_from_iter, text_input, v_stack, v_stack_from_iter,
        Decorators, DynamicContainer, Empty, Stack, TextInput,
    },
    IntoView as _, View,
};

use crate::{
    data::{Arrow, Cell, CellId, CellPos, CellsType, RawCells, RowType, Table},
    theme::MyTheme,
};

// data used to generate the view, it uses our main centralized data storage
#[derive(Clone)]
pub struct ViewData {
    // it points to a cell in Data
    pub displayed_cell: CellPos,
    // temporary value used to determine current arrow creation
    pub arrow_start_id: RwSignal<Option<CellId>>,
}

impl ViewData {
    pub fn new() -> Self {
        Self {
            displayed_cell: CellPos::new(),
            arrow_start_id: RwSignal::new(None),
        }
    }
}

pub trait IntoView: Sized {
    type V: View + 'static;

    fn into_view(&self, my_theme: MyTheme) -> Self::V;
}

impl IntoView for Cell {
    type V = Stack;

    fn into_view(&self, my_theme: MyTheme) -> Self::V {
        fn create_cell_title(
            title: RwSignal<String>,
            my_theme: MyTheme,
            size_multiplier: f32,
        ) -> TextInput {
            let text = title.clone();
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

        let hierarchy_depth = self.hierarchy_depth;
        let size_multiplier = (100. - (hierarchy_depth * 10) as f32) / 100.;

        let table = self.table;
        let cell_global_settings = self.global_settings.clone();
        v_stack((
            create_cell_title(self.title, my_theme.clone(), size_multiplier),
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
            let s = if table.get().is_some() {
                s.gap(15. * size_multiplier)
            } else {
                s
            };
            if cell_global_settings.show_border.get() {
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
            let arrow_start_id = self.arrow_start_id.unwrap();
            move || {
                let id = id.clone();

                let res = Menu::new("");
                if table.get().is_none() {
                    let res = res.entry(MenuEntry::Item(MenuItem::new("Create table").action({
                        let cell_global_settings = cell_global_settings.clone();
                        move || {
                            table.set(
                                Some(Table::new(
                                    hierarchy_depth + 1,
                                    cell_global_settings.clone(),
                                    arrow_start_id,
                                ))
                                .into(),
                            )
                        }
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
                                    cell_global_settings.layers.update(|layers| {
                                        for layer in layers {
                                            layer.arrows.push(arrow.clone());
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
}

impl IntoView for Table {
    type V = DynamicContainer<(RawCells, bool)>;

    fn into_view(&self, my_theme: MyTheme) -> Self::V {
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

        let cells = self.cells;
        let show_border = self.show_border;
        dyn_container(
            move || (cells.get(), show_border.get()),
            move |(raw_cells, show_border): (RawCells, _)| {
                v_stack_from_iter(
                    iter::once(if show_border {
                        let row: &RowType = &raw_cells.borrow_row(0);
                        h_stack_from_iter((0..row.len()).map(|i| {
                            // this is a column pane, in the first of the rows
                            create_table_pane(
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
                                create_table_pane(
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
}
