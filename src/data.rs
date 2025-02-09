use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use floem::prelude::{RwSignal, SignalGet as _};
use uuid::Uuid;

use crate::view_data::ViewData;

// this is a main centralized storage, it is immutable, things inside have interior mutability
pub struct Data {
    // our hierarchy starts with a single cell
    pub cell: Cell,
    pub configuration: Configuration,
}

impl Data {
    pub fn new() -> Self {
        let configuration = Configuration::new();
        Self {
            cell: Cell::new(
                None,
                0,
                CellGlobalSettings {
                    show_border: configuration.show_border,
                    layers: configuration.layers,
                },
                None,
            ),
            configuration,
        }
    }

    pub fn get_cell(&self, place: &CellPos) -> &Cell {
        let first = place.top();
        assert!(first == (0, 0));
        self.cell.get_inner_cell(place.lower())
    }
}

#[derive(Clone)]
pub struct Arrow {
    pub from: CellId,
    pub to: CellId,
}

#[derive(Clone)]
pub struct Layer {
    pub name: String,
    pub enabled: RwSignal<bool>,
    pub arrows: Vec<Arrow>,
}

pub struct Configuration {
    pub show_border: RwSignal<bool>,
    pub layers: RwSignal<Vec<Layer>>,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            show_border: RwSignal::new(true),
            layers: RwSignal::new(vec![Layer {
                name: "First".into(),
                enabled: RwSignal::new(false),
                arrows: vec![],
            }]),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CellId(Uuid);

impl CellId {
    fn new() -> Self {
        Self(Uuid::new_v4())
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

#[derive(Clone)]
pub struct CellGlobalSettings {
    pub show_border: RwSignal<bool>,
    pub layers: RwSignal<Vec<Layer>>,
}

pub struct Cell {
    pub title: RwSignal<String>,
    pub id: CellId,
    pub table: RwSignal<Rc<Option<Table>>>,
    pub hierarchy_depth: usize,
    pub global_settings: CellGlobalSettings,
    // temporary global settings
    pub arrow_start_id: Option<RwSignal<Option<CellId>>>,
}

impl Cell {
    pub fn new(
        table: Option<Table>,
        hierarchy_depth: usize,
        global_settings: CellGlobalSettings,
        arrow_start_id: Option<RwSignal<Option<CellId>>>,
    ) -> Self {
        Self {
            title: RwSignal::new(String::new()),
            id: CellId::new(),
            table: RwSignal::new(Rc::new(table)),
            hierarchy_depth,
            global_settings,
            arrow_start_id,
        }
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

pub type RowType = Vec<Cell>;
pub type RowsType = Vec<RowType>;

#[derive(Clone)]
pub struct RawCells {
    pub data: Rc<RefCell<RowsType>>,
    hierarchy_depth: usize,
    cell_global_settings: CellGlobalSettings,
    arrow_start_id: RwSignal<Option<CellId>>,
}

impl RawCells {
    pub fn new(
        hierarchy_depth: usize,
        cell_global_settings: CellGlobalSettings,
        arrow_start_id: RwSignal<Option<CellId>>,
    ) -> Self {
        Self {
            data: Rc::new(RefCell::new(vec![vec![Cell::new(
                None,
                hierarchy_depth,
                cell_global_settings.clone(),
                Some(arrow_start_id),
            )]])),
            hierarchy_depth,
            cell_global_settings,
            arrow_start_id,
        }
    }

    pub fn add_row(&self, index: usize) {
        let cols = self.cols();
        self.borrow_mut_rows().insert(
            index,
            (0..cols)
                .map(|_| {
                    Cell::new(
                        None,
                        self.hierarchy_depth,
                        self.cell_global_settings.clone(),
                        Some(self.arrow_start_id),
                    )
                })
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
                Cell::new(
                    None,
                    self.hierarchy_depth,
                    self.cell_global_settings.clone(),
                    Some(self.arrow_start_id),
                ),
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
    pub fn new(
        hierarchy_depth: usize,
        cell_global_settings: CellGlobalSettings,
        arrow_start_id: RwSignal<Option<CellId>>,
    ) -> Self {
        Self {
            show_border: cell_global_settings.show_border.clone(),
            cells: RwSignal::new(RawCells::new(
                hierarchy_depth,
                cell_global_settings,
                arrow_start_id,
            )),
        }
    }
}
