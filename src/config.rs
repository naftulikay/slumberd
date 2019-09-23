use std::time::Duration;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "slumberd",
    about = "An HTTP service for slowly serving HTTP responses."
)]
pub struct CliArgs {
    /// Log in line-delimited JSON format.
    #[structopt(short = "j", long = "json")]
    pub json: bool,
    /// The minimum allowed request sleep time in milliseconds. In random mode, this will serve as
    /// the lower bound for random sleep durations.
    #[structopt(long = "min-sleep", default_value = "15")]
    pub min_sleep_ms: u64,
    /// The maximum allowed request sleep time in milliseconds. In random mode, this will serve as
    /// the upper bound for random sleep durations.
    #[structopt(long = "max-sleep", default_value = "30000")]
    pub max_sleep_ms: u64,
    /// By default, sleep for random periods of time between minimum and maximum sleep times.
    #[structopt(short = "r", long = "random")]
    pub random: bool,
    /// Logging verbosity. By default, only INFO and above are logged. Pass once to increase
    /// verbosity to DEBUG, twice for TRACE.
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbosity: u64,
}

impl CliArgs {
    /// The minimum allowed sleep duration.
    pub fn min_sleep(&self) -> Duration {
        Duration::from_millis(self.min_sleep_ms)
    }

    /// The maximum allowed sleep duration.
    pub fn max_sleep(&self) -> Duration {
        Duration::from_millis(self.max_sleep_ms)
    }
}
