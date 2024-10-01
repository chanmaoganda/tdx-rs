# Core library for dumping TDX .day files, with optional feature to calculate dif, dea, macd and so on.

## This is a Rust library for dumping TDX .day files. It can be used to extract data from the files and calculate technical indicators such as DIF, DEA, MACD, etc.

## Usage:

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
let path = format!("{}/*.day", path);
for entry in glob().expect("Failed to read glob pattern") {
    let path = entry?.as_os_str(&path).to_str().unwrap().to_owned();
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

let macd: Vec<ShortIndicator> = tdx_rs::short_macd(&day_line);
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