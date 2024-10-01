use serde::{Deserialize, Serialize};
use ta::indicators::MovingAverageConvergenceDivergenceOutput as MACD;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortIndicator {
    pub dif: f64,
    pub dea: f64,
    pub macd: f64,
}

/// we prefix macd with a double value to match our need in tdx pattern
impl From<MACD> for ShortIndicator {
    fn from(value: MACD) -> Self {
        Self {
            dif: value.macd,
            dea: value.signal,
            macd: value.histogram * 2f64,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedIndicator {
    pub dif: f64,
    pub dea: f64,
    pub macd: f64,
    pub dif_2: f64,
    pub dea_2: f64,
    pub macd_2: f64,
}

/// we prefix macd with a double value to match our need in tdx pattern
impl From<(MACD, MACD)> for CombinedIndicator {
    fn from(value: (MACD, MACD)) -> Self {
        let (short, long) = value;
        Self {
            dif: short.macd,
            dea: short.signal,
            macd: short.histogram * 2f64,
            dif_2: long.macd,
            dea_2: long.signal,
            macd_2: long.histogram * 2f64,
        }
    }
}