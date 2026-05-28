use std::time::Duration;

use crate::{
    BoxFuture, CognitionError, CognitionResult, Request, RequestMethod, Response, ResponseBuilder,
    Transport,
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HttpBuilder;

impl HttpBuilder {
    pub fn transport(self) -> HttpTransportBuilder {
        HttpTransportBuilder::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpTransportBuilder {
    timeout: Duration,
    user_agent: String,
}

impl HttpTransportBuilder {
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn get(self) -> CognitionResult<HttpTransport> {
        HttpTransport::make(self)
    }
}

impl Default for HttpTransportBuilder {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            user_agent: "git-cognition-rs".into(),
        }
    }
}

#[derive(Clone)]
pub struct HttpTransport {
    client: reqwest::Client,
}

impl HttpTransport {
    fn make(builder: HttpTransportBuilder) -> CognitionResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(builder.timeout)
            .user_agent(builder.user_agent)
            .build()
            .map_err(|error| CognitionError::InvalidInput(error.to_string()))?;

        Ok(Self { client })
    }
}

impl Transport for HttpTransport {
    fn send(&self, request: Request) -> BoxFuture<'_, CognitionResult<Response>> {
        Box::pin(async move {
            let response = self
                .client
                .request(method(request.method()), request.url().as_str())
                .headers(headers(&request)?)
                .body(body(&request))
                .send()
                .await
                .map_err(transport_error)?;

            from_response(response).await
        })
    }
}

fn method(method: &RequestMethod) -> reqwest::Method {
    match method {
        RequestMethod::Get => reqwest::Method::GET,
        RequestMethod::Post => reqwest::Method::POST,
        RequestMethod::Put => reqwest::Method::PUT,
        RequestMethod::Patch => reqwest::Method::PATCH,
        RequestMethod::Delete => reqwest::Method::DELETE,
    }
}

fn headers(request: &Request) -> CognitionResult<reqwest::header::HeaderMap> {
    let mut headers = reqwest::header::HeaderMap::new();

    for header in request.headers() {
        let name = reqwest::header::HeaderName::from_bytes(header.name().as_str().as_bytes())
            .map_err(|error| CognitionError::InvalidInput(error.to_string()))?;
        let value = reqwest::header::HeaderValue::from_str(header.value().as_str())
            .map_err(|error| CognitionError::InvalidInput(error.to_string()))?;

        headers.insert(name, value);
    }

    Ok(headers)
}

fn body(request: &Request) -> String {
    match request.body() {
        Some(body) => body.as_str().to_owned(),
        None => String::new(),
    }
}

async fn from_response(http_response: reqwest::Response) -> CognitionResult<Response> {
    let status = http_response.status().as_u16();
    let mut builder = headers_from_response(
        http_response.headers(),
        ResponseBuilder::default().status(status),
    );
    let body = http_response.text().await.map_err(transport_error)?;

    if body.is_empty() {
        return Ok(builder.build());
    }

    builder = builder.body(body);

    Ok(builder.build())
}

fn transport_error(error: reqwest::Error) -> CognitionError {
    if error.is_builder() {
        return CognitionError::InvalidInput(error.to_string());
    }

    if error.is_request() {
        return CognitionError::InvalidInput(error.to_string());
    }

    CognitionError::ProviderUnavailable
}

fn headers_from_response(
    headers: &reqwest::header::HeaderMap,
    builder: ResponseBuilder,
) -> ResponseBuilder {
    headers.iter().fold(builder, |response, (name, value)| {
        let Ok(value) = value.to_str() else {
            return response;
        };

        response.header(name.as_str(), value)
    })
}
