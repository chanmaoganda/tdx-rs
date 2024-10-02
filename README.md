# Core library for dumping TDX .day files, with optional feature to calculate dif, dea, macd and so on.

## This is a Rust library for dumping TDX .day files. It can be used to extract data from the files and calculate technical indicators such as DIF, DEA, MACD, etc.

## Usage:

#### Extracting data to csv files
```rust
use glob::glob;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tdx_rs::{DailyData, DayLineBuilder};

/// anyhow = "1.0.89"
/// csv = "1.3.0"
/// glob = "0.3.1"
/// rayon = "1.10.0"
/// tdx-rs = "0.1.3"

fn main() -> anyhow::Result<()> {
    let directory = std::env::args().nth(1).expect("no pattern given");
    let pattern = format!("../data/{}/*.day", directory);
    let mut files = Vec::new();
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        let path = entry?.to_str().unwrap().to_owned();
        files.push(path);
    }

    const QUERY_DAYS: usize = 300;
    const MATCHED_DAYS: usize = 200;

    files.par_iter().for_each(|file| {
        let day_line: Vec<DailyData> = DayLineBuilder::from_path(file)
            .unwrap()
            .query_days(QUERY_DAYS as u64)
            .build()
            .into();

        if day_line.len() < MATCHED_DAYS {
            return;
        }

        let combined_macds = tdx_rs::full_data(day_line);
        // silly pattern matching to avoid regex
        let code = &file[15..23];
        let indicator_name = format!("../data/{}_indicator/{}_indicator.csv", directory, code);
        let mut writer = csv::Writer::from_path(&indicator_name).unwrap();
        
        for data in combined_macds {
            writer.serialize(data).unwrap();
        }

        writer.flush().unwrap();
    });

    Ok(())
}
```

#### Extracting data from raw files
-  Extracting data from a single file:
```rust
use tdx_rs::{DailyData, DayLineBuilder};
/* 
    Assuming path = "C:/new_tdx/vipdoc/sh/lda"
*/
let file = "C:/new_tdx/vipdoc/sh/lda/sh000001.day";

let day_line: Vec<DailyData> = DayLineBuilder::from_path(file)
    .unwrap()
    .query_days(QUERY_DAYS as u64)
    .build()
    .into();
println!("{:#?}", day_line);

```

-  Extracting data from multiple files:
```rust
use tdx_rs::{DailyData, DayLineBuilder};
use glob::glob;

/* 
    Assuming path = "C:/new_tdx/vipdoc/sh/lda"
*/
let glob_path = format!("{}/*.day", path);
for entry in glob(&glob_path).expect("Failed to read glob pattern") {
    let path = entry?.to_str().unwrap().to_owned();
    files.push(path);
}

for file in files {
    let day_line: Vec<DailyData> = DayLineBuilder::from_path(file)
        .unwrap()
        .query_days(QUERY_DAYS as u64)
        .build()
        .into();

    println!("{:#?}", day_line);
}
```


#### Calculating technical indicators
- calculating short macd with 12, 26, 9:
```rust
/* 
    Assuming directory = "C:/new_tdx/vipdoc/sh/lda"
*/
let file = "C:/new_tdx/vipdoc/sh/lda/sh000001.day";
const QUERY_DAYS: u64 = 300;

let day_line: Vec<DailyData> = DayLineBuilder::from_path(file)
    .unwrap()
    .query_days(QUERY_DAYS)
    .build()
    .into();

let macd = tdx_rs::short_macd(&day_line);
println!("{:#?}", macd);
```
- calculating long macd with 24, 52, 18:
```rust
/* 
    Assuming directory = "C:/new_tdx/vipdoc/sh/lda"
*/
let file = "C:/new_tdx/vipdoc/sh/lda/sh000001.day";
const QUERY_DAYS: u64 = 300;

let day_line: Vec<DailyData> = DayLineBuilder::from_path(file)
    .unwrap()
    .query_days(QUERY_DAYS)
    .build()
    .into();

let macd = tdx_rs::combined_macd(&day_line);
println!("{:#?}", macd);
```
- calculating full macd with 12, 26, 9, 24, 52, 18, together with raw data:
⚠️ note that `tdx_rs::full_data` requires the `full ownership` of the `Vec<DailyData>`, 
```rust
/* 
    Assuming directory = "C:/new_tdx/vipdoc/sh/lda"
*/
let file = "C:/new_tdx/vipdoc/sh/lda/sh000001.day";
const QUERY_DAYS: u64 = 300;

let day_line: Vec<DailyData> = DayLineBuilder::from_path(file)
    .unwrap()
    .query_days(QUERY_DAYS)
    .build()
    .into();

/// 
let macd = tdx_rs::full_data(day_line);
println!("{:#?}", macd);
```