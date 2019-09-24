#[cfg(test)]
mod tests;

mod response;

use actix_web::web::Path;
use actix_web::web::{Data, Query};
use actix_web::{Error, HttpRequest, HttpResponse};

use crate::config::CliArgs;

use futures::{future, Future};

use rand::thread_rng;

use self::response::SleepResponse;

use serde::Deserialize;
use serde::Serialize;

use std::cmp::Ord;
use std::time::Duration;

use tokio::prelude::FutureExt;

use actix_web::http::HeaderMap;
use uuid::Uuid;

static MINIMUM_SLEEP_TIME_HEADER: &'static str = "X-Slumber-Min-Time";

static MAXIMUM_SLEEP_TIME_HEADER: &'static str = "X-Slumber-Max-Time";

static SLEEP_TIME_HEADER: &'static str = "X-Slumber-Time";

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SleepKind {
    Fixed,
    Random,
}

#[derive(Default, Deserialize)]
pub struct SleepQueryParams {
    pub min: Option<u64>,
    pub max: Option<u64>,
    #[serde(rename = "time")]
    pub duration: Option<u64>,
}

/// The response type returned by slumber requests.
type Response = Box<dyn Future<Item = HttpResponse, Error = Error>>;

pub fn default(req: HttpRequest, data: Data<CliArgs>, query: Query<SleepQueryParams>) -> Response {
    slumber(
        SleepKind::Fixed,
        extract_duration(req.headers(), &query, &data),
        extract_min(req.headers(), &query, &data),
        extract_max(req.headers(), &query, &data),
    )
}

/// Extract the sleep time using the query string, header value, or the default value in that priority.
fn extract(headers: &HeaderMap, name: &str, qs: Option<u64>, default: u64) -> Duration {
    Duration::from_millis(
        qs.unwrap_or(
            headers
                .get(name)
                .map(|h| h.to_str())
                .and_then(|r| r.ok())
                .map(|s| s.parse::<u64>())
                .and_then(|r| r.ok())
                .unwrap_or(default),
        ),
    )
}

/// Extract the minimum sleep time, respecting defined bounds.
fn extract_min(headers: &HeaderMap, query: &SleepQueryParams, config: &CliArgs) -> Duration {
    extract(
        headers,
        MINIMUM_SLEEP_TIME_HEADER,
        query.min,
        config.min_sleep_ms,
    )
}

/// Extract the maximum sleep time, respecting defined bounds.
fn extract_max(headers: &HeaderMap, query: &SleepQueryParams, config: &CliArgs) -> Duration {
    extract(
        headers,
        MAXIMUM_SLEEP_TIME_HEADER,
        query.max,
        config.max_sleep_ms,
    )
}

/// Extract the requested sleep duration, respecting defined bounds.
fn extract_duration(headers: &HeaderMap, query: &SleepQueryParams, config: &CliArgs) -> Duration {
    extract(headers, SLEEP_TIME_HEADER, query.duration, config.sleep_ms)
}

pub mod path {
    use super::*;
    use rand::Rng;

    /// Sleep for a specific, path-specified amount of milliseconds.
    ///
    /// The maximum value will be gated to respect the CLI-specified maximum delay value to prevent DoS-like attacks.
    pub fn specific(data: Data<CliArgs>, millis: Path<u64>) -> Response {
        slumber(
            SleepKind::Fixed,
            Duration::from_millis(*millis).min(data.max_sleep()),
            data.min_sleep(),
            data.max_sleep(),
        )
    }

    /// Sleep for a random amount of milliseconds within the CLI-specified minimum and maximum ranges.
    pub fn random(
        req: HttpRequest,
        data: Data<CliArgs>,
        query: Query<SleepQueryParams>,
    ) -> Response {
        let (min, max) = (
            extract_min(req.headers(), &query, &data),
            extract_max(req.headers(), &query, &data),
        );

        slumber(
            SleepKind::Random,
            thread_rng().gen_range(min, max),
            min,
            max,
        )
    }

    /// Sleep for a random amount of milliseconds within the specified range.
    ///
    /// The maximum sleep time will be gated to the CLI-specified maximum delay value to prevent DoS-like attacks.
    pub fn random_range(data: Data<CliArgs>, range: Path<(u64, u64)>) -> Response {
        // transform min and max into durations, gating the upper bound to the maximum sleep time
        let (min, max) = (
            Duration::from_millis(range.0),
            data.max_sleep().min(Duration::from_millis(range.1)),
        );

        let duration = thread_rng().gen_range(min, max);

        slumber(SleepKind::Random, duration, min, max)
    }
}

/// Serve a sleepy request.
fn slumber(
    kind: SleepKind,
    duration: Duration,
    _min: Duration,
    _max: Duration,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    let req_id = Uuid::new_v4();

    log::debug!(
        "{{request_id = {}, kind = {:?}}} Sleeping for {:?}.",
        req_id,
        kind,
        duration
    );

    Box::new(
        future::empty::<(), ()>()
            .timeout(duration.clone())
            .then(move |_r| {
                log::debug!(
                    "{{request_id = {}, kind = {:?}}} Sending response.",
                    req_id,
                    kind
                );

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
