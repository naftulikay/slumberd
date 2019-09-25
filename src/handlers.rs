#[cfg(test)]
mod tests;

mod response;

use actix_web::web::Path;
use actix_web::web::{Data, Query};
use actix_web::{Error, HttpRequest, HttpResponse};

use crate::config::CliArgs;

use futures::{future, Future};

use rand::{thread_rng, Rng};

use self::response::SleepResponse;

use serde::Deserialize;
use serde::Serialize;

use std::cmp::Ord;
use std::time::Duration;

use tokio::prelude::FutureExt;

use actix_web::http::HeaderMap;
use uuid::Uuid;

static MINIMUM_SLEEP_TIME_HEADER: &'static str = "X-Slumber-Min-Time";

static MINIMUM_SLEEP_TIME_MS_HEADER: &'static str = "X-Slumber-Min-Time-Millis";

static MAXIMUM_SLEEP_TIME_HEADER: &'static str = "X-Slumber-Max-Time";

static MAXIMUM_SLEEP_TIME_MS_HEADER: &'static str = "X-Slumber-Max-Time-Millis";

static REQUEST_ID_HEADER: &'static str = "X-Request-Id";

static SLEEP_TIME_HEADER: &'static str = "X-Slumber-Time";

static SLEEP_TIME_MS_HEADER: &'static str = "X-Slumber-Time-Millis";

static SLEEP_KIND_HEADER: &'static str = "X-Slumber-Type";

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

struct SlumberConfig {
    id: Uuid,
    kind: SleepKind,
    min: Duration,
    max: Duration,
    duration: Duration,
}

impl SlumberConfig {
    /// Generate a fixed-time slumber.
    fn fixed(req: &Duration, config: &CliArgs) -> Self {
        let (min, max) = (config.min_sleep(), config.max_sleep());

        Self {
            id: Uuid::new_v4(),
            kind: SleepKind::Fixed,
            min,
            max,
            duration: SleepBounds::duration(req, &min, &max),
        }
    }

    /// Generate a random slumber using the bounds specified.
    fn random(req_min: &Duration, req_max: &Duration, config: &CliArgs) -> Self {
        // avoid multiple allocations
        let (cfg_min, cfg_max) = (config.min_sleep(), config.max_sleep());

        // pre-calculate these
        let (min, max) = (
            SleepBounds::min(&req_min, &req_max, &cfg_min, &cfg_max),
            SleepBounds::max(&req_min, &req_max, &cfg_min, &cfg_max),
        );

        Self {
            id: Uuid::new_v4(),
            kind: SleepKind::Random,
            min,
            max,
            duration: thread_rng().gen_range(min, max),
        }
    }
}

struct SleepBounds;

impl SleepBounds {
    fn duration(req: &Duration, min: &Duration, max: &Duration) -> Duration {
        // enforce the duration being >= the minimum and <= the maximum
        req.max(min).min(max).clone()
    }

    fn max(
        req_min: &Duration,
        req_max: &Duration,
        config_min: &Duration,
        config_max: &Duration,
    ) -> Duration {
        // set the lower bound to the highest lower constraint
        let lower = req_min.max(config_min);
        // set the upper bound to the lowest upper constraint
        let upper = req_max.min(config_max);

        // choose the largest bound between the lower and upper bounds
        lower.max(upper).clone()
    }

    fn min(
        req_min: &Duration,
        req_max: &Duration,
        config_min: &Duration,
        config_max: &Duration,
    ) -> Duration {
        // set the lower bound to the highest lower constraint
        let lower = req_min.max(config_min);
        // set the upper bound to the lowest upper constraint
        let upper = req_max.min(config_max);

        // choose the smallest bound between the lower and upper bounds
        lower.min(upper).clone()
    }
}

/// The response type returned by slumber requests.
type SlumberFuture = Box<dyn Future<Item = HttpResponse, Error = Error>>;

pub fn default(
    req: HttpRequest,
    data: Data<CliArgs>,
    query: Query<SleepQueryParams>,
) -> SlumberFuture {
    let (min, max) = (
        extract_min(req.headers(), &query, &data),
        extract_max(req.headers(), &query, &data),
    );

    slumber(if data.random {
        // if we're random by default, do the random
        SlumberConfig::random(&min, &max, &data)
    } else {
        // otherwise just use a fixed time
        SlumberConfig::fixed(&extract_duration(req.headers(), &query, &data), &data)
    })
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
        MINIMUM_SLEEP_TIME_MS_HEADER,
        query.min,
        config.min_sleep_ms,
    )
}

