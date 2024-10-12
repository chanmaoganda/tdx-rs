# Core library for dumping TDX .day files, with optional feature to calculate dif, dea, macd and so on.

## This is a Rust library for dumping TDX .day files. It can be used to extract data from the files and calculate technical indicators such as DIF, DEA, MACD, etc.

## Usage:

#### Calculating technical indicators
```rust
/* 
    Assuming directory = "C:/new_tdx/vipdoc/sh/lda"
*/
let file = "C:/new_tdx/vipdoc/sh/lda/sh600000.day";
const QUERY_DAYS: u64 = 300;

let day_line = DayLineBuilder::from_path(file)
    .unwrap()
    .query_days(QUERY_DAYS)
    .build();

/// calculating short macd with 12, 26, 9:
let short_macd = tdx_rs::short_macd(&day_line);
/// calculating long macd with 24, 52, 18:
let combined_macd = tdx_rs::combined_macd(&day_line);

/// ⚠️ note that `tdx_rs::full_data` requires the `full ownership` of the `DayLine`, 
let full_data = tdx_rs::full_data(day_line);

```

#### Use builder to fast get data from files
```rust
let builder = IndicatorBuilder::from_path("../data/shlday/sh600000.day")?.query_days(400);
let short_indicator_line = builder.build_short_indicator();
dbg!(short_indicator_line.inner().len());


let builder = IndicatorBuilder::from_path("../data/shlday/sh600000.day")?.query_days(400);
let combined_indicator_line = builder.build_combined_indicator();
dbg!(combined_indicator_line.inner().len());


let builder = IndicatorBuilder::from_path("../data/shlday/sh600000.day")?.query_days(400);
let full_data = builder.build_full_data();
dbg!(full_data.len());
```

#### Extracting data from raw files
-  Extracting data from a single file:
```rust
use tdx_rs::{DailyData, DayLineBuilder};
/* 
    Assuming path = "C:/new_tdx/vipdoc/sh/lda"
*/
let file = "C:/new_tdx/vipdoc/sh/lda/sh600000.day";

let day_line = DayLineBuilder::from_path(file)
    .unwrap()
    .query_days(QUERY_DAYS as u64)
    .build();
println!("{:#?}", day_line.inner());

```

#### Extracting data to csv files
```rust
use glob::glob;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tdx_rs::DayLineBuilder;

/// anyhow = "1.0.89"
/// csv = "1.3.0"
/// glob = "0.3.1"
/// rayon = "1.10.0"
/// tdx-rs = "0.1.3"

fn main() -> anyhow::Result<()> {
    let directory = "sh"; // `sh` or `sz` or sth
    let pattern = format!(
        "C:/Users/avania/AppData/Local/new_tdx/vipdoc/{}/lday/*.day",
        directory
    );
    let mut files = Vec::new();
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        let path = entry?.to_str().unwrap().to_owned();
        files.push(path);
    }
    let code_matcher = Regex::new(r"sh6\d{5}").unwrap();
    const QUERY_DAYS: usize = 300;
    const MATCHED_DAYS: usize = 200;

    files.par_iter().for_each(|file| {
        let day_line = DayLineBuilder::from_path(file)
            .unwrap()
            .query_days(QUERY_DAYS as u64)
            .build();

        if day_line.inner_ref().len() < MATCHED_DAYS {
            return;
        }

        let combined_macds = tdx_rs::full_data(day_line);

        let matcher = code_matcher.find(file).unwrap();
        let code = matcher.as_str();

        let indicator_name = format!("../data/{}_indicator/{}_indicator.csv", directory, code);
        // dbg!(&file);
        // dbg!(&indicator_name);
        let mut writer = csv::Writer::from_path(&indicator_name).unwrap();

        for data in combined_macds {
            writer.serialize(data).unwrap();
        }

        writer.flush().unwrap();
    });

    Ok(())
}
```

#### Spliting CombinedIndicatorDayLine into two SingleIndicatorDayLine
- suppose u have dumped the day files to csv files
```rust
let mut values = csv::Reader::from_path("../data/sh_indicator/shxxxxxx_indicator.csv")?;
let indicators: Vec<tdx_rs::CombinedIndicator> =
    values.deserialize().map(|value| value.unwrap()).collect();
let combined_day_line = tdx_rs::CombinedIndicatorDayLine::new(indicators);
let (short_day_line, long_day_line) = combined_day_line.split_single_indicator();

assert_eq!(short_day_line.inner_ref().len(), 300);
assert_eq!(long_day_line.inner_ref().len(), 300);
```
- otherwise use day files directly
```rust
let builder = IndicatorBuilder::from_path("../data/shlday/sh600000.day")?.query_days(400);
let combined_indicator_line = builder.build_combined_indicator();
let (short_day_line, long_day_line) = combined_day_line.split_single_indicator();
println!("short len: {}, long len: {}", short_day_line.inner_ref().len(), long_day_line.inner_ref().len());
```