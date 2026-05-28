mod support;

use std::sync::{Arc, Mutex};

use git_cognition_core::{
    BoxFuture, CognitionResult, Middleware, Request, Transport, middleware, request, run_async_test,
};

use support::EchoTransport;

#[derive(Clone, Debug)]
struct RecordingMiddleware {
    name: &'static str,
    calls: Arc<Mutex<Vec<&'static str>>>,
}

impl Middleware for RecordingMiddleware {
    fn handle(&self, request: Request) -> BoxFuture<'_, CognitionResult<Request>> {
        let calls = Arc::clone(&self.calls);
        let name = self.name;

        Box::pin(async move {
            calls.lock().map(|mut calls| calls.push(name)).ok();
            Ok(request)
        })
    }
}

fn recording_middleware(
    name: &'static str,
    calls: Arc<Mutex<Vec<&'static str>>>,
) -> RecordingMiddleware {
    RecordingMiddleware { name, calls }
}

#[test]
fn middleware_pipeline_runs_middleware_before_transport() -> CognitionResult<()> {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let pipeline = middleware()
        .with(recording_middleware("first", Arc::clone(&calls)))
        .with(recording_middleware("second", Arc::clone(&calls)))
        .transport(EchoTransport)
        .build();
    let request = request().get("https://api.example.test/repos").build();

    let response = run_async_test(pipeline.send(request))?;

    assert_eq!(response.status().code(), 200);
    assert_eq!(
        calls.lock().map(|calls| calls.clone()).ok(),
        Some(vec!["first", "second"])
    );

    Ok(())
}

#[test]
fn header_middleware_adds_request_header() -> CognitionResult<()> {
    let pipeline = middleware()
        .header("accept", "application/json")
        .transport(EchoTransport)
        .build();
    let request = request().get("https://api.example.test/repos").build();

    let response = run_async_test(pipeline.send(request))?;

    assert_eq!(response.headers().len(), 1);
    assert_eq!(response.headers()[0].name().as_str(), "accept");
    assert_eq!(response.headers()[0].value().as_str(), "application/json");

    Ok(())
}
