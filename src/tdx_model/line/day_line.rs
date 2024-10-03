use serde::{Deserialize, Serialize};

use crate::tdx_model::data::DailyData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayLine {
    pub data: Vec<DailyData>,
}

impl DayLine {
    pub fn new(data: Vec<DailyData>) -> Self {
        Self { data }
    }

    pub fn inner(self) -> Vec<DailyData> {
        self.data
    }

    pub fn inner_ref(&self) -> &[DailyData] {
        self.data.as_ref()
    }
}