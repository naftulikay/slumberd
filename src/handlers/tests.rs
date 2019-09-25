use super::extract_duration;
use super::extract_sleep_kind;
use super::extract_sleep_max_time;
use super::extract_sleep_min_time;
use super::extract_sleep_time;
use super::SleepBounds;
use super::SleepKind;
use super::SleepQueryParams;
use super::MAXIMUM_SLEEP_TIME_MS_HEADER;
use super::MINIMUM_SLEEP_TIME_MS_HEADER;
use super::SLEEP_KIND_HEADER;
use super::SLEEP_TIME_MS_HEADER;

use crate::config::CliArgs;

use actix_web::http::{HeaderMap, HeaderName, HeaderValue};
use std::time::Duration;

#[test]
fn test_extract_sleep_min_time() {
    let mut query: SleepQueryParams = Default::default();
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
        extract_sleep_min_time(&headers, &query, &args)
    );

    // test headers
    headers.insert(
        HeaderName::from_bytes(&MINIMUM_SLEEP_TIME_MS_HEADER.to_lowercase().as_bytes()).unwrap(),
        HeaderValue::from_str("1500").unwrap(),
    );

    assert_eq!(
        Duration::from_millis(1500),
        extract_sleep_min_time(&headers, &query, &args)
    );

    // test query
    query.min = Some(1750);

    assert_eq!(
        Duration::from_millis(1750),
        extract_sleep_min_time(&headers, &query, &args)
    );
}

#[test]
fn test_extract_sleep_max_time() {
    let mut query: SleepQueryParams = Default::default();
    let mut headers = HeaderMap::new();

    let args = CliArgs {
        min_sleep_ms: 2000,
        max_sleep_ms: 4000,
        sleep_ms: 3000,
        verbosity: 0,
        host: "".to_string(),
        json: false,
        port: 8080,
        random: false,
    };

    // test fallback to cli args
    assert_eq!(
        Duration::from_millis(4000),
        extract_sleep_max_time(&headers, &query, &args)
    );

    // test headers
    headers.insert(
        HeaderName::from_bytes(&MAXIMUM_SLEEP_TIME_MS_HEADER.to_lowercase().as_bytes()).unwrap(),
        HeaderValue::from_static("3500"),
    );

    assert_eq!(
        Duration::from_millis(3500),
        extract_sleep_max_time(&headers, &query, &args)
    );

    // test query
    query.max = Some(3000);

    assert_eq!(
        Duration::from_millis(3000),
        extract_sleep_max_time(&headers, &query, &args)
    );
}

#[test]
fn test_extract_sleep_time() {
    let mut query: SleepQueryParams = Default::default();
    let mut headers = HeaderMap::new();

    let args = CliArgs {
        min_sleep_ms: 2000,
        max_sleep_ms: 5000,
        sleep_ms: 3500,
        verbosity: 0,
        host: "".to_string(),
        json: false,
        port: 8080,
        random: false,
    };

    // test fallback to cli args
    assert_eq!(
        Duration::from_millis(3500),
        extract_sleep_time(&headers, &query, &args)
    );

    // test headers
    headers.insert(
        HeaderName::from_bytes(&SLEEP_TIME_MS_HEADER.to_lowercase().as_bytes()).unwrap(),
        HeaderValue::from_static("3000"),
    );

    assert_eq!(
        Duration::from_millis(3000),
        extract_sleep_time(&headers, &query, &args)
    );

    // test query
    query.duration = Some(2500);

    assert_eq!(
        Duration::from_millis(2500),
        extract_sleep_time(&headers, &query, &args)
    );
}

/// Test that extraction of arbitrary durations from query parameters and headers operate in the correct priority.
#[test]
fn test_extract() {
    let mut query: SleepQueryParams = Default::default();
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
        extract_duration(
            &headers,
            MINIMUM_SLEEP_TIME_MS_HEADER,
            query.min,
            args.min_sleep_ms
        )
    );

    // test fallback to header
    headers.insert(
        HeaderName::from_bytes(MINIMUM_SLEEP_TIME_MS_HEADER.to_lowercase().as_bytes()).unwrap(),
        HeaderValue::from_static("2000"),
    );

    assert_eq!(
        Duration::from_millis(2000),
        extract_duration(
            &headers,
            MINIMUM_SLEEP_TIME_MS_HEADER,
            query.min,
            args.min_sleep_ms
        )
    );

    // test fallback to query
    query.min = Some(3000);

    assert_eq!(
        Duration::from_millis(3000),
        extract_duration(
            &headers,
            MINIMUM_SLEEP_TIME_MS_HEADER,
            query.min,
            args.min_sleep_ms
        )
    );
}

