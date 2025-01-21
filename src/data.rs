use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use floem::prelude::{RwSignal, SignalGet as _};

use crate::view_data::ViewData;

// this is a main centralized storage, it is immutable, things inside have interior mutability
pub struct Data {
    // our hierarchy starts with a single cell
    cell: Cell,
    pub configuration: Configuration,
}

impl Data {
    pub fn new() -> Self {
        let configuration = Configuration::new();
        Self {
            cell: Cell::new(None, 0, configuration.show_border),
            configuration,
        }
    }

    pub fn get_cell(&self, place: &ViewData) -> &Cell {
        let cell_id = place.get_cell();
        let first = cell_id.current();
        assert!(first == 0);
        self.cell.get_inner_cell(cell_id.next())
    }
}

pub struct Configuration {
    pub show_border: RwSignal<bool>,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            show_border: RwSignal::new(true),
        }
    }
}

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
    pub title: RwSignal<String>,
    pub table: RwSignal<Rc<Option<Table>>>,
    pub hierarchy_depth: usize,
    pub show_border: RwSignal<bool>,
}

impl Cell {
    pub fn new(table: Option<Table>, hierarchy_depth: usize, show_border: RwSignal<bool>) -> Self {
        Self {
            title: RwSignal::new(String::new()),
            table: RwSignal::new(Rc::new(table)),
            hierarchy_depth,
            show_border,
        }
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

        let _rest = id_ref.next();

        None.unwrap()
    }
}

pub type RowType = Vec<Cell>;
pub type RowsType = Vec<RowType>;

#[derive(Clone)]
pub struct RawCells {
    data: Rc<RefCell<RowsType>>,
    hierarchy_depth: usize,
    show_border: RwSignal<bool>,
}

impl RawCells {
    pub fn new(hierarchy_depth: usize, show_border: RwSignal<bool>) -> Self {
        Self {
            data: Rc::new(RefCell::new(vec![vec![Cell::new(
                None,
                hierarchy_depth,
                show_border,
            )]])),
            hierarchy_depth,
            show_border,
        }
    }

    pub fn add_row(&self, index: usize) {
        let cols = self.cols();
        self.borrow_mut_rows().insert(
            index,
            (0..cols)
                .map(|_| Cell::new(None, self.hierarchy_depth, self.show_border))
                .collect(),
        );
    }

    pub fn remove_row(&self, index: usize) {
        self.borrow_mut_rows().remove(index);
    }

    pub fn add_col(&self, index: usize) {
        for row in self.borrow_mut_rows().iter_mut() {
            row.insert(
                index,
                Cell::new(None, self.hierarchy_depth, self.show_border),
            );
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

pub type CellsType = RwSignal<RawCells>;

pub struct Table {
    pub cells: CellsType,
    pub show_border: RwSignal<bool>,
}

impl Table {
    pub fn new(hierarchy_depth: usize, show_border: RwSignal<bool>) -> Self {
        Self {
            cells: RwSignal::new(RawCells::new(hierarchy_depth, show_border)),
            show_border,
        }
    }
}
