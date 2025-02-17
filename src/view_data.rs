use floem::prelude::{RwSignal, SignalUpdate};

use crate::data::cell::{CellId, CellPos};

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

    pub fn reset(&mut self) {
        self.displayed_cell = CellPos::new();
        self.arrow_start_id.set(None);
    }
}
