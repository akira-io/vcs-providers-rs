use serde::{Deserialize, Serialize};

use crate::{BoxFuture, Response, ResponseStatus};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum VcsError {
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    RateLimited,
    ProviderUnavailable,
    TransportNotConfigured,
    ProviderAlreadyRegistered(String),
    ProviderNotRegistered(String),
    InvalidInput(String),
}

impl VcsError {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::Unauthorized => ErrorKind::Unauthorized,
            Self::Forbidden => ErrorKind::Forbidden,
            Self::NotFound => ErrorKind::NotFound,
            Self::Conflict => ErrorKind::Conflict,
            Self::RateLimited => ErrorKind::RateLimited,
            Self::ProviderUnavailable => ErrorKind::ProviderUnavailable,
            Self::TransportNotConfigured => ErrorKind::TransportNotConfigured,
            Self::ProviderAlreadyRegistered(_) => ErrorKind::ProviderAlreadyRegistered,
            Self::ProviderNotRegistered(_) => ErrorKind::ProviderNotRegistered,
            Self::InvalidInput(_) => ErrorKind::InvalidInput,
        }
    }
}

pub type VcsResult<T> = Result<T, VcsError>;

pub(crate) fn transport_not_configured<'a, T>() -> BoxFuture<'a, VcsResult<T>> {
    Box::pin(async { Err(VcsError::TransportNotConfigured) })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ErrorKind {
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    RateLimited,
    ProviderUnavailable,
    TransportNotConfigured,
    ProviderAlreadyRegistered,
    ProviderNotRegistered,
    InvalidInput,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ErrorBuilder;

impl ErrorBuilder {
    pub fn from_response(self, response: &Response) -> Option<VcsError> {
        self.from_status(response.status())
    }

    pub fn from_status(self, status: &ResponseStatus) -> Option<VcsError> {
        match status.code() {
            400 => Some(VcsError::InvalidInput("bad request".into())),
            401 => Some(VcsError::Unauthorized),
            403 => Some(VcsError::Forbidden),
            404 => Some(VcsError::NotFound),
            409 => Some(VcsError::Conflict),
            429 => Some(VcsError::RateLimited),
            500..=599 => Some(VcsError::ProviderUnavailable),
            _ => None,
        }
    }

    pub fn invalid_input(self, message: impl Into<String>) -> VcsError {
        VcsError::InvalidInput(message.into())
    }

    pub fn provider_already_registered(self, provider: impl Into<String>) -> VcsError {
        VcsError::ProviderAlreadyRegistered(provider.into())
    }

    pub fn provider_not_registered(self, provider: impl Into<String>) -> VcsError {
        VcsError::ProviderNotRegistered(provider.into())
    }
}
