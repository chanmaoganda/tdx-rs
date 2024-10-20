use serde::{Deserialize, Serialize};

use crate::MinuteData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteLine {
    pub data: Vec<MinuteData>,
}

impl MinuteLine {
    pub fn new(data: Vec<MinuteData>) -> Self {
        Self { data }
    }

    pub fn inner(self) -> Vec<MinuteData> {
        self.data
    }

    pub fn inner_ref(&self) -> &[MinuteData] {
        self.data.as_ref()
    }
}
