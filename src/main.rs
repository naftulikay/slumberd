use slumberd::config::CliArgs;
use slumberd::logging;

use structopt::StructOpt;

fn main() {
    // parse CLI args
    let cli = CliArgs::from_args();

    // setup logging real quick
    logging::init(&cli);

    log::info!(
        "Starting slumberd (min-sleep: {:?}, max-sleep: {:?}, random: {}).",
        cli.min_sleep(),
        cli.max_sleep(),
        cli.random,
    );
}
