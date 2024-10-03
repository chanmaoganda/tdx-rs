#[cfg(feature = "analysis")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Indicator {
    pub date: u32,
    pub value: f64,
}

#[cfg(feature = "analysis")]
impl From<(u32, f64)> for Indicator {
    fn from((date, value): (u32, f64)) -> Self {
        Indicator {
            date,
            value,
        }
    }
}

#[cfg(feature = "analysis")]
impl PartialOrd for Indicator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}