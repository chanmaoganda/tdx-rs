use crate::tdx_model::{DayLine, FullDailyData};
use crate::{CombinedIndicatorDayLine, SingleIndicatorDayLine};

use ta::indicators::MovingAverageConvergenceDivergence as MACD;
use ta::Next;

const INDICATOR_PARAMS: [usize; 3] = [12, 26, 9];
const DOUBLE_INDICATOR_PARAMS: [usize; 3] = [24, 52, 18];

pub fn short_macd(line: &DayLine) -> SingleIndicatorDayLine {
    let mut indicators = Vec::new();
    let (fast, slow, signal) = INDICATOR_PARAMS.into();
    let mut macd = MACD::new(fast, slow, signal).unwrap();

    let date_and_close = line.inner_ref().iter().map(|data| (data.date, data.close));
    for (date, close) in date_and_close {
        let output = macd.next(close);
        let indicator = (date, output).into();
        indicators.push(indicator);
    }
    SingleIndicatorDayLine::new(indicators)
}

pub fn combined_macd(line: &DayLine) -> CombinedIndicatorDayLine {
    let mut indicators = Vec::new();

    let (fast, slow, signal) = INDICATOR_PARAMS.into();
    let mut short_macd = MACD::new(fast, slow, signal).unwrap();

    let (fast, slow, signal) = DOUBLE_INDICATOR_PARAMS.into();
    let mut long_macd = MACD::new(fast, slow, signal).unwrap();

    let date_and_close = line.inner_ref().iter().map(|data| (data.date, data.close));
    for (date, close) in date_and_close {
        let combined_indicator = (date, short_macd.next(close), long_macd.next(close)).into();
        indicators.push(combined_indicator);
    }
    CombinedIndicatorDayLine::new(indicators)
}

pub fn full_data(line: DayLine) -> Vec<FullDailyData> {
    let combined_indicators = combined_macd(&line);
    combined_indicators
        .inner()
        .into_iter()
        .zip(line.inner())
        .map(|(indicator, daily_data)| (daily_data, indicator).into())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{tdx_model::DayLineBuilder, CombinedIndicator, DailyData, SingleIndicator};

    #[test]
    fn short_macd_test() {
        let file = "../shlday/sh600000.day";
        const QUERY_DAYS: u64 = 300;

        let day_line = DayLineBuilder::from_path(file)
            .unwrap()
            .query_blocks(QUERY_DAYS)
            .build();
        let macd = crate::short_macd(&day_line);

        let display_data = day_line
            .inner_ref()
            .iter()
            .zip(macd.inner())
            .collect::<Vec<(&DailyData, SingleIndicator)>>();

        let selected = display_data
            .iter()
            .skip(280)
            .collect::<Vec<&(&DailyData, SingleIndicator)>>();
        dbg!(selected);
    }

    #[test]
    fn combined_macd_test() {
        let file = "../data/shlday/sh600000.day";
        const QUERY_DAYS: u64 = 300;

        let day_line = DayLineBuilder::from_path(file)
            .unwrap()
            .query_blocks(QUERY_DAYS)
            .build();
        let macd = crate::combined_macd(&day_line);
        dbg!(macd.inner_ref().len());

        let display_data = day_line
            .inner_ref()
            .iter()
            .zip(macd.inner())
            .collect::<Vec<(&DailyData, CombinedIndicator)>>();
        let selected = display_data
            .iter()
            .skip(180)
            .collect::<Vec<&(&DailyData, CombinedIndicator)>>();
        dbg!(selected);
    }
}
