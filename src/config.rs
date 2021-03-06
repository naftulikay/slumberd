use std::default::Default;
use std::time::Duration;

use structopt::StructOpt;

/// An HTTP server which sleeps for a specific or random amount of time.
///
/// Usage information is available over HTTP at /_help or /_usage; use --disable-help to disable this endpoint.
#[derive(StructOpt)]
#[structopt(name = "slumberd")]
pub struct CliArgs {
    /// The amount of time to sleep in milliseconds on each request by default. This value is ignored in random mode.
    #[structopt(short = "s", long = "sleep", default_value = "5000")]
    pub sleep_ms: u64,
    /// The host to listen on for HTTP requests.
    #[structopt(short = "H", long = "host", default_value = "127.0.0.1")]
    pub host: String,
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
    /// The port to listen for connections on.
    #[structopt(short = "P", long = "port", default_value = "8080")]
    pub port: u64,
    /// Instead of sleeping for the default sleep time, sleep for a random duration for each request by default.
    /// This random duration will be selected between the minimum and maximum sleep times.
    #[structopt(short = "r", long = "random")]
    pub random: bool,
    /// Logging verbosity. By default, only INFO and above are logged. Pass once to increase
    /// verbosity to DEBUG, twice for TRACE.
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbosity: u64,
    /// Disable serving usage information at /_help and /_usage. These endpoints will otherwise serve markdown usage
    /// information from USAGE.md which is compiled into in the binary.
    #[structopt(long = "disable-help")]
    pub disable_help: bool,
}

impl CliArgs {
    /// The default sleep duration.
    pub fn sleep(&self) -> Duration {
        let (min, max) = (self.min_sleep(), self.max_sleep());

        Duration::from_millis(self.sleep_ms).min(max).max(min)
    }

    /// The minimum allowed sleep duration.
    pub fn min_sleep(&self) -> Duration {
        // prevent footshot: minimum must always be less than or equal to maximum, this will prevent user error on
        // the command-line
        let (min, max) = (
            Duration::from_millis(self.min_sleep_ms),
            Duration::from_millis(self.max_sleep_ms),
        );

        min.min(max)
    }

    /// The maximum allowed sleep duration.
    pub fn max_sleep(&self) -> Duration {
        // prevent footshot: maximum must always be greater than or equal to minimum, this will prevent user error on
        // the command-line
        let (min, max) = (
            Duration::from_millis(self.min_sleep_ms),
            Duration::from_millis(self.max_sleep_ms),
        );

        max.max(min)
    }
}

impl Default for CliArgs {
    fn default() -> Self {
        // FIXME this is duplicated code due to default structopt values listed above, not sure how to combat this
        Self {
            sleep_ms: 5000,
            host: "127.0.0.1".to_string(),
            json: false,
            min_sleep_ms: 15,
            max_sleep_ms: 30000,
            port: 8080,
            verbosity: 0,
            disable_help: false,
            random: false,
        }
    }
}
