use crate::{
    AuthCredential, CodeReviews, Issues, ManagedClientProvider, Pipelines, Provider,
    ProviderClient, Releases, Repos, RetryPolicy, RetryTransport, Transport, VcsManager,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRetryTransportBuilder<Driver, TransportKind> {
    manager: VcsManager<Driver>,
    transport: TransportKind,
    max_attempts: u16,
    retry_status_codes: Vec<u16>,
}

impl<Driver, TransportKind> ManagedRetryTransportBuilder<Driver, TransportKind> {
    pub fn make(manager: VcsManager<Driver>, transport: TransportKind) -> Self {
        Self {
            manager,
            transport,
            max_attempts: 3,
            retry_status_codes: vec![429, 500, 502, 503, 504],
        }
    }

    pub fn attempts(mut self, max_attempts: u16) -> Self {
        self.max_attempts = max_attempts.max(1);
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
}

impl<Driver, TransportKind> ManagedRetryTransportBuilder<Driver, TransportKind>
where
    Driver: ManagedClientProvider,
    TransportKind: Transport + 'static,
{
    pub fn build(self) -> Driver::Client {
        let policy = RetryPolicy::make(self.max_attempts, self.retry_status_codes);

        self.manager
            .transport(RetryTransport::make(self.transport, policy))
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
