use serde::{Deserialize, Serialize};

use crate::{AuthHeader, BoxFuture, RequestUrl, VcsResult};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequestHeaderName(String);

impl RequestHeaderName {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequestHeaderValue(String);

impl RequestHeaderValue {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequestHeader {
    name: RequestHeaderName,
    value: RequestHeaderValue,
}

impl RequestHeader {
    pub fn make(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: RequestHeaderName::make(name),
            value: RequestHeaderValue::make(value),
        }
    }

    pub fn name(&self) -> &RequestHeaderName {
        &self.name
    }

    pub fn value(&self) -> &RequestHeaderValue {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Request {
    method: RequestMethod,
    url: RequestUrl,
    headers: Vec<RequestHeader>,
    body: Option<RequestBody>,
}

impl Request {
    pub fn method(&self) -> &RequestMethod {
        &self.method
    }

    pub fn url(&self) -> &RequestUrl {
        &self.url
    }

    pub fn headers(&self) -> &[RequestHeader] {
        &self.headers
    }

    pub fn body(&self) -> Option<&RequestBody> {
        self.body.as_ref()
    }

    pub fn with_header(mut self, header: RequestHeader) -> Self {
        self.headers.push(header);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestBuilder {
    method: RequestMethod,
    url: RequestUrl,
    headers: Vec<RequestHeader>,
    body: Option<RequestBody>,
}

impl RequestBuilder {
    pub fn get(self, url: impl Into<String>) -> Self {
        self.with_method(RequestMethod::Get, url)
    }

    pub fn post(self, url: impl Into<String>) -> Self {
        self.with_method(RequestMethod::Post, url)
    }

    pub fn put(self, url: impl Into<String>) -> Self {
        self.with_method(RequestMethod::Put, url)
    }

    pub fn patch(self, url: impl Into<String>) -> Self {
        self.with_method(RequestMethod::Patch, url)
    }

    pub fn delete(self, url: impl Into<String>) -> Self {
        self.with_method(RequestMethod::Delete, url)
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push(RequestHeader::make(name, value));
        self
    }

    pub fn auth_header(mut self, auth_header: Option<AuthHeader>) -> Self {
        if let Some(header) = auth_header {
            self.headers.push(RequestHeader::make(
                header.name().as_str(),
                header.value().as_str(),
            ));
        }

        self
    }

    pub fn body(mut self, body: RequestBody) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Request {
        Request {
            method: self.method,
            url: self.url,
            headers: self.headers,
            body: self.body,
        }
    }

    fn with_method(mut self, method: RequestMethod, url: impl Into<String>) -> Self {
        self.method = method;
        self.url = RequestUrl::make(url);
        self
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self {
            method: RequestMethod::Get,
            url: RequestUrl::make(""),
            headers: Vec::new(),
            body: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequestBody {
    content: String,
}

impl RequestBody {
    pub fn make(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn as_str(&self) -> &str {
        &self.content
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResponseStatus(u16);

impl ResponseStatus {
    pub fn make(code: u16) -> Self {
        Self(code)
    }

    pub fn code(&self) -> u16 {
        self.0
    }

    pub fn is_success(&self) -> bool {
        (200..=299).contains(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: ResponseStatus,
    headers: Vec<RequestHeader>,
    body: Option<ResponseBody>,
}

impl Response {
    pub fn make(status: ResponseStatus, headers: Vec<RequestHeader>) -> Self {
        Self {
            status,
            headers,
            body: None,
        }
    }

    pub fn status(&self) -> &ResponseStatus {
        &self.status
    }

    pub fn headers(&self) -> &[RequestHeader] {
        &self.headers
    }

    pub fn body(&self) -> Option<&ResponseBody> {
        self.body.as_ref()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResponseBuilder {
    status: Option<ResponseStatus>,
    headers: Vec<RequestHeader>,
    body: Option<ResponseBody>,
}

impl ResponseBuilder {
    pub fn status(mut self, code: u16) -> Self {
        self.status = Some(ResponseStatus::make(code));
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push(RequestHeader::make(name, value));
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(ResponseBody::make(body));
        self
    }

    pub fn build(self) -> Response {
        Response {
            status: self.status.unwrap_or_else(|| ResponseStatus::make(200)),
            headers: self.headers,
            body: self.body,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResponseBody {
    content: String,
}

impl ResponseBody {
    pub fn make(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.content
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

pub trait Transport: Send + Sync {
    fn send(&self, request: Request) -> BoxFuture<'_, VcsResult<Response>>;
}
