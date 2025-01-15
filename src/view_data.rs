use crate::data::CellId;

// data used to generate the view, it uses our main centralized data storage
#[derive(Clone)]
pub struct ViewData {
    // it points to a cell in Data
    displayed_cell: CellId,
}

impl ViewData {
    pub fn new() -> Self {
        Self {
            displayed_cell: CellId::new(),
        }
    }

    pub fn get_cell(&self) -> &CellId {
        &self.displayed_cell
    }
}
