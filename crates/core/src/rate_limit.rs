use serde::{Deserialize, Serialize};

use crate::{Response, Transport};

mod transport;

pub use transport::{
    ProvidedRateLimitTransport, RateLimitRecorder, RateLimitSink, RateLimitTransport,
    RateLimitTransportBuilder,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RateLimitBuilder;

impl RateLimitBuilder {
    pub fn headers(self) -> RateLimitHeaderProfileBuilder {
        RateLimitHeaderProfileBuilder::default()
    }

    pub fn recorder(self) -> RateLimitRecorder {
        RateLimitRecorder::default()
    }

    pub fn transport(
        self,
        transport: impl Transport + 'static,
    ) -> RateLimitTransportBuilder<ProvidedRateLimitTransport> {
        RateLimitTransportBuilder::make(transport)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RateLimitHeaderProfileBuilder {
    remaining_headers: Vec<RateLimitHeaderName>,
    reset_at_headers: Vec<RateLimitHeaderName>,
    retry_after_headers: Vec<RateLimitHeaderName>,
    cost_headers: Vec<RateLimitHeaderName>,
}

impl RateLimitHeaderProfileBuilder {
    pub fn remaining(mut self, headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.remaining_headers.extend(header_names(headers));
        self
    }

    pub fn reset_at(mut self, headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.reset_at_headers.extend(header_names(headers));
        self
    }

    pub fn retry_after(mut self, headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.retry_after_headers.extend(header_names(headers));
        self
    }

    pub fn cost(mut self, headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.cost_headers.extend(header_names(headers));
        self
    }

    pub fn build(self) -> RateLimitHeaderProfile {
        RateLimitHeaderProfile {
            remaining_headers: self.remaining_headers,
            reset_at_headers: self.reset_at_headers,
            retry_after_headers: self.retry_after_headers,
            cost_headers: self.cost_headers,
        }
    }
}

fn header_names(headers: impl IntoIterator<Item = impl Into<String>>) -> Vec<RateLimitHeaderName> {
    headers.into_iter().map(RateLimitHeaderName::make).collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RateLimitHeaderName(String);

impl RateLimitHeaderName {
    pub fn make(header: impl Into<String>) -> Self {
        Self(header.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RateLimitHeaderProfile {
    remaining_headers: Vec<RateLimitHeaderName>,
    reset_at_headers: Vec<RateLimitHeaderName>,
    retry_after_headers: Vec<RateLimitHeaderName>,
    cost_headers: Vec<RateLimitHeaderName>,
}

impl RateLimitHeaderProfile {
    pub fn observe(&self, response: &Response) -> RateLimitObservation {
        RateLimitObservation {
            remaining: find_header(response, &self.remaining_headers)
                .and_then(parse_u64)
                .map(RateLimitQuota::make),
            reset_at: find_header(response, &self.reset_at_headers).map(RateLimitReset::make),
            retry_after: find_header(response, &self.retry_after_headers).map(RetryAfter::make),
            cost: find_header(response, &self.cost_headers)
                .and_then(parse_u64)
                .map(RateLimitCost::make),
        }
    }
}

fn find_header<'a>(response: &'a Response, names: &[RateLimitHeaderName]) -> Option<&'a str> {
    response.headers().iter().find_map(|header| {
        names
            .iter()
            .any(|name| header.name().as_str().eq_ignore_ascii_case(name.as_str()))
            .then(|| header.value().as_str())
    })
}

fn parse_u64(raw_header: &str) -> Option<u64> {
    raw_header.parse::<u64>().ok()
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RateLimitObservation {
    remaining: Option<RateLimitQuota>,
    reset_at: Option<RateLimitReset>,
    retry_after: Option<RetryAfter>,
    cost: Option<RateLimitCost>,
}

impl RateLimitObservation {
    pub fn remaining(&self) -> Option<&RateLimitQuota> {
        self.remaining.as_ref()
    }

    pub fn reset_at(&self) -> Option<&RateLimitReset> {
        self.reset_at.as_ref()
    }

    pub fn retry_after(&self) -> Option<&RetryAfter> {
        self.retry_after.as_ref()
    }

    pub fn cost(&self) -> Option<&RateLimitCost> {
        self.cost.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RateLimitQuota(u64);

impl RateLimitQuota {
    pub fn make(quota: u64) -> Self {
        Self(quota)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RateLimitCost(u64);

impl RateLimitCost {
    pub fn make(cost: u64) -> Self {
        Self(cost)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RateLimitReset(String);

impl RateLimitReset {
    pub fn make(reset_at: impl Into<String>) -> Self {
        Self(reset_at.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetryAfter(String);

impl RetryAfter {
    pub fn make(retry_after: impl Into<String>) -> Self {
        Self(retry_after.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
