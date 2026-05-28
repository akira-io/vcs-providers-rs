use std::sync::Arc;

use crate::{
    AuthCredential, BoxFuture, CognitionError, CognitionResult, Provider, Request, RequestBuilder,
    RequestUrl, Response, TelemetrySink, Transport, error, middleware, telemetry, url,
};

mod configuration;

pub use configuration::{RuntimeConfiguredProvider, RuntimeProviderConfigurationBuilder};

pub trait IntoProvider {
    type Provider: Provider + 'static;

    fn into_provider(self) -> Self::Provider;
}

impl<T> IntoProvider for T
where
    T: Provider + 'static,
{
    type Provider = T;

    fn into_provider(self) -> Self::Provider {
        self
    }
}

#[derive(Clone)]
pub struct ProviderRuntime {
    provider: Arc<dyn Provider>,
    transport: Arc<dyn Transport>,
    telemetry_sink: Option<Arc<dyn TelemetrySink>>,
}

impl ProviderRuntime {
    pub fn request(&self) -> ProviderRequestBuilder {
        ProviderRequestBuilder {
            runtime: self.clone(),
            request: crate::request(),
        }
    }

    pub fn execute(&self, request: Request) -> BoxFuture<'_, CognitionResult<Response>> {
        Box::pin(async move {
            let response = self.transport().send(request).await?;

            if let Some(error) = error().from_status(response.status()) {
                return Err(error);
            }

            Ok(response)
        })
    }

    fn transport(&self) -> Arc<dyn Transport> {
        let pipeline = middleware()
            .transport(ArcTransport::make(&self.transport))
            .build();

        match &self.telemetry_sink {
            Some(sink) => Arc::new(
                telemetry()
                    .transport(pipeline)
                    .sink(ArcTelemetrySink::make(sink))
                    .build(),
            ),
            None => Arc::new(pipeline),
        }
    }
}

#[derive(Clone)]
pub struct ProviderRequestBuilder {
    runtime: ProviderRuntime,
    request: RequestBuilder,
}

impl ProviderRequestBuilder {
    pub fn get(self, path: impl Into<String>) -> Self {
        let url = self.url(path);

        self.with_request(|request| request.get(url.as_str()))
    }

    pub fn post(self, path: impl Into<String>) -> Self {
        let url = self.url(path);

        self.with_request(|request| request.post(url.as_str()))
    }

    pub fn auth(self, credential: &AuthCredential) -> Self {
        let auth_header = self.runtime.provider.auth_header(credential);

        self.with_request(|request| request.auth_header(auth_header))
    }

    pub fn header(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.with_request(|request| request.header(name, value))
    }

    pub fn build(self) -> Request {
        self.request.build()
    }

    pub fn send(self) -> BoxFuture<'static, CognitionResult<Response>> {
        let runtime = self.runtime.clone();
        let request = self.build();

        Box::pin(async move { runtime.execute(request).await })
    }

    fn url(&self, path: impl Into<String>) -> RequestUrl {
        url(self.runtime.provider.default_base_url())
            .path(path.into())
            .build()
    }

    fn with_request(self, update: impl FnOnce(RequestBuilder) -> RequestBuilder) -> Self {
        Self {
            runtime: self.runtime,
            request: update(self.request),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderRuntimeBuilder;

impl ProviderRuntimeBuilder {
    pub fn provider(self) -> RuntimeProviderConfigurationBuilder {
        RuntimeProviderConfigurationBuilder::default()
    }

    pub fn with_provider(
        self,
        provider: impl IntoProvider,
    ) -> ProviderRuntimeWithProviderBuilder<MissingProviderTransport> {
        self.provider().from(provider)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MissingProviderTransport;

#[derive(Clone)]
pub struct ProvidedProviderTransport {
    transport: Arc<dyn Transport>,
}

#[derive(Clone)]
pub struct ProviderRuntimeWithProviderBuilder<TransportState> {
    provider: Arc<dyn Provider>,
    transport: TransportState,
    telemetry_sink: Option<Arc<dyn TelemetrySink>>,
}

impl ProviderRuntimeWithProviderBuilder<MissingProviderTransport> {
    pub fn transport(
        self,
        transport: impl Transport + 'static,
    ) -> ProviderRuntimeWithProviderBuilder<ProvidedProviderTransport> {
        ProviderRuntimeWithProviderBuilder {
            provider: self.provider,
            transport: ProvidedProviderTransport {
                transport: Arc::new(transport),
            },
            telemetry_sink: self.telemetry_sink,
        }
    }
}

impl<TransportState> ProviderRuntimeWithProviderBuilder<TransportState> {
    pub fn telemetry(self, sink: impl TelemetrySink + 'static) -> Self {
        Self {
            provider: self.provider,
            transport: self.transport,
            telemetry_sink: Some(Arc::new(sink)),
        }
    }
}

impl ProviderRuntimeWithProviderBuilder<ProvidedProviderTransport> {
    pub fn build(self) -> ProviderRuntime {
        ProviderRuntime {
            provider: self.provider,
            transport: self.transport.transport,
            telemetry_sink: self.telemetry_sink,
        }
    }

    pub fn request(self) -> ProviderRequestBuilder {
        self.build().request()
    }

    pub fn execute(self, request: Request) -> BoxFuture<'static, CognitionResult<Response>> {
        let runtime = self.build();

        Box::pin(async move { runtime.execute(request).await })
    }
}

#[derive(Clone)]
struct ArcTransport {
    transport: Arc<dyn Transport>,
}

impl ArcTransport {
    fn make(transport: &Arc<dyn Transport>) -> Self {
        Self {
            transport: Arc::clone(transport),
        }
    }
}

impl Transport for ArcTransport {
    fn send(&self, request: Request) -> BoxFuture<'_, CognitionResult<Response>> {
        self.transport.send(request)
    }
}

#[derive(Clone)]
struct ArcTelemetrySink {
    sink: Arc<dyn TelemetrySink>,
}

impl ArcTelemetrySink {
    fn make(sink: &Arc<dyn TelemetrySink>) -> Self {
        Self {
            sink: Arc::clone(sink),
        }
    }
}

impl TelemetrySink for ArcTelemetrySink {
    fn record(&self, event: crate::TelemetryEvent) {
        self.sink.record(event);
    }
}

pub fn transport_status_error(response: &Response) -> Option<CognitionError> {
    error().from_response(response)
}
