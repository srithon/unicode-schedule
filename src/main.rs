use chrono::prelude::*;
use Weekday::*;

use show_schedule;

fn main() {
    let local = Local::now();
    let day = local.weekday();

    let day_string = local.format("%A");

    use show_schedule::schedules::*;
    let schedule_vector = match day {
        Mon | Tue => [FULL_SCHEDULE_BASE, AFTERNOON_SCHEDULE_C].concat(),
        Wed => [HALF_DAY_SCHEDULE].concat(),
        Thu | Fri => [FULL_SCHEDULE_BASE, AFTERNOON_SCHEDULE_D].concat(),
        _ => {
            println!("You don't have school today! Relax!");
            return;
        }
    };

    println!("{}\n{:#?}", day_string, schedule_vector)
}
