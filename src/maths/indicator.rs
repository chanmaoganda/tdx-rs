use crate::{CombinedIndicator, DailyData, ShortIndicator};

use ta::indicators::MovingAverageConvergenceDivergence as MACD;
use ta::Next;

const INDICATOR_PARAMS: [usize; 3] = [12, 26, 9];
const DOUBLE_INDICATOR_PARAMS: [usize; 3] = [24, 52, 18];

pub fn short_macd(data: &[DailyData]) -> Vec<ShortIndicator> {
    let mut indicators = Vec::new();
    let (fast, slow, signal) = INDICATOR_PARAMS.into();
    let mut macd = MACD::new(fast, slow, signal).unwrap();

    let date_and_close = data.iter().map(|data| (data.date, data.close));
    for (date, close) in date_and_close {
        let output = macd.next(close);
        let indicator = (date, output).into();
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

    let date_and_close = data.iter().map(|data| (data.date, data.close));
    for (date, close) in date_and_close {
        let combined_indicator = (date, short_macd.next(close), long_macd.next(close)).into();
        indicators.push(combined_indicator);
    }
    indicators
}

#[cfg(test)]
mod tests {
    use crate::DayLineBuilder;

    use super::*;
    #[test]
    fn short_macd_test() {
        let file = "../shlday/sh600000.day";
        const QUERY_DAYS: u64 = 300;

        let day_line: Vec<DailyData> = DayLineBuilder::from_path(file)
            .unwrap()
            .query_days(QUERY_DAYS)
            .build()
            .into();
        let macd = crate::short_macd(&day_line);

        let display_data = day_line
            .iter()
            .zip(macd)
            .collect::<Vec<(&DailyData, ShortIndicator)>>();
        let selected = display_data
            .iter()
            .skip(280)
            .collect::<Vec<&(&DailyData, ShortIndicator)>>();
        dbg!(selected);
    }

    #[test]
    fn combined_macd_test() {
        let file = "../data/shlday/sh600000.day";
        const QUERY_DAYS: u64 = 300;

        let day_line: Vec<DailyData> = DayLineBuilder::from_path(file)
            .unwrap()
            .query_days(QUERY_DAYS)
            .build()
            .into();
        let macd = crate::combined_macd(&day_line);
        dbg!(macd.len());

        let display_data = day_line
            .iter()
            .zip(macd)
            .collect::<Vec<(&DailyData, CombinedIndicator)>>();
        let selected = display_data
            .iter()
            .skip(180)
            .collect::<Vec<&(&DailyData, CombinedIndicator)>>();
        dbg!(selected);
    }
}
