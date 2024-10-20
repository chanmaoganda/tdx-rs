use std::{fs::File, path::Path};

use crate::{CombinedIndicatorDayLine, FullDailyData, SingleIndicatorDayLine, DAY_SIZE};

use super::DayLineBuilder;

#[derive(Debug)]
pub struct IndicatorBuilder {
    day_line_builder: DayLineBuilder,
}

impl IndicatorBuilder {
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = File::open(path.as_ref())?;
        let max_days = std::fs::metadata(path.as_ref())?.len() / (DAY_SIZE as u64);
        let day_line_builder = DayLineBuilder { file, max_blocks: max_days };
        Ok(Self { day_line_builder })
    }

    pub fn query_days(self, days: u64) -> Self {
        let inner_builder = self.day_line_builder.query_blocks(days);
        Self {
            day_line_builder: inner_builder,
        }
    }

    pub fn build_short_indicator(self) -> SingleIndicatorDayLine {
        let day_line = self.day_line_builder.build();
        crate::short_macd(&day_line)
    }

    pub fn build_combined_indicator(self) -> CombinedIndicatorDayLine {
        let day_line = self.day_line_builder.build();
        crate::combined_macd(&day_line)
    }

    pub fn build_full_data(self) -> Vec<FullDailyData> {
        let day_line = self.day_line_builder.build();
        crate::full_data(day_line)
    }
}

#[cfg(test)]
mod indicator_tests {
    use super::*;

    #[test]
    fn test_indicator_builder() -> anyhow::Result<()> {
        let builder = IndicatorBuilder::from_path("../data/shlday/sh600000.day")?.query_days(400);

        let short_indicator_line = builder.build_short_indicator();
        dbg!(short_indicator_line.inner().len());
        Ok(())
    }

    #[test]
    fn test_combined_indicator() -> anyhow::Result<()> {
        let builder = IndicatorBuilder::from_path("../data/shlday/sh600000.day")?.query_days(400);

        let combined_indicator_line = builder.build_combined_indicator();
        dbg!(combined_indicator_line.inner().len());
        Ok(())
    }

    #[test]
    fn test_full_data() -> anyhow::Result<()> {
        let builder = IndicatorBuilder::from_path("../data/shlday/sh600000.day")?.query_days(400);

        let full_data = builder.build_full_data();
        dbg!(full_data.len());
        Ok(())
    }
}
