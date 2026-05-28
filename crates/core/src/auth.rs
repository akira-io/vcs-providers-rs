use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    BoxFuture, CognitionResult, ManagedAuthProvider, Request, RequestHeader, Transport, error,
    request, transport_not_configured,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AuthKind {
    Anonymous,
    PersonalAccessToken,
    OAuth,
    AppInstallation,
    Jwt,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AuthHeaderStyle {
    AuthorizationBearer,
    AuthorizationToken,
    CustomHeader(String),
    None,
}

#[derive(Clone, Eq, PartialEq)]
pub struct AuthToken(String);

impl AuthToken {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AuthToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AuthToken(**redacted**)")
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum AuthCredential {
    Anonymous,
    PersonalAccessToken(AuthToken),
    OAuth(AuthToken),
    AppInstallation(AuthToken),
    Jwt(AuthToken),
}

impl AuthCredential {
    pub fn kind(&self) -> AuthKind {
        match self {
            Self::Anonymous => AuthKind::Anonymous,
            Self::PersonalAccessToken(_) => AuthKind::PersonalAccessToken,
            Self::OAuth(_) => AuthKind::OAuth,
            Self::AppInstallation(_) => AuthKind::AppInstallation,
            Self::Jwt(_) => AuthKind::Jwt,
        }
    }

    pub fn header(&self, style: AuthHeaderStyle) -> Option<AuthHeader> {
        match self {
            Self::Anonymous => None,
            Self::PersonalAccessToken(token)
            | Self::OAuth(token)
            | Self::AppInstallation(token)
            | Self::Jwt(token) => AuthHeader::from_token(token, style),
        }
    }
}

impl fmt::Debug for AuthCredential {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Anonymous => formatter.write_str("AuthCredential::Anonymous"),
            Self::PersonalAccessToken(_) => {
                formatter.write_str("AuthCredential::PersonalAccessToken(**redacted**)")
            }
            Self::OAuth(_) => formatter.write_str("AuthCredential::OAuth(**redacted**)"),
            Self::AppInstallation(_) => {
                formatter.write_str("AuthCredential::AppInstallation(**redacted**)")
            }
            Self::Jwt(_) => formatter.write_str("AuthCredential::Jwt(**redacted**)"),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AuthBuilder;

impl AuthBuilder {
    pub fn anonymous(self) -> AuthCredential {
        AuthCredential::Anonymous
    }

    pub fn personal_access_token(self, token: impl Into<String>) -> AuthCredential {
        AuthCredential::PersonalAccessToken(AuthToken::make(token))
    }

    pub fn oauth(self, token: impl Into<String>) -> AuthCredential {
        AuthCredential::OAuth(AuthToken::make(token))
    }

    pub fn app_installation(self, token: impl Into<String>) -> AuthCredential {
        AuthCredential::AppInstallation(AuthToken::make(token))
    }

    pub fn jwt(self, token: impl Into<String>) -> AuthCredential {
        AuthCredential::Jwt(AuthToken::make(token))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthHeader {
    name: AuthHeaderName,
    value: AuthHeaderValue,
}

impl AuthHeader {
    fn from_token(token: &AuthToken, style: AuthHeaderStyle) -> Option<Self> {
        match style {
            AuthHeaderStyle::AuthorizationBearer => Some(Self {
                name: AuthHeaderName::make("authorization"),
                value: AuthHeaderValue::make(format!("Bearer {}", token.as_str())),
            }),
            AuthHeaderStyle::AuthorizationToken => Some(Self {
                name: AuthHeaderName::make("authorization"),
                value: AuthHeaderValue::make(format!("token {}", token.as_str())),
            }),
            AuthHeaderStyle::CustomHeader(name) => Some(Self {
                name: AuthHeaderName::make(name),
                value: AuthHeaderValue::make(token.as_str()),
            }),
            AuthHeaderStyle::None => None,
        }
    }

    pub fn name(&self) -> &AuthHeaderName {
        &self.name
    }

    pub fn value(&self) -> &AuthHeaderValue {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthHeaderName(String);

impl AuthHeaderName {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct AuthHeaderValue(String);

impl AuthHeaderValue {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AuthHeaderValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AuthHeaderValue(**redacted**)")
    }
}

pub trait Authentication: Send + Sync {
    fn validate(&self) -> BoxFuture<'_, CognitionResult<()>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredAuthentication;

impl Authentication for TransportNotConfiguredAuthentication {
    fn validate(&self) -> BoxFuture<'_, CognitionResult<()>> {
        transport_not_configured()
    }
}

#[derive(Clone)]
pub struct TransportBackedAuthentication<Driver> {
    driver: Driver,
    transport: Arc<dyn Transport>,
    headers: Vec<RequestHeader>,
}

impl<Driver> TransportBackedAuthentication<Driver>
where
    Driver: ManagedAuthProvider,
{
    pub fn make(driver: Driver, transport: Arc<dyn Transport>) -> Self {
        Self {
            driver,
            transport,
            headers: Vec::new(),
        }
    }

    pub fn with_headers(mut self, headers: impl IntoIterator<Item = RequestHeader>) -> Self {
        self.headers.extend(headers);
        self
    }

    fn apply_headers(&self, request: Request) -> Request {
        self.headers
            .iter()
            .cloned()
            .fold(request, Request::with_header)
    }
}

impl<Driver> Authentication for TransportBackedAuthentication<Driver>
where
    Driver: ManagedAuthProvider + Send + Sync,
{
    fn validate(&self) -> BoxFuture<'_, CognitionResult<()>> {
        Box::pin(async move {
            let request = request()
                .get(self.driver.auth_validate_url().value())
                .build();
            let response = self.transport.send(self.apply_headers(request)).await?;

            if let Some(error) = error().from_response(&response) {
                return Err(error);
            }

            Ok(())
        })
    }
}
