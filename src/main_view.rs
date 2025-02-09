use std::{cell::RefCell, collections::HashMap, rc::Rc};

use floem::{
    kurbo::{BezPath, Point, Stroke},
    peniko::Color,
    prelude::{RwSignal, SignalGet as _},
    views::{dyn_container, Decorators as _},
    Renderer, View, ViewId,
};

use crate::{
    data::{Arrow, CellId, Data, Layer},
    theme::MyTheme,
    view_data::{IntoView, ViewData},
};

// we do that to draw over inner
pub struct Main {
    id: ViewId,
    layers: RwSignal<Vec<Layer>>,
    positions: HashMap<CellId, Point>,
}

impl Main {
    pub fn new(view_data: RwSignal<ViewData>, data: Rc<RefCell<Data>>, my_theme: MyTheme) -> Self {
        let inner = {
            let data = data.clone();
            dyn_container(move || view_data.get(), {
                // the cell that we view
                move |view_data: ViewData| {
                    data.borrow_mut().cell.arrow_start_pos = Some(view_data.arrow_start_pos);
                    data.borrow()
                        .get_cell(&view_data)
                        .into_view(my_theme.clone())
                }
            })
            .style(|s| s.size_full())
        };

        let id = ViewId::new();
        id.set_children(vec![inner]);
        Self {
            id,
            layers: data.borrow().configuration.layers,
            positions: HashMap::new(),
        }
    }
}

impl View for Main {
    fn id(&self) -> floem::ViewId {
        self.id
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
                let mut path = BezPath::new();
                path.move_to(from);
                path.line_to(to);
                cx.stroke(&path, &Color::RED, &Stroke::new(2.0));
            }
        }
    }
}
