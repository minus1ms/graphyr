use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

use floem::{
    kurbo::{BezPath, Point, Rect, Stroke},
    prelude::{palette::css, RwSignal, SignalGet as _},
    views::{dyn_container, Decorators as _},
    Renderer, View, ViewId,
};

use crate::{
    data::{Arrow, Cell, CellId, Data, Layer},
    theme::MyTheme,
    utils::Segment,
    view_data::{IntoView, ViewData},
};

// we do that to draw over inner
pub struct Main {
    id: ViewId,
    data: Rc<RefCell<Data>>,
    view_data: RwSignal<ViewData>,
    layers: RwSignal<Vec<Layer>>,
    positions: HashMap<CellId, Rect>,
}

impl Main {
    pub fn new(view_data: RwSignal<ViewData>, data: Rc<RefCell<Data>>, my_theme: MyTheme) -> Self {
        let inner = {
            let data = data.clone();
            dyn_container(move || view_data.get(), {
                // the cell that we view
                move |view_data: ViewData| {
                    data.borrow_mut().cell.arrow_start_id = Some(view_data.arrow_start_id);
                    data.borrow()
                        .get_cell(&view_data.displayed_cell)
                        .into_view(my_theme.clone())
                }
            })
            .style(|s| s.size_full())
        };

        let id = ViewId::new();
        id.set_children(vec![inner]);
        Self {
            id,
            data: data.clone(),
            view_data,
            layers: data.borrow().configuration.layers,
            positions: HashMap::new(),
        }
    }

    fn handle_cell_layout(
        positions: &mut HashMap<CellId, Rect>,
        cx: &mut floem::context::ComputeLayoutCx,
        cell: &Cell,
        cell_view: ViewId,
    ) {
        let cell_text = cell_view.children()[0];
        let cell_rect = cell_text.layout_rect();
        positions.insert(cell.id.clone(), cell_rect);
        let cell_table = cell_view.children()[1]; // container made by cell

        if let Some(table) = cell.table.get_untracked().deref() {
            let cell_table = cell_table.children()[0]; // container made by table
            let v_stack = cell_table.children()[0];
            let _v_pane = v_stack.children()[0];

            let cells = table.cells.get_untracked();
            let cells_data = cells.data.borrow();
            for (row_id, row) in cells_data.iter().enumerate() {
                let h_stack = v_stack.children()[row_id + 1];
                let _h_pane = h_stack.children()[0];
                for (cell_id, cell) in row.iter().enumerate() {
                    let cell_view = h_stack.children()[cell_id + 1];
                    Self::handle_cell_layout(positions, cx, cell, cell_view);
                }
            }
        }
    }
}

impl View for Main {
    fn id(&self) -> floem::ViewId {
        self.id
    }

    fn compute_layout(
        &mut self,
        cx: &mut floem::context::ComputeLayoutCx,
    ) -> Option<floem::kurbo::Rect> {
        self.positions.clear();
        let data = self.data.borrow();

        let container = self.id.children()[0];
        let layout_rect = cx.compute_view_layout(container).unwrap();

        let cell = data.get_cell(&self.view_data.get_untracked().displayed_cell);
        let cell_view = container.children()[0];
        Self::handle_cell_layout(&mut self.positions, cx, cell, cell_view);
        Some(layout_rect)
    }

    fn paint(&mut self, cx: &mut floem::context::PaintCx) {
        cx.paint_children(self.id);
        for layer in self
            .layers
            .get_untracked()
            .iter()
            .filter(|layer| layer.enabled.get_untracked())
        {
            for Arrow { from, to } in &layer.arrows {
                let from = self.positions[from];
                let to = self.positions[to];
                let center_segment = Segment {
                    p1: from.center(),
                    p2: to.center(),
                };

                let from_cross = center_segment.intersect_rect(&from).unwrap();
                let to_cross = center_segment.intersect_rect(&to).unwrap();

                // Draw the main line.
                let mut line_path = BezPath::new();
                line_path.move_to(from_cross);
                line_path.line_to(to_cross);
                cx.stroke(&line_path, &css::RED.with_alpha(0.5), &Stroke::new(2.0));

                // Compute arrowhead at `to_cross`.
                let dx = to_cross.x - from_cross.x;
                let dy = to_cross.y - from_cross.y;
                let theta = dy.atan2(dx);
                let arrow_length = 8.0;
                let arrow_angle = std::f64::consts::PI / 6.0; // 30° angle.

                let left = Point {
                    x: to_cross.x - arrow_length * (theta + arrow_angle).cos(),
                    y: to_cross.y - arrow_length * (theta + arrow_angle).sin(),
                };
                let right = Point {
                    x: to_cross.x - arrow_length * (theta - arrow_angle).cos(),
                    y: to_cross.y - arrow_length * (theta - arrow_angle).sin(),
                };

                // Draw arrowhead as a filled triangle.
                let mut arrow_path = BezPath::new();
                arrow_path.move_to(to_cross);
                arrow_path.line_to(left);
                arrow_path.line_to(right);
                arrow_path.close_path();
                cx.fill(&arrow_path, &css::RED.with_alpha(0.5), 0.0);
            }
        }
    }
}