#[test]
fn test_extract_sleep_kind() {
    let mut query: SleepQueryParams = Default::default();
    let mut headers = HeaderMap::new();

    let mut args = CliArgs {
        min_sleep_ms: 1000,
        max_sleep_ms: 3000,
        sleep_ms: 2000,
        verbosity: 0,
        host: "".to_string(),
        json: false,
        port: 8080,
        random: false,
    };

    // test defaults
    assert_eq!(
        SleepKind::Fixed,
        extract_sleep_kind(&headers, &query, &args)
    );

    args.random = true;

    assert_eq!(
        SleepKind::Random,
        extract_sleep_kind(&headers, &query, &args)
    );

    // test headers
    headers.insert(
        HeaderName::from_bytes(SLEEP_KIND_HEADER.to_lowercase().as_bytes()).unwrap(),
        HeaderValue::from_static("fixed"),
    );

    assert_eq!(
        SleepKind::Fixed,
        extract_sleep_kind(&headers, &query, &args)
    );

    headers.insert(
        HeaderName::from_bytes(SLEEP_KIND_HEADER.to_lowercase().as_bytes()).unwrap(),
        HeaderValue::from_static("random"),
    );

    assert_eq!(
        SleepKind::Random,
        extract_sleep_kind(&headers, &query, &args)
    );

    headers.insert(
        HeaderName::from_bytes(SLEEP_KIND_HEADER.to_lowercase().as_bytes()).unwrap(),
        HeaderValue::from_static("unknown"),
    );

    assert_eq!(
        SleepKind::Random,
        extract_sleep_kind(&headers, &query, &args)
    );

    args.random = false;

    assert_eq!(
        SleepKind::Fixed,
        extract_sleep_kind(&headers, &query, &args)
    );

    // test query string
    query.kind = Some(SleepKind::Random);

    assert_eq!(
        SleepKind::Random,
        extract_sleep_kind(&headers, &query, &args)
    );

    query.kind = Some(SleepKind::Fixed);

    assert_eq!(
        SleepKind::Fixed,
        extract_sleep_kind(&headers, &query, &args)
    );
}

#[test]
fn test_sleep_bounds_min() {
    let (req_min, req_max) = (Duration::from_millis(2000), Duration::from_millis(3000));
    let (config_min, config_max) = (Duration::from_millis(1000), Duration::from_millis(4000));

    // test sane bounds
    assert_eq!(
        Duration::from_millis(2000),
        SleepBounds::min(&req_min, &req_max, &config_min, &config_max)
    );

    // test lower bound violated (req < config)
    let req_min = Duration::from_millis(500);

    assert_eq!(
        Duration::from_millis(1000),
        SleepBounds::min(&req_min, &req_max, &config_min, &config_max)
    );

    // test request upper bound violated (req_min > req_max)
    let req_min = Duration::from_millis(3500);

    assert_eq!(
        Duration::from_millis(3000),
        SleepBounds::min(&req_min, &req_max, &config_min, &config_max)
    );
}

#[test]
fn test_sleep_bounds_max() {
    let (req_min, req_max) = (Duration::from_millis(2000), Duration::from_millis(3000));
    let (config_min, config_max) = (Duration::from_millis(1000), Duration::from_millis(4000));

    // test sane bounds
    assert_eq!(
        Duration::from_millis(3000),
        SleepBounds::max(&req_min, &req_max, &config_min, &config_max)
    );

    // test lower bound violation
    let req_max = Duration::from_millis(1500);

    assert_eq!(
        Duration::from_millis(2000),
        SleepBounds::max(&req_min, &req_max, &config_min, &config_max)
    );

    // test upper bound violation
    let req_max = Duration::from_millis(5000);

    assert_eq!(
        Duration::from_millis(4000),
        SleepBounds::max(&req_min, &req_max, &config_min, &config_max)
    );
}

#[test]
fn test_sleep_bounds_duration() {
    let (min, max) = (Duration::from_millis(1000), Duration::from_millis(3000));

    // test sane bounds
    assert_eq!(
        Duration::from_millis(2000),
        SleepBounds::duration(&Duration::from_millis(2000), &min, &max)
    );

    // test lower bound violation
    assert_eq!(
        min,
        SleepBounds::duration(&Duration::from_millis(500), &min, &max)
    );

    // test upper bound violation
    assert_eq!(
        max,
        SleepBounds::duration(&Duration::from_millis(5000), &min, &max)
    );
}
