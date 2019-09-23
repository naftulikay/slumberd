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
            .route("/random", web::to_async(handlers::path::random))
            .route("/sleep/{millis}", web::to_async(handlers::path::specific))
            .default_service(web::route().to_async(handlers::default))
    })
    .bind(bind_addr)
    .unwrap()
    .run()
    .unwrap();
}
