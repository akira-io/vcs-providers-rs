use crate::{
    AuthCredential, CodeReviews, HeaderMiddleware, Issues, ManagedClientProvider, Middleware,
    Pipelines, Provider, ProviderClient, Releases, Repos, Transport, TransportPipeline,
    TransportPipelineBuilder, VcsManager, middleware,
};

#[derive(Clone)]
pub struct ManagedMiddlewareTransportBuilder<Driver, TransportState> {
    manager: VcsManager<Driver>,
    pipeline: TransportPipelineBuilder<TransportState>,
}

impl<Driver> ManagedMiddlewareTransportBuilder<Driver, crate::ProvidedTransport> {
    pub fn make(manager: VcsManager<Driver>, transport: impl Transport + 'static) -> Self {
        Self {
            manager,
            pipeline: middleware().transport(transport),
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
}

impl<Driver> ManagedMiddlewareTransportBuilder<Driver, crate::ProvidedTransport>
where
    Driver: ManagedClientProvider,
{
    pub fn build(self) -> Driver::Client {
        self.manager.transport(self.pipeline.build())
    }

    pub fn auth(self, credential: AuthCredential) -> Driver::Client {
        self.build().auth(credential)
    }

    pub fn repos(self) -> Box<dyn Repos> {
        self.build().repos()
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

    pub fn transport(self) -> TransportPipeline {
        self.pipeline.build()
    }
}
