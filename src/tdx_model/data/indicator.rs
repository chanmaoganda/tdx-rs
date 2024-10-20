#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Indicator {
    pub date: u32,
    pub value: f64,
}

impl Indicator {
    pub fn new(date: u32, value: f64) -> Self {
        Self { date, value }
    }
}

impl From<Indicator> for (u32, f64) {
    fn from(value: Indicator) -> Self {
        (value.date, value.value)
    }
}

impl PartialOrd for Indicator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
