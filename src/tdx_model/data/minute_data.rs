use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteData {
    pub date: u16,
    pub minute: u16,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    // last 4 bytes are padding and should be ignored
}

impl MinuteData {
    pub fn new(date: u16, minute: u16, open: f64, high: f64, low: f64, close: f64) -> Self {
        Self {
            date,
            minute,
            open,
            high,
            low,
            close,
        }
    }
}
