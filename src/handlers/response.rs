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
    pub fn builder(
        request_id: &Uuid,
        kind: SleepKind,
        duration: &Duration,
    ) -> SleepResponseBuilder {
        SleepResponseBuilder {
            request_id: request_id.clone(),
            kind,
            duration: duration.clone(),
            min: None,
            max: None,
        }
    }
}

pub struct SleepResponseBuilder {
    kind: SleepKind,
    request_id: Uuid,
    duration: Duration,
    min: Option<Duration>,
    max: Option<Duration>,
}

impl SleepResponseBuilder {
    pub fn min(mut self, duration: &Duration) -> Self {
        self.min = Some(duration.clone());

        self
    }

    pub fn max(mut self, duration: &Duration) -> Self {
        self.max = Some(duration.clone());

        self
    }

    pub fn build(self) -> SleepResponse {
        SleepResponse {
            request_id: self.request_id,
            duration: SleepDuration {
                kind: self.kind,
                duration_millis: self.duration.as_millis(),
                duration_pretty: format!("{:?}", self.duration),
                max_pretty: self.max.as_ref().map(|d| format!("{:?}", d)),
                max_millis: self.max.as_ref().map(|d| d.as_millis()),
                min_pretty: self.min.as_ref().map(|d| format!("{:?}", d)),
                min_millis: self.min.as_ref().map(|d| d.as_millis()),
            },
        }
    }
}

#[derive(Serialize)]
pub struct SleepDuration {
    #[serde(rename = "type")]
    pub kind: SleepKind,
    #[serde(rename = "duration_millis")]
    pub duration_millis: u128,
    #[serde(rename = "duration")]
    pub duration_pretty: String,
    #[serde(rename = "max_duration", skip_serializing_if = "Option::is_none")]
    pub max_pretty: Option<String>,
    #[serde(
        rename = "max_duration_millis",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_millis: Option<u128>,
    #[serde(
        rename = "min_duration_millis",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_millis: Option<u128>,
    #[serde(rename = "min_duration", skip_serializing_if = "Option::is_none")]
    pub min_pretty: Option<String>,
}
