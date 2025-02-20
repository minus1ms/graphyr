use crate::utils::signal_serde;
use floem::{peniko::Color, prelude::RwSignal};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::data::cell::CellId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Arrow {
    pub from: CellId,
    pub to: CellId,
    #[serde(with = "signal_serde")]
    pub color: RwSignal<Color>,
    #[serde(with = "signal_serde")]
    pub padding: RwSignal<f64>,
}

impl Arrow {
    pub fn new(from: CellId, to: CellId) -> Self {
        let mut rng = rand::rng();
        Self {
            from,
            to,
            color: RwSignal::new(Color::from_rgb8(
                rng.random_range(0..=255),
                rng.random_range(0..=255),
                rng.random_range(0..=255),
            )),
            padding: RwSignal::new(rng.random_range(3.0..15.0)),
        }
    }
}
