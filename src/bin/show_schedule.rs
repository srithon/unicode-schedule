use chrono::prelude::*;
use Weekday::*;

use clap::*;

use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

use unicode_schedule;

fn create_cli() -> App<'static, 'static> {
    clap_app!(show_schedule =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg SHOW_BLOCKS_REMAINING: -b --("blocks-remaining") "Only show blocks remaining in the day")
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

    let only_remaining = matches.is_present("SHOW_BLOCKS_REMAINING");

    let local = Local::now();
    let day = local.weekday();

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    let mut header_table = Table::new();
    header_table
        .load_preset(UTF8_FULL)
        .add_row(vec![local.format("%A")]);

    use unicode_schedule::schedules::*;

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

    let current_block_index = base_blocks.iter().position(|row| {
        let times = row[1].split("-").map(str::trim).collect::<Vec<_>>();
        let start_time_string = times[0].trim_end();
        let end_time_string = times[1].trim_start();

        let start_time = parse_time(start_time_string).unwrap();
        let end_time = parse_time(end_time_string).unwrap();

        // Block struct currently isn't necessary, but may be in the future
        let block = unicode_schedule::Block::new(start_time, end_time);

        block.contains(&current_time)
    });

    if only_remaining && current_block_index.is_none() {
        println!("School is over!");
        return;
    }

    macro_rules! add_row {
        ($row:ident) => {
            table.add_row($row.to_vec());
        };
    }

    // add passed blocks
    // if you want to print everything, print the ones leading up to the current block
    if let Some(current_block_index) = current_block_index {
        // add passed cells
        if only_remaining {
            for row in &base_blocks[0..current_block_index] {
                // TODO style each row
                add_row!(row);
            }
        }

        // add current row
        let current_row = &base_blocks[current_block_index];
        table.add_row(
            current_row
                .into_iter()
                .map(|text| Cell::new(text).add_attribute(Attribute::Bold)),
        );

        // add rest of cells
        for row in &base_blocks[current_block_index + 1..base_blocks.len()] {
            add_row!(row);
        }
    } else {
        // add all cells
        for row in &base_blocks[..] {
            add_row!(row);
        }
    }

    let table_string = table.to_string();

    let header_column = header_table.get_column_mut(0).unwrap();
    header_column.set_cell_alignment(CellAlignment::Center);
    header_column.set_constraint(ColumnConstraint::Width(
        // subtract 2 because we do not count the left and right separators
        (table_string.lines().nth(0).unwrap().chars().count() - 2) as u16,
    ));

    // only take the top two lines in the header table output
    // we only want the "header" of the header
    let header_string =
        header_table
            .to_string()
            .lines()
            .take(2)
            // effectively join('\n')
            .fold(String::new(), |mut a, b| {
                a.push_str(b);
                a.push('\n');
                a
            });

    print!("{}", header_string);
    println!("{}", table_string)
}
