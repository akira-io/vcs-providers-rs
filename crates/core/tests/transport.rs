mod support;

use vcs_provider_core::{AuthHeaderStyle, RequestMethod, Transport, VcsResult, auth, request};

use support::EchoTransport;

#[test]
fn request_builder_creates_get_request() {
    let request = request()
        .get("https://api.example.test/repos")
        .header("accept", "application/json")
        .build();

    assert_eq!(request.method(), &RequestMethod::Get);
    assert_eq!(request.url().as_str(), "https://api.example.test/repos");
    assert_eq!(request.headers().len(), 1);
    assert_eq!(request.headers()[0].name().as_str(), "accept");
    assert_eq!(request.headers()[0].value().as_str(), "application/json");
}

#[test]
fn request_builder_applies_auth_header() {
    let credential = auth().personal_access_token("test-token");
    let request = request()
        .get("https://api.example.test/repos")
        .auth_header(credential.header(AuthHeaderStyle::AuthorizationBearer))
        .build();

    assert_eq!(request.headers().len(), 1);
    assert_eq!(request.headers()[0].name().as_str(), "authorization");
    assert_eq!(request.headers()[0].value().as_str(), "Bearer test-token");
}

#[test]
fn transport_contract_sends_request_and_returns_response() -> VcsResult<()> {
    let request = request()
        .get("https://api.example.test/repos")
        .header("accept", "application/json")
        .build();
    let response = futures::executor::block_on(EchoTransport.send(request))?;

    assert_eq!(response.status().code(), 200);
    assert_eq!(response.headers().len(), 1);
    assert_eq!(response.headers()[0].name().as_str(), "accept");

    Ok(())
}
