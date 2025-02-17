use std::rc::Rc;

use cell::{Cell, CellPos};
use configuration::Configuration;
use floem::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{main_view::Main, theme::MyTheme, view_data::ViewData};

pub mod cell;
pub mod configuration;
mod table;

// this is a main centralized storage
#[derive(Clone, Deserialize, Serialize)]
pub struct Data {
    // our hierarchy starts with a single cell, Rc because it has to be 'static when view exists
    pub cell: Rc<Cell>,
    pub configuration: Configuration,
}

impl Data {
    pub fn new() -> Self {
        let configuration = Configuration::new();
        Self {
            cell: Rc::new(Cell::new(None, 0)),
            configuration,
        }
    }

    pub fn build_view(
        &self,
        view_data: RwSignal<ViewData>,
        // a crazy thing that just refers for de/serialization
        data: RwSignal<Data>,
        my_theme: MyTheme,
    ) -> Stack {
        h_stack((
            Main::new(view_data, &self.configuration, self.cell.clone(), my_theme)
                .style(|s| s.size_full()),
            self.configuration.build_view(data),
        ))
    }

    pub fn get_cell<'a>(main_cell: &'a Cell, place: &CellPos) -> &'a Cell {
        let first = place.top();
        assert!(first == (0, 0));
        main_cell.get_inner_cell(place.lower())
    }
}
