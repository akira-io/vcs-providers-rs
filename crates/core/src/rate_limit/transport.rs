use std::sync::{Arc, Mutex};

use super::{RateLimitHeaderProfile, RateLimitHeaderProfileBuilder, RateLimitObservation};
use crate::{BoxFuture, CognitionResult, Request, Response, Transport};

#[derive(Clone)]
pub struct ProvidedRateLimitTransport {
    transport: Arc<dyn Transport>,
}

#[derive(Clone)]
pub struct RateLimitTransportBuilder<TransportState> {
    transport: TransportState,
    headers: RateLimitHeaderProfileBuilder,
    recorder: RateLimitRecorder,
}

impl RateLimitTransportBuilder<ProvidedRateLimitTransport> {
    pub fn make(transport: impl Transport + 'static) -> Self {
        Self {
            transport: ProvidedRateLimitTransport {
                transport: Arc::new(transport),
            },
            headers: RateLimitHeaderProfileBuilder::default(),
            recorder: RateLimitRecorder::default(),
        }
    }

    pub fn build(self) -> RateLimitTransport {
        RateLimitTransport {
            transport: self.transport.transport,
            headers: self.headers.build(),
            recorder: self.recorder,
        }
    }
}

impl<TransportState> RateLimitTransportBuilder<TransportState> {
    pub fn remaining(mut self, headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.headers = self.headers.remaining(headers);
        self
    }

    pub fn reset_at(mut self, headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.headers = self.headers.reset_at(headers);
        self
    }

    pub fn retry_after(mut self, headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.headers = self.headers.retry_after(headers);
        self
    }

    pub fn cost(mut self, headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.headers = self.headers.cost(headers);
        self
    }

    pub fn recorder(mut self, recorder: RateLimitRecorder) -> Self {
        self.recorder = recorder;
        self
    }
}

pub trait RateLimitSink: Clone + Send + Sync + 'static {
    fn record(&self, observation: RateLimitObservation);
}

#[derive(Clone, Debug, Default)]
pub struct RateLimitRecorder {
    observations: Arc<Mutex<Vec<RateLimitObservation>>>,
}

impl RateLimitRecorder {
    pub fn observations(&self) -> Vec<RateLimitObservation> {
        self.observations
            .lock()
            .map(|observations| observations.clone())
            .unwrap_or_default()
    }
}

impl RateLimitSink for RateLimitRecorder {
    fn record(&self, observation: RateLimitObservation) {
        self.observations
            .lock()
            .map(|mut observations| observations.push(observation))
            .ok();
    }
}

#[derive(Clone)]
pub struct RateLimitTransport {
    transport: Arc<dyn Transport>,
    headers: RateLimitHeaderProfile,
    recorder: RateLimitRecorder,
}

impl RateLimitTransport {
    pub fn make(
        transport: impl Transport + 'static,
        headers: RateLimitHeaderProfile,
        recorder: RateLimitRecorder,
    ) -> Self {
        Self {
            transport: Arc::new(transport),
            headers,
            recorder,
        }
    }
}

impl Transport for RateLimitTransport {
    fn send(&self, request: Request) -> BoxFuture<'_, CognitionResult<Response>> {
        Box::pin(async move {
            let response = self.transport.send(request).await?;
            self.recorder.record(self.headers.observe(&response));

            Ok(response)
        })
    }
}
