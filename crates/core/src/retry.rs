use std::sync::Arc;

use crate::{BoxFuture, CognitionResult, Request, Response, Transport};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RetryBuilder;

impl RetryBuilder {
    pub fn transport(
        self,
        transport: impl Transport + 'static,
    ) -> RetryTransportBuilder<ProvidedRetryTransport> {
        RetryTransportBuilder {
            transport: ProvidedRetryTransport {
                transport: Arc::new(transport),
            },
            max_attempts: 3,
            retry_status_codes: default_retry_status_codes(),
        }
    }
}

#[derive(Clone)]
pub struct ProvidedRetryTransport {
    transport: Arc<dyn Transport>,
}

#[derive(Clone)]
pub struct RetryTransportBuilder<TransportState> {
    transport: TransportState,
    max_attempts: u16,
    retry_status_codes: Vec<u16>,
}

impl<TransportState> RetryTransportBuilder<TransportState> {
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

impl RetryTransportBuilder<ProvidedRetryTransport> {
    pub fn build(self) -> RetryTransport {
        RetryTransport {
            transport: self.transport.transport,
            policy: RetryPolicy::make(self.max_attempts, self.retry_status_codes),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetryPolicy {
    max_attempts: u16,
    retry_status_codes: Vec<u16>,
}

impl RetryPolicy {
    pub fn make(max_attempts: u16, retry_status_codes: impl IntoIterator<Item = u16>) -> Self {
        Self {
            max_attempts: max_attempts.max(1),
            retry_status_codes: retry_status_codes.into_iter().collect(),
        }
    }

    pub fn max_attempts(&self) -> u16 {
        self.max_attempts
    }

    pub fn retry_status_codes(&self) -> &[u16] {
        &self.retry_status_codes
    }

    pub fn should_retry(&self, response: &Response) -> bool {
        self.retry_status_codes
            .iter()
            .any(|status_code| response.status().code() == *status_code)
    }
}

#[derive(Clone)]
pub struct RetryTransport {
    transport: Arc<dyn Transport>,
    policy: RetryPolicy,
}

impl RetryTransport {
    pub fn make(transport: impl Transport + 'static, policy: RetryPolicy) -> Self {
        Self {
            transport: Arc::new(transport),
            policy,
        }
    }

    pub fn policy(&self) -> &RetryPolicy {
        &self.policy
    }
}

impl Transport for RetryTransport {
    fn send(&self, request: Request) -> BoxFuture<'_, CognitionResult<Response>> {
        Box::pin(async move {
            let mut attempts_made = 1;

            loop {
                let response = self.transport.send(request.clone()).await?;

                if !self.policy.should_retry(&response) {
                    return Ok(response);
                }

                if attempts_made >= self.policy.max_attempts() {
                    return Ok(response);
                }

                attempts_made += 1;
            }
        })
    }
}

fn default_retry_status_codes() -> Vec<u16> {
    vec![429, 500, 502, 503, 504]
}
