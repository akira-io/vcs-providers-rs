mod support;

use std::sync::{Arc, Mutex};

use vcs_provider_core::{
    BoxFuture, HeaderMiddleware, Middleware, Request, Transport, VcsResult, middleware, request,
};

use support::EchoTransport;

#[derive(Clone, Debug)]
struct RecordingMiddleware {
    name: &'static str,
    calls: Arc<Mutex<Vec<&'static str>>>,
}

impl RecordingMiddleware {
    fn make(name: &'static str, calls: Arc<Mutex<Vec<&'static str>>>) -> Self {
        Self { name, calls }
    }
}

impl Middleware for RecordingMiddleware {
    fn handle(&self, request: Request) -> BoxFuture<'_, VcsResult<Request>> {
        let calls = Arc::clone(&self.calls);
        let name = self.name;

        Box::pin(async move {
            calls.lock().map(|mut calls| calls.push(name)).ok();
            Ok(request)
        })
    }
}

#[test]
fn middleware_pipeline_runs_middleware_before_transport() -> VcsResult<()> {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let pipeline = middleware()
        .with(RecordingMiddleware::make("first", Arc::clone(&calls)))
        .with(RecordingMiddleware::make("second", Arc::clone(&calls)))
        .transport(EchoTransport)
        .build();
    let request = request().get("https://api.example.test/repos").build();

    let response = futures::executor::block_on(pipeline.send(request))?;

    assert_eq!(response.status().code(), 200);
    assert_eq!(
        calls.lock().map(|calls| calls.clone()).ok(),
        Some(vec!["first", "second"])
    );

    Ok(())
}

#[test]
fn header_middleware_adds_request_header() -> VcsResult<()> {
    let pipeline = middleware()
        .with(HeaderMiddleware::make("accept", "application/json"))
        .transport(EchoTransport)
        .build();
    let request = request().get("https://api.example.test/repos").build();

    let response = futures::executor::block_on(pipeline.send(request))?;

    assert_eq!(response.headers().len(), 1);
    assert_eq!(response.headers()[0].name().as_str(), "accept");
    assert_eq!(response.headers()[0].value().as_str(), "application/json");

    Ok(())
}
