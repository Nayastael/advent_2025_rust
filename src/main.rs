mod day;
mod errors;
use crate::errors::AdventError;
use clap::Parser;
mod util;
use std::time::Instant;

macro_rules! handle_days {
    ($day:expr, $( $func:expr),* ) => {
        {
            println!("Day {}", $day);
            $(
                $func()?;
            )*
        }
};
}

// Macro for days 7-16 with consistent solve() pattern
/*macro_rules! handle_solve_days {
    ($($day:expr),*) => {
            $(
                day::$day::solve;
            )*
        }
}
        */

fn main() -> Result<(), AdventError> {
    let args = util::api::Args::parse();
    let start = Instant::now();

    match args.day {
        1 => day::day_1::solve()?,
        2 => day::day_2::solve()?,
        3 => day::day_3::solve()?,
        _ => println!("Not implemented yet"),
    }

    if args.time {
        println!("Execution time: {:?}", start.elapsed());
    }

    Ok(())
}
