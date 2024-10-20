mod day_line;
mod indicated_day_line;

use std::ops::Range;

pub use day_line::*;
pub use indicated_day_line::*;

pub trait QueryLine {
    type Output;

    fn query_data(&self, range: Range<usize>) -> &[Self::Output];
}