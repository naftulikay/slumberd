use actix_web::web::Data;
use actix_web::{Error, HttpResponse};

use futures::{future, Future};

use std::time::Duration;

use crate::config::CliArgs;
use tokio::prelude::FutureExt;

use uuid::Uuid;

pub fn handler(data: Data<CliArgs>) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    sleeper(data.sleep())
}

fn sleeper(duration: Duration) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let req_id = Uuid::new_v4();

    log::debug!("{{request_id = {}}} Sleeping for {:?}.", req_id, duration);

    Box::new(future::empty::<(), ()>().timeout(duration).then(move |_r| {
        let pretty = format!("{:?}", duration);
        let millis = duration.as_millis();

        log::debug!("{{request_id = {}}} Sending response.", req_id);

        Ok(HttpResponse::Ok()
            .header("X-Request-Id", req_id.to_string())
            .header("X-Sleep-Duration", pretty)
            .header("X-Sleep-Duration-Millis", format!("{}", millis))
            .finish())
    }))
}
