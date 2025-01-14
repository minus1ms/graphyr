// data used to generate the view, it uses our main centralized data storage
#[derive(Clone)]
pub struct ViewData {
    // it points to a cell in Data
    displayed_cell: Vec<usize>,
}

impl ViewData {
    pub fn new() -> Self {
        Self {
            displayed_cell: vec![0],
        }
    }

    pub fn get_cell(&self) -> &[usize] {
        &self.displayed_cell
    }
}
