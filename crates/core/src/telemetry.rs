use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::{BoxFuture, CognitionResult, Request, Response, Transport};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TelemetryEvent {
    RequestStarted(RequestTelemetry),
    RequestFinished(ResponseTelemetry),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequestTelemetry {
    method: String,
    url: String,
}

impl RequestTelemetry {
    pub fn make(request: &Request) -> Self {
        Self {
            method: format!("{:?}", request.method()),
            url: request.url().as_str().to_owned(),
        }
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResponseTelemetry {
    status_code: u16,
    duration_millis: u128,
}

impl ResponseTelemetry {
    pub fn make(response: &Response, duration: Duration) -> Self {
        Self {
            status_code: response.status().code(),
            duration_millis: duration.as_millis(),
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn duration_millis(&self) -> u128 {
        self.duration_millis
    }
}

pub trait TelemetrySink: Send + Sync {
    fn record(&self, event: TelemetryEvent);
}

#[derive(Clone, Debug, Default)]
pub struct TelemetryRecorder {
    events: Arc<Mutex<Vec<TelemetryEvent>>>,
}

impl TelemetryRecorder {
    pub fn events(&self) -> Vec<TelemetryEvent> {
        match self.events.lock() {
            Ok(events) => events.clone(),
            Err(_) => Vec::new(),
        }
    }
}

impl TelemetrySink for TelemetryRecorder {
    fn record(&self, event: TelemetryEvent) {
        self.events.lock().map(|mut events| events.push(event)).ok();
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TelemetryBuilder;

impl TelemetryBuilder {
    pub fn recorder(self) -> TelemetryRecorder {
        TelemetryRecorder::default()
    }

    pub fn request(self) -> RequestTelemetryBuilder {
        RequestTelemetryBuilder
    }

    pub fn response(self) -> ResponseTelemetryBuilder {
        ResponseTelemetryBuilder
    }

    pub fn transport(
        self,
        transport: impl Transport + 'static,
    ) -> TelemetryTransportBuilder<ProvidedTelemetryTransport, MissingTelemetrySink> {
        TelemetryTransportBuilder {
            transport: ProvidedTelemetryTransport {
                transport: Arc::new(transport),
            },
            sink: MissingTelemetrySink,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RequestTelemetryBuilder;

impl RequestTelemetryBuilder {
    pub fn make(self, request: &Request) -> RequestTelemetry {
        RequestTelemetry::make(request)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ResponseTelemetryBuilder;

impl ResponseTelemetryBuilder {
    pub fn make(self, response: &Response, duration: Duration) -> ResponseTelemetry {
        ResponseTelemetry::make(response, duration)
    }
}

#[derive(Clone)]
pub struct ProvidedTelemetryTransport {
    transport: Arc<dyn Transport>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MissingTelemetrySink;

#[derive(Clone)]
pub struct ProvidedTelemetrySink {
    sink: Arc<dyn TelemetrySink>,
}

#[derive(Clone)]
pub struct TelemetryTransportBuilder<TransportState, SinkState> {
    transport: TransportState,
    sink: SinkState,
}

impl<TransportState> TelemetryTransportBuilder<TransportState, MissingTelemetrySink> {
    pub fn sink(
        self,
        sink: impl TelemetrySink + 'static,
    ) -> TelemetryTransportBuilder<TransportState, ProvidedTelemetrySink> {
        TelemetryTransportBuilder {
            transport: self.transport,
            sink: ProvidedTelemetrySink {
                sink: Arc::new(sink),
            },
        }
    }
}

impl TelemetryTransportBuilder<ProvidedTelemetryTransport, ProvidedTelemetrySink> {
    pub fn build(self) -> TelemetryTransport {
        TelemetryTransport {
            transport: self.transport.transport,
            sink: self.sink.sink,
        }
    }
}

#[derive(Clone)]
pub struct TelemetryTransport {
    transport: Arc<dyn Transport>,
    sink: Arc<dyn TelemetrySink>,
}

impl TelemetryTransport {
    pub fn make(transport: impl Transport + 'static, sink: impl TelemetrySink + 'static) -> Self {
        Self {
            transport: Arc::new(transport),
            sink: Arc::new(sink),
        }
    }
}

impl Transport for TelemetryTransport {
    fn send(&self, request: Request) -> BoxFuture<'_, CognitionResult<Response>> {
        Box::pin(async move {
            self.sink
                .record(TelemetryEvent::RequestStarted(RequestTelemetry::make(
                    &request,
                )));

            let started_at = Instant::now();
            let response = self.transport.send(request).await?;

            self.sink
                .record(TelemetryEvent::RequestFinished(ResponseTelemetry::make(
                    &response,
                    started_at.elapsed(),
                )));

            Ok(response)
        })
    }
}
