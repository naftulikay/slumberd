use serde::Serialize;

use std::time::Duration;

use super::SleepKind;

use uuid::Uuid;

#[derive(Serialize)]
pub struct SleepResponse {
    #[serde(rename = "sleep")]
    pub duration: SleepDuration,
    #[serde(rename = "request_id")]
    pub request_id: Uuid,
}

impl SleepResponse {
    pub fn new(request_id: &Uuid, duration: &Duration, kind: SleepKind) -> Self {
        Self {
            duration: SleepDuration::new(duration, kind),
            request_id: request_id.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct SleepDuration {
    #[serde(rename = "type")]
    pub kind: SleepKind,
    #[serde(rename = "duration_millis")]
    pub millis: u128,
    #[serde(rename = "duration")]
    pub pretty: String,
}

impl SleepDuration {
    pub fn new(duration: &Duration, kind: SleepKind) -> Self {
        Self {
            kind,
            millis: duration.as_millis(),
            pretty: format!("{:?}", duration),
        }
    }
}
