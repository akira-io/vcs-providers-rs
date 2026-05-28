use git_cognition_core::{
    Authentication, CodeReviews, Issues, Organizations, Pipelines, RecordingTransport, Repos,
    ResponseBuilder, SingleResponseTransport, response, test_transport,
};

use crate::{BitbucketClient, BitbucketProvider};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitbucketResponseBuilder {
    provider: BitbucketProvider,
    response: ResponseBuilder,
}

impl BitbucketResponseBuilder {
    pub fn make(provider: BitbucketProvider) -> Self {
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

    pub fn pipelines(self) -> Box<dyn Pipelines> {
        self.client().pipelines()
    }

    pub fn record(self) -> RecordingTransport {
        RecordingTransport::make(self.response.build())
    }

    fn client(self) -> BitbucketClient {
        BitbucketClient::with_provider(
            self.provider,
            SingleResponseTransport::make(self.response.build()),
        )
    }
}

impl BitbucketProvider {
    pub fn status(self, code: u16) -> BitbucketResponseBuilder {
        BitbucketResponseBuilder::make(self).status(code)
    }

    pub fn header(
        self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> BitbucketResponseBuilder {
        BitbucketResponseBuilder::make(self).header(name, value)
    }

    pub fn body(self, body: impl Into<String>) -> BitbucketResponseBuilder {
        BitbucketResponseBuilder::make(self).body(body)
    }

    pub fn responses(self) -> git_cognition_core::TestTransportSequenceBuilder {
        test_transport().responses()
    }
}
