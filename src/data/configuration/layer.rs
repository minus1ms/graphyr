use crate::utils::signal_serde;
use floem::prelude::*;
use serde::{Deserialize, Serialize};

use super::arrow::Arrow;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Layer {
    #[serde(with = "signal_serde")]
    pub name: RwSignal<String>,
    #[serde(with = "signal_serde")]
    pub enabled: RwSignal<bool>,
    #[serde(with = "signal_serde")]
    pub arrows: RwSignal<Vec<Arrow>>,
}

impl Layer {
    pub fn new() -> Self {
        Self {
            name: RwSignal::new("Unnamed".into()),
            enabled: RwSignal::new(false),
            arrows: RwSignal::new(vec![]),
        }
    }
}
