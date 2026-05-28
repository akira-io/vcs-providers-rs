use std::sync::Arc;

use crate::{BoxFuture, CognitionResult, Request, RequestHeader, Response, Transport};

pub trait Middleware: Send + Sync {
    fn handle(&self, request: Request) -> BoxFuture<'_, CognitionResult<Request>>;
}

#[derive(Clone)]
pub struct TransportPipeline {
    middleware: Vec<Arc<dyn Middleware>>,
    transport: Arc<dyn Transport>,
}

impl TransportPipeline {
    pub fn builder() -> TransportPipelineBuilder<MissingTransport> {
        TransportPipelineBuilder {
            middleware: Vec::new(),
            transport: MissingTransport,
        }
    }
}

impl Transport for TransportPipeline {
    fn send(&self, request: Request) -> BoxFuture<'_, CognitionResult<Response>> {
        Box::pin(async move {
            let mut current_request = request;

            for middleware in &self.middleware {
                current_request = middleware.handle(current_request).await?;
            }

            self.transport.send(current_request).await
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingTransport;

#[derive(Clone)]
pub struct ProvidedTransport {
    transport: Arc<dyn Transport>,
}

#[derive(Clone)]
pub struct TransportPipelineBuilder<TransportState> {
    middleware: Vec<Arc<dyn Middleware>>,
    transport: TransportState,
}

impl<TransportState> TransportPipelineBuilder<TransportState> {
    pub fn with(mut self, middleware: impl Middleware + 'static) -> Self {
        self.middleware.push(Arc::new(middleware));
        self
    }

    pub fn header(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.with(HeaderMiddleware::make(name, value))
    }
}

impl TransportPipelineBuilder<MissingTransport> {
    pub fn transport(
        self,
        transport: impl Transport + 'static,
    ) -> TransportPipelineBuilder<ProvidedTransport> {
        TransportPipelineBuilder {
            middleware: self.middleware,
            transport: ProvidedTransport {
                transport: Arc::new(transport),
            },
        }
    }
}

impl TransportPipelineBuilder<ProvidedTransport> {
    pub fn build(self) -> TransportPipeline {
        TransportPipeline {
            middleware: self.middleware,
            transport: self.transport.transport,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeaderMiddleware {
    header: RequestHeader,
}

impl HeaderMiddleware {
    pub fn make(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            header: RequestHeader::make(name, value),
        }
    }
}

impl Middleware for HeaderMiddleware {
    fn handle(&self, request: Request) -> BoxFuture<'_, CognitionResult<Request>> {
        let header = self.header.clone();

        Box::pin(async move { Ok(request.with_header(header)) })
    }
}
