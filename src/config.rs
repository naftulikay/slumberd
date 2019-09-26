use std::time::Duration;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "slumberd",
    about = "An HTTP service for slowly serving HTTP responses."
)]
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
    /// Enable TLS support via rustls. A TLS certificate and private key must be passed if TLS is enabled.
    #[structopt(long = "tls")]
    pub tls: bool,
    /// A path to a TLS certificate chain to use in TLS mode. The certificate chain should be in descending order,
    /// such that the root CA is the very last certificate in the file and the server certificate is the very first
    /// certificate in the file.
    #[structopt(long = "certificate")]
    pub certificate: Option<PathBuf>,
    /// A path to a private key to use in TLS mode.
    #[structopt(long = "private-key")]
    pub private_key: Option<PathBuf>,
    /// Logging verbosity. By default, only INFO and above are logged. Pass once to increase
    /// verbosity to DEBUG, twice for TRACE.
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbosity: u64,
}

impl Default for CliArgs {
    fn default() -> Self {
        Self {
            sleep_ms: 5000,
            host: "127.0.0.1".to_string(),
            port: 8080,
            json: false,
            min_sleep_ms: 15,
            max_sleep_ms: 30000,
            random: false,
            tls: false,
            certificate: None,
            private_key: None,
            verbosity: 0,
        }
    }
}

impl CliArgs {
    /// Get the listen address.
    pub fn listen_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

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
