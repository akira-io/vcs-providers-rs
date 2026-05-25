use std::future::Future;

use crate::{BoxFuture, Request, Response, ResponseBuilder, Transport, VcsResult, response};

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
}

pub fn provider_response() -> ProviderResponseBuilder {
    ProviderResponseBuilder
}

pub fn run_async_test<T>(future: impl Future<Output = VcsResult<T>>) -> VcsResult<T> {
    futures::executor::block_on(future)
}
