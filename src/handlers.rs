use actix_web::web::Data;
use actix_web::{Error, HttpResponse};

use futures::{future, Future};

use std::time::Duration;

use crate::config::CliArgs;
use tokio::prelude::FutureExt;

pub fn handler(data: Data<CliArgs>) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    log::debug!("Sleeping for {:?}...", data.sleep());
    sleeper(data.sleep())
}

fn sleeper(duration: Duration) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    Box::new(
        future::empty::<(), ()>()
            .timeout(duration)
            .then(|_r| Ok(HttpResponse::Ok().finish())),
    )
}
