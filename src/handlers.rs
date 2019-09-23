mod response;

use actix_web::web::Data;
use actix_web::web::Path;
use actix_web::{Error, HttpResponse};

use crate::config::CliArgs;

use futures::{future, Future};

use rand::thread_rng;

use self::response::SleepResponse;

use serde::Serialize;

use std::time::Duration;

use tokio::prelude::FutureExt;

use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SleepKind {
    Fixed,
    Random,
}

type Response = Box<dyn Future<Item = HttpResponse, Error = Error>>;

pub fn default(data: Data<CliArgs>) -> Response {
    slumber(
        SleepKind::Fixed,
        data.sleep(),
        data.min_sleep(),
        data.max_sleep(),
    )
}

pub mod path {
    use super::*;
    use rand::Rng;

    pub fn specific(data: Data<CliArgs>, millis: Path<u64>) -> Response {
        slumber(
            SleepKind::Fixed,
            Duration::from_millis(*millis),
            data.min_sleep(),
            data.max_sleep(),
        )
    }

    pub fn random(data: Data<CliArgs>) -> Response {
        slumber(
            SleepKind::Random,
            thread_rng().gen_range(data.min_sleep(), data.max_sleep()),
            data.min_sleep(),
            data.max_sleep(),
        )
    }
}

fn slumber(
    kind: SleepKind,
    duration: Duration,
    _min: Duration,
    _max: Duration,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let req_id = Uuid::new_v4();

    log::debug!("{{request_id = {}}} Sleeping for {:?}.", req_id, duration);

    Box::new(
        future::empty::<(), ()>()
            .timeout(duration.clone())
            .then(move |_r| {
                log::debug!("{{request_id = {}}} Sending response.", req_id);

                let resp = SleepResponse::new(&req_id, &duration, SleepKind::Fixed);

                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .header("X-Request-Id", req_id.to_string())
                    .header("X-Sleep-Duration", resp.duration.pretty.as_str())
                    .header(
                        "X-Sleep-Duration-Millis",
                        format!("{}", resp.duration.millis),
                    )
                    .body(serde_json::to_string_pretty(&resp)?))
            }),
    )
}
