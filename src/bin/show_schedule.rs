use chrono::prelude::*;
use Weekday::*;

use unicode_prettytable::*;

use unicode_schedule;

fn main() {
    let local = Local::now();
    let day = local.weekday();

    use unicode_schedule::schedules::*;
    let schedule = {
        let base_blocks = match day {
            Mon | Tue => [FULL_SCHEDULE_BASE, AFTERNOON_SCHEDULE_C].concat(),
            Wed => [HALF_DAY_SCHEDULE].concat(),
            Thu | Fri => [FULL_SCHEDULE_BASE, AFTERNOON_SCHEDULE_D].concat(),
            _ => {
                println!("You don't have school today! Relax!");
                return;
            }
        };

        let formatted_blocks = {
            let blocks_iterator = base_blocks.into_iter().map(|row| row.to_vec()).collect::<Vec<_>>();

            vec![vec!["Name", "Time"]].into_iter().chain(blocks_iterator).collect::<Vec<_>>()
        };

        formatted_blocks
    };

    let table = unicode_prettytable::TableBuilder::default()
        .header(
            HeaderBuilder::default()
                .double_bar(true)
                .centered_text(true)
                .build()
                .unwrap()
        )
        .rows(&schedule)
        .build()
        .unwrap();

    println!("{}", table)
}
