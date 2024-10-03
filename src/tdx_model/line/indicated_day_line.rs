use serde::{Deserialize, Serialize};

use crate::tdx_model::{CombinedIndicator, ShortIndicator};

#[cfg(feature = "analysis")]
use crate::tdx_model::Indicator;

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

    #[cfg(feature = "analysis")]
    pub fn short_indicator(&self) -> Vec<Indicator> {
        self.data.iter()
            .map(|data| 
                (data.date, data.macd).into()
            )
            .collect()
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

    #[cfg(feature = "analysis")]
    pub fn short_indicator(&self) -> Vec<Indicator> {
        self.data.iter()
            .map(|data| 
                (data.date, data.macd).into()
            )
            .collect()
    }

    #[cfg(feature = "analysis")]
    pub fn long_indicator(&self) -> Vec<Indicator> {
        self.data.iter()
            .map(|data| 
                (data.date, data.macd_2).into()
            )
            .collect()
    }
}