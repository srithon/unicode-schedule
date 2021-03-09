use chrono::prelude::*;
use std::fmt;

pub mod schedules;

pub struct Block {
    start_time: NaiveTime,
    end_time: NaiveTime,

    // caches this value rather than recalculating each time
    length: chrono::Duration,
}

impl Block {
    pub fn new(start_time: NaiveTime, end_time: NaiveTime) -> Block {
        Block {
            start_time,
            end_time,
            length: end_time.signed_duration_since(start_time),
        }
    }

    pub fn contains(&self, time: &NaiveTime) -> bool {
        let duration = self.end_time.signed_duration_since(*time);

        // not before the block started and not after the block ended
        duration < self.length && duration > chrono::Duration::zero()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} - {}",
            self.start_time.format("%H:%M"),
            self.end_time.format("%H:%M")
        )
    }
}
