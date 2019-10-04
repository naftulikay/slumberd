use serde::Serialize;

use std::time::Duration;

use super::SlumberKind;

use uuid::Uuid;

#[derive(Serialize)]
pub struct SlumberResponse {
    #[serde(rename = "slumber")]
    pub duration: SlumberDuration,
    #[serde(rename = "request_id")]
    pub request_id: Uuid,
}

impl SlumberResponse {
    pub fn builder(
        request_id: &Uuid,
        kind: SlumberKind,
        duration: &Duration,
    ) -> SlumberResponseBuilder {
        SlumberResponseBuilder {
            request_id: request_id.clone(),
            kind,
            duration: duration.clone(),
            min: None,
            max: None,
        }
    }
}

pub struct SlumberResponseBuilder {
    kind: SlumberKind,
    request_id: Uuid,
    duration: Duration,
    min: Option<Duration>,
    max: Option<Duration>,
}

impl SlumberResponseBuilder {
    pub fn min(mut self, duration: &Duration) -> Self {
        self.min = Some(duration.clone());

        self
    }

    pub fn max(mut self, duration: &Duration) -> Self {
        self.max = Some(duration.clone());

        self
    }

    pub fn build(self) -> SlumberResponse {
        SlumberResponse {
            request_id: self.request_id,
            duration: SlumberDuration {
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
pub struct SlumberDuration {
    #[serde(rename = "type")]
    pub kind: SlumberKind,
    #[serde(rename = "time_millis")]
    pub duration_millis: u128,
    #[serde(rename = "time")]
    pub duration_pretty: String,
    #[serde(rename = "max_time", skip_serializing_if = "Option::is_none")]
    pub max_pretty: Option<String>,
    #[serde(rename = "max_time_millis", skip_serializing_if = "Option::is_none")]
    pub max_millis: Option<u128>,
    #[serde(rename = "min_time_millis", skip_serializing_if = "Option::is_none")]
    pub min_millis: Option<u128>,
    #[serde(rename = "min_time", skip_serializing_if = "Option::is_none")]
    pub min_pretty: Option<String>,
}
