use git_cognition_core::{
    Authentication, CodeReviews, Issues, Organizations, Pipelines, RecordingTransport, Releases,
    Repos, ResponseBuilder, SingleResponseTransport, response, test_transport,
};

use crate::{GitLabClient, GitLabProvider};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabResponseBuilder {
    provider: GitLabProvider,
    response: ResponseBuilder,
}

impl GitLabResponseBuilder {
    pub fn make(provider: GitLabProvider) -> Self {
        Self {
            provider,
            response: response(),
        }
    }

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

    pub fn repos(self) -> Box<dyn Repos> {
        self.client().repos()
    }

    pub fn authentication(self) -> Box<dyn Authentication> {
        self.client().authentication()
    }

    pub fn organizations(self) -> Box<dyn Organizations> {
        self.client().organizations()
    }

    pub fn issues(self) -> Box<dyn Issues> {
        self.client().issues()
    }

    pub fn code_reviews(self) -> Box<dyn CodeReviews> {
        self.client().code_reviews()
    }

    pub fn releases(self) -> Box<dyn Releases> {
        self.client().releases()
    }

    pub fn pipelines(self) -> Box<dyn Pipelines> {
        self.client().pipelines()
    }

    pub fn record(self) -> RecordingTransport {
        RecordingTransport::make(self.response.build())
    }

    fn client(self) -> GitLabClient {
        GitLabClient::with_provider(
            self.provider,
            SingleResponseTransport::make(self.response.build()),
        )
    }
}

impl GitLabProvider {
    pub fn status(self, code: u16) -> GitLabResponseBuilder {
        GitLabResponseBuilder::make(self).status(code)
    }

    pub fn header(
        self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> GitLabResponseBuilder {
        GitLabResponseBuilder::make(self).header(name, value)
    }

    pub fn body(self, body: impl Into<String>) -> GitLabResponseBuilder {
        GitLabResponseBuilder::make(self).body(body)
    }

    pub fn responses(self) -> git_cognition_core::TestTransportSequenceBuilder {
        test_transport().responses()
    }
}
