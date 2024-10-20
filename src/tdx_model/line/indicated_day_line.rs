use serde::{Deserialize, Serialize};

use crate::tdx_model::{CombinedIndicator, SingleIndicator};

#[cfg(feature = "analysis")]
use crate::tdx_model::Indicator;

use super::QueryLine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleIndicatorDayLine {
    pub data: Vec<SingleIndicator>,
}

impl SingleIndicatorDayLine {
    pub fn new(data: Vec<SingleIndicator>) -> Self {
        Self { data }
    }

    pub fn inner(self) -> Vec<SingleIndicator> {
        self.data
    }

    pub fn inner_ref(&self) -> &[SingleIndicator] {
        self.data.as_ref()
    }

    #[cfg(feature = "analysis")]
    pub fn single_indicator(&self) -> Vec<Indicator> {
        self.data.iter()
            .map(|data| 
                Indicator::new(data.date, data.macd)
            )
            .collect()
    }
}

impl QueryLine for SingleIndicatorDayLine {
    type Output = SingleIndicator;

    fn query_data(&self, range: std::ops::Range<usize>) -> &[Self::Output] {
        &self.data[range]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedIndicatorDayLine {
    pub data: Vec<CombinedIndicator>,
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
                Indicator::new(data.date, data.macd)
            )
            .collect()
    }

    #[cfg(feature = "analysis")]
    pub fn long_indicator(&self) -> Vec<Indicator> {
        self.data.iter()
            .map(|data| 
                Indicator::new(data.date, data.macd_2)
            )
            .collect()
    }

    pub fn split_single_indicator(self) -> (SingleIndicatorDayLine, SingleIndicatorDayLine) {
        let mut short_line = Vec::with_capacity(self.data.len());
        let mut long_line = Vec::with_capacity(self.data.len());
        self.data.into_iter()
            .map(|data| data.split())
            .for_each(|(short, long)| {
                short_line.push(short);
                long_line.push(long);
            });
        (SingleIndicatorDayLine::new(short_line), SingleIndicatorDayLine::new(long_line))
    }
}

impl QueryLine for CombinedIndicatorDayLine {
    type Output = CombinedIndicator;

    fn query_data(&self, range: std::ops::Range<usize>) -> &[Self::Output] {
        &self.data[range]
    }
}