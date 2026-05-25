use serde::{Deserialize, Serialize};

use crate::{AuthHeader, BoxFuture, VcsResult};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequestUrl(String);

impl RequestUrl {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestBuilder {
    method: RequestMethod,
    url: RequestUrl,
    headers: Vec<RequestHeader>,
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

    pub fn build(self) -> Request {
        Request {
            method: self.method,
            url: self.url,
            headers: self.headers,
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
        }
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
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: ResponseStatus,
    headers: Vec<RequestHeader>,
}

impl Response {
    pub fn make(status: ResponseStatus, headers: Vec<RequestHeader>) -> Self {
        Self { status, headers }
    }

    pub fn status(&self) -> &ResponseStatus {
        &self.status
    }

    pub fn headers(&self) -> &[RequestHeader] {
        &self.headers
    }
}

pub trait Transport: Send + Sync {
    fn send(&self, request: Request) -> BoxFuture<'_, VcsResult<Response>>;
}
