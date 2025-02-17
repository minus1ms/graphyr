use serde::{Deserialize, Serialize};

use crate::data::cell::CellId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Arrow {
    pub from: CellId,
    pub to: CellId,
}
