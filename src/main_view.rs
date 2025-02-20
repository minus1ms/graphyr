use std::{collections::HashMap, rc::Rc};

use floem::{
    kurbo::{BezPath, Point, Rect, Stroke},
    prelude::{RwSignal, SignalGet as _},
    views::{dyn_container, Decorators as _},
    Renderer, View, ViewId,
};

use crate::{
    data::{
        cell::{Cell, CellId},
        configuration::{arrow::Arrow, layer::Layer, Configuration},
        Data,
    },
    theme::MyTheme,
    utils::{compute_path, Segment},
    view_data::ViewData,
};

// we do that to draw over inner
pub struct Main {
    id: ViewId,
    view_data: RwSignal<ViewData>,
    main_cell: Rc<Cell>,
    layers: RwSignal<Vec<Layer>>,
    positions: HashMap<CellId, Rect>,
}

impl Main {
    pub fn new(
        view_data: RwSignal<ViewData>,
        configuration: &Configuration,
        main_cell: Rc<Cell>,
        my_theme: MyTheme,
    ) -> Self {
        let layers = configuration.layers;
        let inner = dyn_container(move || view_data.get(), {
            // the cell that we view
            {
                let show_border = configuration.show_border;
                let show_panes = configuration.show_panes;
                let main_cell = main_cell.clone();
                move |view_data: ViewData| {
                    Data::get_cell(&main_cell, &view_data.displayed_cell).build_view(
                        show_border,
                        show_panes,
                        layers,
                        view_data.arrow_start_id,
                        my_theme.clone(),
                    )
                }
            }
        })
        .style(|s| s.size_full());

        let id = ViewId::new();
        id.set_children(vec![inner]);
        Self {
            id,
            view_data,
            main_cell,
            layers,
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

        if let Some(table) = cell.table.get_untracked() {
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

        let container = self.id.children()[0];
        let layout_rect = cx.compute_view_layout(container).unwrap();

        let cell = Data::get_cell(
            &self.main_cell,
            &self.view_data.get_untracked().displayed_cell,
        );
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
            for Arrow {
                from,
                to,
                color,
                padding,
            } in &layer.arrows.get_untracked()
            {
                let from_rect = self.positions[from];
                let to_rect = self.positions[to];
                let center_segment = Segment {
                    p1: from_rect.center(),
                    p2: to_rect.center(),
                };

                let from_cross = center_segment.intersect_rect(&from_rect).unwrap();
                let to_cross = center_segment.intersect_rect(&to_rect).unwrap();

                // Pathfinding
                let margin = padding.get_untracked();
                let rest_rects = self
                    .positions
                    .iter()
                    .filter_map(|(id, rect)| {
                        if id != from && id != to {
                            Some(rect.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let path = compute_path(&rest_rects, &from_cross, &to_cross, margin).unwrap();

                // Draw the main line.
                let mut line_path = BezPath::new();
                line_path.move_to(path[0]);

                for point in &path[1..] {
                    line_path.line_to(point.clone());
                }

                let from_cross = path[path.len() - 2];

                cx.stroke(&line_path, &color.get_untracked(), &Stroke::new(2.0));

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
                cx.fill(&arrow_path, &color.get_untracked(), 0.0);
            }
        }
    }
}
