use serde::{Deserialize, Serialize};
use ta::indicators::MovingAverageConvergenceDivergenceOutput as MACD;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortIndicator {
    dif: f64,
    dea: f64,
    macd: f64,
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
    dif: f64,
    dea: f64,
    macd: f64,
    dif_2: f64,
    dea_2: f64,
    macd_2: f64,
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