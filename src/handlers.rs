use actix_web::web::Data;
use actix_web::{Error, HttpResponse};

use crate::config::CliArgs;

use futures::{future, Future};

use serde_json::json;

use std::convert::TryInto;
use std::time::Duration;

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

        let body = json!({
            "request-id": req_id,
            "sleep": {
                "duration": pretty,
                "duration-ms": millis,
            }
        });

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .header("X-Request-Id", req_id.to_string())
            .header("X-Sleep-Duration", pretty)
            .header("X-Sleep-Duration-Millis", format!("{}", millis))
            .body(serde_json::to_string_pretty(&body)?))
    }))
}
