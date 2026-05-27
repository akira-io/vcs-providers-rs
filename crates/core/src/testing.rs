use std::future::Future;
use std::sync::{Arc, Mutex};

#[cfg(feature = "testing")]
#[path = "testing/conformance.rs"]
mod conformance;

use crate::{BoxFuture, Request, Response, ResponseBuilder, Transport, VcsResult, response};

#[cfg(feature = "testing")]
pub use conformance::{ProviderConformance, ProviderConformanceBuilder};

#[derive(Clone, Copy, Debug, Default)]
pub struct EchoTransport;

impl Transport for EchoTransport {
    fn send(&self, request: Request) -> BoxFuture<'_, VcsResult<Response>> {
        Box::pin(async move {
            let mut response = response().status(200);

            for header in request.headers() {
                response = response.header(header.name().as_str(), header.value().as_str());
            }

            Ok(response.build())
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SingleResponseTransport {
    response: Response,
}

impl SingleResponseTransport {
    pub fn make(response: Response) -> Self {
        Self { response }
    }
}

impl Transport for SingleResponseTransport {
    fn send(&self, _request: Request) -> BoxFuture<'_, VcsResult<Response>> {
        let response = self.response.clone();

        Box::pin(async move { Ok(response) })
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderResponseBuilder;

impl ProviderResponseBuilder {
    pub fn status(self, code: u16) -> ProviderResponseTransportBuilder {
        self.response().status(code)
    }

    pub fn header(
        self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> ProviderResponseTransportBuilder {
        self.response().header(name, value)
    }

    pub fn body(self, body: impl Into<String>) -> ProviderResponseTransportBuilder {
        self.response().body(body)
    }

    fn response(self) -> ProviderResponseTransportBuilder {
        ProviderResponseTransportBuilder {
            response: response(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderResponseTransportBuilder {
    response: ResponseBuilder,
}

impl ProviderResponseTransportBuilder {
    pub fn status(mut self, code: u16) -> Self {
        self.response = self.response.status(code);
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.response = self.response.header(name, value);
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.response = self.response.body(body);
        self
    }

    pub fn get(self) -> SingleResponseTransport {
        SingleResponseTransport::make(self.response.build())
    }

    pub fn record(self) -> RecordingTransport {
        RecordingTransport::make(self.response.build())
    }
}

pub fn provider_response() -> ProviderResponseBuilder {
    ProviderResponseBuilder
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProviderResponseSequenceBuilder {
    responses: Vec<Response>,
}

impl ProviderResponseSequenceBuilder {
    pub fn response(self) -> ProviderResponseSequenceResponseBuilder {
        ProviderResponseSequenceResponseBuilder {
            responses: self,
            response: response(),
        }
    }

    pub fn status(self, code: u16) -> Self {
        self.append_response(response().status(code).build())
    }

    pub fn body(self, body: impl Into<String>) -> Self {
        self.append_response(response().body(body).build())
    }

    pub fn record(self) -> ResponseSequenceTransport {
        ResponseSequenceTransport::make(self.responses)
    }

    fn append_response(mut self, provider_response: Response) -> Self {
        self.responses.push(provider_response);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderResponseSequenceResponseBuilder {
    responses: ProviderResponseSequenceBuilder,
    response: ResponseBuilder,
}

impl ProviderResponseSequenceResponseBuilder {
    pub fn status(mut self, code: u16) -> Self {
        self.response = self.response.status(code);
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.response = self.response.header(name, value);
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.response = self.response.body(body);
        self
    }

    pub fn next_response(self) -> ProviderResponseSequenceBuilder {
        self.responses.append_response(self.response.build())
    }
}

pub fn provider_responses() -> ProviderResponseSequenceBuilder {
    ProviderResponseSequenceBuilder::default()
}

pub fn run_async_test<T>(future: impl Future<Output = VcsResult<T>>) -> VcsResult<T> {
    futures::executor::block_on(future)
}

#[derive(Clone, Debug)]
pub struct RecordingTransport {
    response: Response,
    requests: Arc<Mutex<Vec<Request>>>,
}

impl RecordingTransport {
    pub fn make(response: Response) -> Self {
        Self {
            response,
            requests: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn requests(&self) -> Vec<Request> {
        self.requests
            .lock()
            .map(|requests| requests.clone())
            .unwrap_or_default()
    }
}

#[cfg(feature = "testing")]
pub fn conformance() -> ProviderConformanceBuilder {
    ProviderConformanceBuilder
}

impl Transport for RecordingTransport {
    fn send(&self, request: Request) -> BoxFuture<'_, VcsResult<Response>> {
        let response = self.response.clone();
        let requests = Arc::clone(&self.requests);

        Box::pin(async move {
            requests
                .lock()
                .map(|mut recorded_requests| recorded_requests.push(request))
                .ok();

            Ok(response)
        })
    }
}

#[derive(Clone, Debug)]
pub struct ResponseSequenceTransport {
    responses: Vec<Response>,
    requests: Arc<Mutex<Vec<Request>>>,
}

impl ResponseSequenceTransport {
    pub fn make(responses: impl IntoIterator<Item = Response>) -> Self {
        Self {
            responses: responses.into_iter().collect(),
            requests: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn requests(&self) -> Vec<Request> {
        self.requests
            .lock()
            .map(|requests| requests.clone())
            .unwrap_or_default()
    }

    fn response_for_request(&self, request_index: usize) -> Response {
        if let Some(response) = self.responses.get(request_index) {
            return response.clone();
        }

        if let Some(response) = self.responses.last() {
            return response.clone();
        }

        response().build()
    }
}

impl Transport for ResponseSequenceTransport {
    fn send(&self, request: Request) -> BoxFuture<'_, VcsResult<Response>> {
        let requests = Arc::clone(&self.requests);
        let response = self.response_for_request(self.requests().len());

        Box::pin(async move {
            requests
                .lock()
                .map(|mut recorded_requests| recorded_requests.push(request))
                .ok();

            Ok(response)
        })
    }
}
