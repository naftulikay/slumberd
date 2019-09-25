use actix_web::{web, App, HttpServer};

use slumberd::config::CliArgs;
use slumberd::handlers;
use slumberd::logging;

use structopt::StructOpt;

fn main() {
    // parse CLI args
    let cli = CliArgs::from_args();

    // setup logging real quick
    logging::init(&cli);

    // log a warning if bounds are violated
    if cli.min_sleep_ms > cli.max_sleep_ms {
        log::warn!(
            "Minimum sleep time ({}ms) is greater than maximum sleep time ({}ms), normalizing to {:?}.",
            cli.min_sleep_ms,
            cli.max_sleep_ms,
            cli.min_sleep(),
        );
    }

    if cli.sleep_ms < cli.min_sleep_ms || cli.sleep_ms > cli.max_sleep_ms {
        log::warn!(
            "Sleep time ({}ms) is outside of minimum/maximum range ({:?}-{:?}), normalizing to {:?}.",
            cli.sleep_ms,
            cli.min_sleep(),
            cli.max_sleep(),
            cli.sleep()
        );
    }

    let bind_addr = format!("{}:{}", cli.host, cli.port);

    log::info!(
        "Starting slumberd (min sleep time: {:?}, default sleep time: {:?}, max sleep time: {:?}, random: {}).",
        cli.min_sleep(),
        cli.sleep(),
        cli.max_sleep(),
        cli.random,
    );

    // let's rock and fucking roll
    log::info!("Listening on {}.", bind_addr);

    let state = web::Data::new(cli);

    let _s = HttpServer::new(move || {
        App::new()
            .register_data(state.clone())
            .route(
                "/random/{min}/{max}",
                web::to_async(handlers::path::random_range),
            )
            .route(
                "/random/{min}/{max}/",
                web::to_async(handlers::path::random_range),
            )
            .route("/random", web::to_async(handlers::path::random))
            .route("/random/", web::to_async(handlers::path::random))
            .route("/sleep/{millis}", web::to_async(handlers::path::specific))
            .route("/sleep/{millis}/", web::to_async(handlers::path::specific))
            .default_service(web::route().to_async(handlers::default))
    })
    .bind(bind_addr)
    .unwrap()
    .run()
    .unwrap();
}
