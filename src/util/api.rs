use clap::Parser;

/// day API
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// Calculate execution time
    #[arg(short, long)]
    pub(crate) time: bool,

    /// Days to evaluate
    #[arg(short, long, default_value_t = 1)]
    pub(crate) day: u8,
}
