use chrono::prelude::*;
use Weekday::*;

use clap::*;

use unicode_prettytable::*;

use unicode_schedule;

fn create_cli() -> App<'static, 'static> {
    clap_app!(show_schedule =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg SHOW_REMAINING: -r --remaining "Only show blocks remaining in the day")
    )
}

/// Takes a string representing a Time and parses it
/// The string does not contain period (AM/PM) information, so the function will determine it on
/// its own
/// Any hour before 6 will be considered a PM, while everything else will be considered normally
fn parse_time(string: &str) -> Option<NaiveTime> {
    let time = NaiveTime::parse_from_str(string, "%k:%_M").ok()?;
    let hour = time.hour();

    // if it's before 6 AM, then it must mean PM
    if hour < 6 {
        time.with_hour(hour + 12)
    } else {
        Some(time)
    }
}

fn main() {
    let cli = create_cli();
    let matches = cli.get_matches();

    let only_remaining = matches.is_present("SHOW_REMAINING");

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

        let current_time = local.time();

        let formatted_blocks = {
            let blocks_iterator = base_blocks.into_iter();

            // this is the only way to conditionally iterate over iterators
            // have all of the options there, but only have one of them yield values
            let (standard_iterator, skip_iterator) = if only_remaining {
                (
                    None,
                    Some(blocks_iterator.skip_while(|row| {
                        let times = row[1].split("-").map(str::trim).collect::<Vec<_>>();
                        let start_time_string = times[0].trim_end();
                        let end_time_string = times[1].trim_start();

                        let start_time = parse_time(start_time_string).unwrap();
                        let end_time = parse_time(end_time_string).unwrap();

                        // Block struct currently isn't necessary, but may be in the future
                        let block = unicode_schedule::Block::new(start_time, end_time);

                        !block.contains(&current_time)
                    })),
                )
            } else {
                (Some(blocks_iterator), None)
            };

            // Conditionally iterate over one of several possible iterators
            // https://stackoverflow.com/a/52064434
            vec![vec!["Name", "Time"]]
                .into_iter()
                .chain(
                    skip_iterator
                        .into_iter()
                        .flatten()
                        .chain(standard_iterator.into_iter().flatten())
                        .map(|row| row.to_vec()),
                )
                .collect::<Vec<_>>()
        };

        formatted_blocks
    };

    if only_remaining && schedule.len() == 1 {
        println!("School is over!");
        return;
    }

    let table = unicode_prettytable::TableBuilder::default()
        .header(
            HeaderBuilder::default()
                .double_bar(true)
                .centered_text(true)
                .build()
                .unwrap(),
        )
        .rows(&schedule)
        .build()
        .unwrap();

    println!("{}", table)
}
