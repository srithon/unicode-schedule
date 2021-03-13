use chrono::prelude::*;
use std::fmt;

pub mod schedules;

pub enum Order {
    /// Block already finished
    Finished,
    /// Block is in progress
    InProgress,
    /// Block has not begun yet
    NotStarted
}

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

    pub fn check_order(&self, time: &NaiveTime) -> Order {
        let duration = self.end_time.signed_duration_since(*time);

        if duration > self.length {
            // duration extends past the beginning of the block
            // block hasn't started yet
            Order::NotStarted
        } else if duration >= chrono::Duration::zero() {
            // duration between 0 and block length
            // block is in progress
            Order::InProgress
        } else {
            // the end time of the block has passed
            // the block has finished
            Order::Finished
        }
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
