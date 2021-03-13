use chrono::prelude::*;
use Weekday::*;

use clap::*;

use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

use unicode_schedule;

mod colors {
    use comfy_table::Color;

    pub static FINISHED_BLOCK: Color = Color::Rgb {
        r: 186,
        g: 28,
        b: 96,
    };

    pub static CURRENT_BLOCK: Color = Color::Rgb {
        r: 25,
        g: 117,
        b: 254,
    };

    pub static NEXT_BLOCK: Color = Color::Rgb {
        r: 21,
        g: 100,
        b: 217,
    };

    pub static NOT_STARTED_BLOCK: Color = Color::Rgb {
        r: 18,
        g: 85,
        b: 186,
    };
}

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

pub fn with_params<T: TimeZone>(matches: ArgMatches, local: DateTime<T>)
where
    <T as chrono::TimeZone>::Offset: std::fmt::Display,
{
    let only_remaining = matches.is_present("SHOW_BLOCKS_REMAINING");

    let day = local.weekday();

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    let mut header_table = {
        let mut header = Table::new();
        header.load_preset(UTF8_FULL);

        let day_cell = local.format("%A").to_cell().add_attribute(Attribute::Bold);

        header.add_row(vec![day_cell]);

        header
    };

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

    let block_orders = base_blocks
        .iter()
        .map(|row| {
            let times = row[1].split("-").map(str::trim).collect::<Vec<_>>();
            let start_time_string = times[0].trim_end();
            let end_time_string = times[1].trim_start();

            let start_time = parse_time(start_time_string).unwrap();
            let end_time = parse_time(end_time_string).unwrap();

            // Block struct currently isn't necessary, but may be in the future
            unicode_schedule::Block::new(start_time, end_time).check_order(&current_time)
        })
        .collect::<Vec<_>>();

    let current_block_index = block_orders
        .iter()
        .position(|order| matches!(order, unicode_schedule::Order::InProgress));

    let next_block_index = {
        if let Some(current_block_index) = current_block_index {
            block_orders
                .iter()
                .skip(current_block_index)
                .position(|order| matches!(order, unicode_schedule::Order::NotStarted))
                .map(|x| x + current_block_index)
        } else {
            None
        }
    };

    if only_remaining && current_block_index.is_none() {
        println!("School is over!");
        return;
    }

    // add passed blocks
    // if you want to print everything, print the ones leading up to the current block
    if let Some(current_block_index) = current_block_index {
        // add passed cells
        if !only_remaining {
            for row in &base_blocks[0..current_block_index] {
                table.add_row(row.into_iter().map(|text| {
                    Cell::new(text)
                        .add_attribute(Attribute::Dim)
                        .bg(colors::FINISHED_BLOCK)
                }));
            }
        }

        // add current row
        let current_row = &base_blocks[current_block_index];
        table.add_row(current_row.into_iter().map(|text| {
            Cell::new(text)
                .add_attribute(Attribute::Bold)
                .fg(colors::CURRENT_BLOCK)
        }));

        if let Some(next_block_index) = next_block_index {
            // add next block with styling
            let next_row = Row::from(base_blocks[next_block_index].into_iter().map(|field| {
                let cell = Cell::new(field).fg(colors::NEXT_BLOCK);

                cell
            }));

            table.add_row(next_row);

            // add rest of cells
            for row in &base_blocks[next_block_index + 1..base_blocks.len()] {
                table.add_row(Row::from(
                    row.into_iter()
                        .map(|field| Cell::new(field).fg(colors::NOT_STARTED_BLOCK)),
                ));
            }
        }
    } else {
        // add all cells
        for row in &base_blocks[..] {
            table.add_row(Row::from(
                row.into_iter()
                    .map(|field| Cell::new(field).fg(colors::NEXT_BLOCK)),
            ));
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
    let header_string = header_table
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

fn main() {
    let cli = create_cli();
    let matches = cli.get_matches();

    let local = Local::now();

    with_params(matches, local)
}
