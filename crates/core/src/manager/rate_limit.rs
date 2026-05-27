use crate::{
    AuthCredential, CodeReviews, Issues, ManagedClientProvider, Pipelines, Provider,
    ProviderClient, RateLimitHeaderProfileBuilder, RateLimitRecorder, RateLimitTransport, Releases,
    Repos, Transport, VcsManager,
};

#[derive(Clone)]
pub struct ManagedRateLimitTransportBuilder<Driver, TransportKind> {
    manager: VcsManager<Driver>,
    transport: TransportKind,
    headers: RateLimitHeaderProfileBuilder,
    recorder: RateLimitRecorder,
}

impl<Driver, TransportKind> ManagedRateLimitTransportBuilder<Driver, TransportKind> {
    pub fn make(manager: VcsManager<Driver>, transport: TransportKind) -> Self {
        Self {
            manager,
            transport,
            headers: RateLimitHeaderProfileBuilder::default(),
            recorder: RateLimitRecorder::default(),
        }
    }

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

impl<Driver, TransportKind> ManagedRateLimitTransportBuilder<Driver, TransportKind>
where
    Driver: ManagedClientProvider,
    TransportKind: Transport + 'static,
{
    pub fn build(self) -> Driver::Client {
        self.manager.transport(RateLimitTransport::make(
            self.transport,
            self.headers.build(),
            self.recorder,
        ))
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
}
