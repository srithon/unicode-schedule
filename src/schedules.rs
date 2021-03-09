/// Common morning schedule for full-length days
pub static FULL_SCHEDULE_BASE: &[&[&'static str; 3]] = &[
    &["Block 1", "7:30", "8:36"],
    &["Block 2", "8:44", "9:44"],
    &["Block 3", "9:52", "10:52"],
    &["Block 4", "11:00", "12:00"],
    &["Lunch", "12:00", "1:00"],
];

/// Repeats blocks 1 and 2 in the afternoon
pub static AFTERNOON_SCHEDULE_C: &[&[&'static str; 3]] =
    &[&["Block 1", "1:00", "1:25"], &["Block 2", "1:30", "1:55"]];

/// Repeats blocks 3 and 4 in the afternoon
pub static AFTERNOON_SCHEDULE_D: &[&[&'static str; 3]] =
    &[&["Block 3", "1:00", "1:25"], &["Block 4", "1:30", "1:55"]];

/// No afternoon session
pub static HALF_DAY_SCHEDULE: &[&[&'static str; 3]] = &[
    &["Block 1", "7:30", "8:30"],
    &["Block 2", "8:40", "9:40"],
    &["Block 3", "9:50", "10:50"],
    &["Block 4", "11:00", "12:00"],
];
