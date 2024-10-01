use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyData {
    pub date: u32,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    // last 4 bytes are padding and should be ignored
}

impl DailyData {
    pub fn new(date: u32, open: f64, high: f64, low: f64, close: f64) -> Self {
        Self {
            date,
            open,
            high,
            low,
            close,
        }
    }
}