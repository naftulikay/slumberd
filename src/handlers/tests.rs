use super::extract;
use super::SleepQueryParams;
use super::MINIMUM_SLEEP_TIME_HEADER;

use crate::config::CliArgs;
use actix_web::http::{HeaderMap, HeaderName, HeaderValue};
use std::time::Duration;

/// Establish that it's not possible to pass minimum sleep durations that are out of bounds.
#[test]
fn test_min_safety() {
    unimplemented!();
}

/// Establish that it's not possible to pass maximum sleep durations that are out of bounds.
#[test]
fn test_max_safety() {
    unimplemented!();
}

/// Establish that it's not possible to pass sleep durations that are out of bounds.
#[test]
fn test_time_safety() {
    unimplemented!();
}

/// Test that extraction of arbitrary durations from query parameters and headers operate in the correct priority.
#[test]
fn test_extract() {
    let mut query = SleepQueryParams {
        min: Option::None,
        max: Option::None,
        duration: Option::None,
    };

    let mut headers = HeaderMap::new();

    let args = CliArgs {
        min_sleep_ms: 1000,
        max_sleep_ms: 3000,
        sleep_ms: 2000,
        verbosity: 0,
        host: "".to_string(),
        json: false,
        port: 8080,
        random: false,
    };

    // test fallback to cli args
    assert_eq!(
        Duration::from_millis(1000),
        extract(
            &headers,
            MINIMUM_SLEEP_TIME_HEADER,
            query.min,
            args.min_sleep_ms
        )
    );

    // test fallback to header
    headers.insert(
        HeaderName::from_bytes(MINIMUM_SLEEP_TIME_HEADER.to_lowercase().as_bytes()).unwrap(),
        HeaderValue::from_static("2000"),
    );

    assert_eq!(
        Duration::from_millis(2000),
        extract(
            &headers,
            MINIMUM_SLEEP_TIME_HEADER,
            query.min,
            args.min_sleep_ms
        )
    );

    // test fallback to query
    query.min = Some(3000);

    assert_eq!(
        Duration::from_millis(3000),
        extract(
            &headers,
            MINIMUM_SLEEP_TIME_HEADER,
            query.min,
            args.min_sleep_ms
        )
    );
}
