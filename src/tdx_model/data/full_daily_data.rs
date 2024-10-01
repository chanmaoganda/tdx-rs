use serde::{Deserialize, Serialize};

use super::{CombinedIndicator, DailyData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullDailyData {
    pub date: u32,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub dif: f64,
    pub dea: f64,
    pub macd: f64,
    pub dif_2: f64,
    pub dea_2: f64,
    pub macd_2: f64,
}

impl From<(DailyData, CombinedIndicator)> for FullDailyData {
    fn from(value: (DailyData, CombinedIndicator)) -> Self {
        let (daily_data, combined_indicator) = value;

        let (daily_date, open, high, low, close) = daily_data.into();
        let (indicator_date, dif, dea, macd, dif_2, dea_2, macd_2) = combined_indicator.into();
        assert_eq!(daily_date, indicator_date);

        FullDailyData {
            date: daily_date,
            open,
            high,
            low,
            close,
            dif,
            dea,
            macd,
            dif_2,
            dea_2,
            macd_2,
        }
    }
}
