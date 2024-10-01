use serde::{Deserialize, Serialize};
use ta::indicators::MovingAverageConvergenceDivergenceOutput as MACD;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortIndicator {
    pub date: u32,
    pub dif: f64,
    pub dea: f64,
    pub macd: f64,
}

/// we prefix macd with a double value to match our need in tdx pattern
impl From<(u32, MACD)> for ShortIndicator {
    fn from(value: (u32, MACD)) -> Self {
        let (date, indicator) = value;
        let (dif, dea, macd) = indicator.into();
        Self {
            date,
            dif,
            dea,
            macd: macd * 2f64,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedIndicator {
    pub date: u32,
    pub dif: f64,
    pub dea: f64,
    pub macd: f64,
    pub dif_2: f64,
    pub dea_2: f64,
    pub macd_2: f64,
}

/// we prefix macd with a double value to match our need in tdx pattern
impl From<(u32, MACD, MACD)> for CombinedIndicator {
    fn from(value: (u32, MACD, MACD)) -> Self {
        let (date, short, long) = value;
        let (dif, dea, macd) = short.into();
        let (dif_2, dea_2, macd_2) = long.into();

        Self {
            date,
            dif,
            dea,
            macd: macd * 2f64,
            dif_2,
            dea_2,
            macd_2: macd_2 * 2f64,
        }
    }
}

impl From<CombinedIndicator> for (u32, f64, f64, f64, f64, f64, f64) {
    fn from(value: CombinedIndicator) -> Self {
        (
            value.date,
            value.dif,
            value.dea,
            value.macd,
            value.dif_2,
            value.dea_2,
            value.macd_2,
        )
    }
}
