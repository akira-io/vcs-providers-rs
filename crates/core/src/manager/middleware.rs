use crate::{
    AuthCredential, Authentication, CodeReviews, CognitionManager, CognitionResult,
    HeaderMiddleware, Issues, ManagedClientProvider, Middleware, Organizations, Pipelines,
    Provider, ProviderClient, RateLimitHeaderProfileBuilder, RateLimitRecorder, RateLimitTransport,
    Releases, Repos, Request, Response, RetryPolicy, RetryTransport, TelemetryRecorder,
    TelemetryTransport, Transport, TransportPipelineBuilder, middleware,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ManagedMiddlewareTransportBuilder<Driver, TransportState> {
    manager: CognitionManager<Driver>,
    pipeline: TransportPipelineBuilder<TransportState>,
    retry_enabled: bool,
    retry_attempts: u16,
    retry_status_codes: Vec<u16>,
    rate_limit_enabled: bool,
    rate_limit_headers: RateLimitHeaderProfileBuilder,
    rate_limit_recorder: RateLimitRecorder,
    telemetry_recorder: Option<TelemetryRecorder>,
}

impl<Driver> ManagedMiddlewareTransportBuilder<Driver, crate::ProvidedTransport> {
    pub fn make(manager: CognitionManager<Driver>, transport: impl Transport + 'static) -> Self {
        Self {
            manager,
            pipeline: middleware().transport(transport),
            retry_enabled: false,
            retry_attempts: 3,
            retry_status_codes: vec![429, 500, 502, 503, 504],
            rate_limit_enabled: false,
            rate_limit_headers: RateLimitHeaderProfileBuilder::default(),
            rate_limit_recorder: RateLimitRecorder::default(),
            telemetry_recorder: None,
        }
    }
}

impl<Driver, TransportState> ManagedMiddlewareTransportBuilder<Driver, TransportState> {
    pub fn with(mut self, middleware: impl Middleware + 'static) -> Self {
        self.pipeline = self.pipeline.with(middleware);
        self
    }

    pub fn header(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.with(HeaderMiddleware::make(name, value))
    }

    pub fn retry(mut self) -> Self {
        self.retry_enabled = true;
        self
    }

    pub fn attempts(mut self, retry_attempts: u16) -> Self {
        self.retry_attempts = retry_attempts.max(1);
        self
    }

    pub fn on_status(mut self, status_code: u16) -> Self {
        self.retry_status_codes.push(status_code);
        self
    }

    pub fn on_statuses(mut self, status_codes: impl IntoIterator<Item = u16>) -> Self {
        self.retry_status_codes.extend(status_codes);
        self
    }

    pub fn rate_limit(mut self) -> Self {
        self.rate_limit_enabled = true;
        self
    }

    pub fn remaining(mut self, headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.rate_limit_headers = self.rate_limit_headers.remaining(headers);
        self
    }

    pub fn reset_at(mut self, headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.rate_limit_headers = self.rate_limit_headers.reset_at(headers);
        self
    }

    pub fn retry_after(mut self, headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.rate_limit_headers = self.rate_limit_headers.retry_after(headers);
        self
    }

    pub fn cost(mut self, headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.rate_limit_headers = self.rate_limit_headers.cost(headers);
        self
    }

    pub fn recorder(mut self, recorder: RateLimitRecorder) -> Self {
        self.rate_limit_recorder = recorder;
        self
    }

    pub fn telemetry(mut self, recorder: TelemetryRecorder) -> Self {
        self.telemetry_recorder = Some(recorder);
        self
    }
}

impl<Driver> ManagedMiddlewareTransportBuilder<Driver, crate::ProvidedTransport>
where
    Driver: ManagedClientProvider,
{
    pub fn build(self) -> Driver::Client {
        let manager = self.manager.clone();

        manager.transport(self.composed_transport())
    }

    pub fn auth(self, credential: AuthCredential) -> Driver::Client {
        self.build().auth(credential)
    }

    pub fn repos(self) -> Box<dyn Repos> {
        self.build().repos()
    }

    pub fn authentication(self) -> Box<dyn Authentication> {
        self.build().authentication()
    }

    pub fn organizations(self) -> Box<dyn Organizations> {
        self.build().organizations()
    }

    pub fn issues(self) -> Box<dyn Issues> {
        self.build().issues()
    }

    pub fn code_reviews(self) -> Box<dyn CodeReviews> {
        self.build().code_reviews()
    }

    pub fn pipelines(self) -> Box<dyn Pipelines> {
        self.build().pipelines()
    }

    pub fn releases(self) -> Box<dyn Releases> {
        self.build().releases()
    }

    pub fn transport(self) -> impl Transport {
        self.composed_transport()
    }

    fn composed_transport(self) -> SharedTransport {
        let mut transport: Arc<dyn Transport> = Arc::new(self.pipeline.build());

        if self.rate_limit_enabled {
            transport = Arc::new(RateLimitTransport::make(
                SharedTransport::make(&transport),
                self.rate_limit_headers.build(),
                self.rate_limit_recorder,
            ));
        }

        if self.retry_enabled {
            transport = Arc::new(RetryTransport::make(
                SharedTransport::make(&transport),
                RetryPolicy::make(self.retry_attempts, self.retry_status_codes),
            ));
        }

        if let Some(telemetry_recorder) = self.telemetry_recorder {
            transport = Arc::new(TelemetryTransport::make(
                SharedTransport::make(&transport),
                telemetry_recorder,
            ));
        }

        SharedTransport::make(&transport)
    }
}

#[derive(Clone)]
struct SharedTransport {
    transport: Arc<dyn Transport>,
}

impl SharedTransport {
    fn make(transport: &Arc<dyn Transport>) -> Self {
        Self {
            transport: Arc::clone(transport),
        }
    }
}

impl Transport for SharedTransport {
    fn send(&self, request: Request) -> crate::BoxFuture<'_, CognitionResult<Response>> {
        self.transport.send(request)
    }
}
