use std::{fs::File, io::{Read, Seek, SeekFrom}, path::Path};

use byteorder::ByteOrder;

use crate::{MinuteData, MinuteLine, DAY_SIZE};

#[derive(Debug)]
pub struct MinuteLineBuilder {
    pub file: File,
    pub max_blocks: u64,
}

impl MinuteLineBuilder {
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = File::open(path.as_ref())?;
        let max_blocks = std::fs::metadata(path.as_ref())?.len() / (DAY_SIZE as u64);

        Ok(Self { file, max_blocks })
    }

    pub fn query_blocks(mut self, blocks: u64) -> Self {
        if self.max_blocks > blocks {
            // take the smaller one between the two values
            self.max_blocks = blocks;
        }

        let pos_offset = (self.max_blocks * DAY_SIZE as u64) as i64;
        self.file.seek(SeekFrom::End(0 - pos_offset)).unwrap();

        self
    }

    pub fn build(mut self) -> MinuteLine {
        let mut buffer = [0u8; DAY_SIZE];
        let mut minute_line = Vec::with_capacity(self.max_blocks as usize);

        for _ in 0..self.max_blocks {
            self.file.read_exact(buffer.as_mut()).unwrap();
            let date = byteorder::LE::read_u16(&buffer[0..2]);
            let minute = byteorder::LE::read_u16(&buffer[2..4]);
            let open = byteorder::LE::read_u32(&buffer[4..8]) as f64 / 100f64;
            let high = byteorder::LE::read_u32(&buffer[8..12]) as f64 / 100f64;
            let low = byteorder::LE::read_u32(&buffer[12..16]) as f64 / 100f64;
            let close = byteorder::LE::read_u32(&buffer[16..20]) as f64 / 100f64;
            // last 12 bytes consists of 4 bytes of turnover, 4 bytes of volume, and last 4 bytes reserved
            // those bytes are not needed now, so we drop these bytes
            let daily_data = MinuteData::new(date, minute, open, high, low, close);
            minute_line.push(daily_data);
        }

        MinuteLine::new(minute_line)
    }
}