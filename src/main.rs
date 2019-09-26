use actix_web::{web, App, HttpServer};

use slumberd::config::CliArgs;
use slumberd::handlers;
use slumberd::logging;

use std::process;

use rustls::ServerConfig;
use structopt::StructOpt;

fn main() {
    // parse CLI args
    let cli = CliArgs::from_args();

    // setup logging real quick
    logging::init(&cli);

    // emit warnings if bounds are violated
    lint_args(&cli);

    // generate a server config if tls is enabled
    let tls_config = gen_rustls_config(&cli);

    log::info!(
        "Starting slumberd (min sleep time: {:?}, default sleep time: {:?}, max sleep time: {:?}, random: {}).",
        cli.min_sleep(),
        cli.sleep(),
        cli.max_sleep(),
        cli.random,
    );

    let listen_addr = cli.listen_addr();

    // let's rock and fucking roll
    log::info!(
        "Listening for traffic at {}://{}.",
        if cli.tls { "https" } else { "http" },
        listen_addr,
    );

    let state = web::Data::new(cli);

    let server = HttpServer::new(move || {
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
    });

    if tls_config.is_some() {

    } else {
        server.bind(listen_addr).unwrap().run().unwrap();
    }
}

fn lint_args(args: &CliArgs) {
    // log a warning if bounds are violated
    if args.min_sleep_ms > args.max_sleep_ms {
        log::warn!(
            "Minimum sleep time ({}ms) is greater than maximum sleep time ({}ms), normalizing to {:?}.",
            args.min_sleep_ms,
            args.max_sleep_ms,
            args.min_sleep(),
        );
    }

    if args.sleep_ms < args.min_sleep_ms || args.sleep_ms > args.max_sleep_ms {
        log::warn!(
            "Sleep time ({}ms) is outside of minimum/maximum range ({:?}-{:?}), normalizing to {:?}.",
            args.sleep_ms,
            args.min_sleep(),
            args.max_sleep(),
            args.sleep()
        );
    }
}

fn gen_rustls_config(cli: &CliArgs) -> Option<ServerConfig> {
    if !cli.tls {
        return None;
    }

    let (_cert, _key) = (cli.certificate.as_ref().unwrap_or_else(|| {
        log::error!("If TLS is enabled, you must pass the path to a certificate via the --certificate flag.");
        process::exit(1)
    }), cli.private_key.as_ref().unwrap_or_else(|| {
        log::error!("If TLS is enabled, you must pass the path to a private key via the --private-key flag.");
        process::exit(1)
    }));

    None
}