/// Extract the maximum sleep time, respecting defined bounds.
fn extract_max(headers: &HeaderMap, query: &SleepQueryParams, config: &CliArgs) -> Duration {
    extract(
        headers,
        MAXIMUM_SLEEP_TIME_MS_HEADER,
        query.max,
        config.max_sleep_ms,
    )
}

/// Extract the requested sleep duration, respecting defined bounds.
fn extract_duration(headers: &HeaderMap, query: &SleepQueryParams, config: &CliArgs) -> Duration {
    extract(
        headers,
        SLEEP_TIME_MS_HEADER,
        query.duration,
        config.sleep_ms,
    )
}

pub mod path {
    use super::*;

    /// Sleep for a specific, path-specified amount of milliseconds.
    ///
    /// The maximum value will be gated to respect the CLI-specified maximum delay value to prevent DoS-like attacks.
    pub fn specific(data: Data<CliArgs>, millis: Path<u64>) -> SlumberFuture {
        slumber(SlumberConfig::fixed(&Duration::from_millis(*millis), &data))
    }

    /// Sleep for a random amount of milliseconds within the CLI-specified minimum and maximum ranges.
    pub fn random(
        req: HttpRequest,
        data: Data<CliArgs>,
        query: Query<SleepQueryParams>,
    ) -> SlumberFuture {
        let (req_min, req_max) = (
            extract_min(req.headers(), &query, &data),
            extract_max(req.headers(), &query, &data),
        );

        slumber(SlumberConfig::random(&req_min, &req_max, &data))
    }

    /// Sleep for a random amount of milliseconds within the specified range.
    ///
    /// The maximum sleep time will be gated to the CLI-specified maximum delay value to prevent DoS-like attacks.
    pub fn random_range(data: Data<CliArgs>, range: Path<(u64, u64)>) -> SlumberFuture {
        slumber(SlumberConfig::random(
            &Duration::from_millis(range.0),
            &Duration::from_millis(range.1),
            &data,
        ))
    }
}

/// Serve a sleepy request.
fn slumber(config: SlumberConfig) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    log::debug!(
        "{{request_id = {}, kind = {:?}}} Sleeping for {:?}.",
        config.id,
        config.kind,
        config.duration,
    );

    Box::new(
        future::empty::<(), ()>()
            .timeout(config.duration.clone())
            .then(move |_r| {
                log::debug!(
                    "{{request_id = {}, kind = {:?}}} Sending response.",
                    config.id,
                    config.kind,
                );

                // generate json response
                let payload = match config.kind {
                    SleepKind::Fixed => {
                        SleepResponse::builder(&config.id, config.kind, &config.duration).build()
                    }
                    SleepKind::Random => {
                        SleepResponse::builder(&config.id, config.kind, &config.duration)
                            .min(&config.min)
                            .max(&config.max)
                            .build()
                    }
                };

                let mut response = HttpResponse::Ok();

                response
                    .content_type("application/json")
                    .header(REQUEST_ID_HEADER, config.id.to_string())
                    .header(SLEEP_TIME_HEADER, payload.duration.duration_pretty.as_str())
                    .header(
                        SLEEP_TIME_MS_HEADER,
                        format!("{}", payload.duration.duration_millis),
                    );

                match &payload.duration.kind {
                    SleepKind::Random => {
                        response.header(SLEEP_KIND_HEADER, "random");
                        response.header(MINIMUM_SLEEP_TIME_HEADER, format!("{:?}", config.min));
                        response.header(
                            MINIMUM_SLEEP_TIME_MS_HEADER,
                            format!("{}", config.min.as_millis()),
                        );
                        response.header(MAXIMUM_SLEEP_TIME_HEADER, format!("{:?}", config.max));
                        response.header(
                            MAXIMUM_SLEEP_TIME_MS_HEADER,
                            format!("{}", config.max.as_millis()),
                        );
                    }
                    SleepKind::Fixed => {
                        response.header(SLEEP_KIND_HEADER, "fixed");
                    }
                };

                Ok(response.body(serde_json::to_string_pretty(&payload)?))
            }),
    )
}
