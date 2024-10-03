use serde::{Deserialize, Serialize};

use crate::tdx_model::{CombinedIndicator, ShortIndicator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortIndicatorDayLine {
    pub data: Vec<ShortIndicator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedIndicatorDayLine {
    pub data: Vec<CombinedIndicator>,
}

impl ShortIndicatorDayLine {
    pub fn new(data: Vec<ShortIndicator>) -> Self {
        Self { data }
    }

    pub fn inner(self) -> Vec<ShortIndicator> {
        self.data
    }

    pub fn inner_ref(&self) -> &[ShortIndicator] {
        self.data.as_ref()
    }
}

impl CombinedIndicatorDayLine {
    pub fn new(data: Vec<CombinedIndicator>) -> Self {
        Self { data }
    }

    pub fn inner(self) -> Vec<CombinedIndicator> {
        self.data
    }

    pub fn inner_ref(&self) -> &[CombinedIndicator] {
        self.data.as_ref()
    }
}