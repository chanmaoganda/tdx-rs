use crate::{DailyData, ShortIndicator, CombinedIndicator};

use ta::indicators::MovingAverageConvergenceDivergence as MACD;
use ta::Next;

const INDICATOR_PARAMS: [usize; 3] = [12, 26, 9];
const DOUBLE_INDICATOR_PARAMS: [usize; 3] = [24, 52, 18];

pub fn short_macd(data: &[DailyData]) -> Vec<ShortIndicator> {
    let mut indicators = Vec::new();
    let (fast, slow, signal) = INDICATOR_PARAMS.into();
    let mut macd = MACD::new(fast, slow, signal).unwrap();
    let close_data = data.iter().map(|data| data.close);
    for close in close_data {
        let indicator = macd.next(close).into();
        indicators.push(indicator);
    }
    indicators
}

pub fn combined_macd(data: &[DailyData]) -> Vec<CombinedIndicator> {
    let mut indicators = Vec::new();

    let (fast, slow, signal) = INDICATOR_PARAMS.into();
    let mut short_macd = MACD::new(fast, slow, signal).unwrap();

    let (fast, slow, signal) = DOUBLE_INDICATOR_PARAMS.into();
    let mut long_macd = MACD::new(fast, slow, signal).unwrap();

    let close_data = data.iter().map(|data| data.close);
    for close in close_data {
        let combined_indicator = (short_macd.next(close), long_macd.next(close)).into();
        indicators.push(combined_indicator);
    }
    indicators
}