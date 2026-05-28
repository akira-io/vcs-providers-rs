use serde::{Deserialize, Serialize};

use crate::{BoxFuture, Response, ResponseStatus};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CognitionError {
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    RateLimited,
    ProviderUnavailable,
    TransportNotConfigured,
    UnsupportedOperation(String),
    ProviderAlreadyRegistered(String),
    ProviderNotRegistered(String),
    InvalidInput(String),
}

impl CognitionError {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::Unauthorized => ErrorKind::Unauthorized,
            Self::Forbidden => ErrorKind::Forbidden,
            Self::NotFound => ErrorKind::NotFound,
            Self::Conflict => ErrorKind::Conflict,
            Self::RateLimited => ErrorKind::RateLimited,
            Self::ProviderUnavailable => ErrorKind::ProviderUnavailable,
            Self::TransportNotConfigured => ErrorKind::TransportNotConfigured,
            Self::UnsupportedOperation(_) => ErrorKind::UnsupportedOperation,
            Self::ProviderAlreadyRegistered(_) => ErrorKind::ProviderAlreadyRegistered,
            Self::ProviderNotRegistered(_) => ErrorKind::ProviderNotRegistered,
            Self::InvalidInput(_) => ErrorKind::InvalidInput,
        }
    }
}

pub type CognitionResult<T> = Result<T, CognitionError>;

pub(crate) fn transport_not_configured<'a, T>() -> BoxFuture<'a, CognitionResult<T>> {
    Box::pin(async { Err(CognitionError::TransportNotConfigured) })
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
    UnsupportedOperation,
    ProviderAlreadyRegistered,
    ProviderNotRegistered,
    InvalidInput,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ErrorBuilder;

impl ErrorBuilder {
    pub fn from_response(self, response: &Response) -> Option<CognitionError> {
        self.from_status(response.status())
    }

    pub fn from_status(self, status: &ResponseStatus) -> Option<CognitionError> {
        match status.code() {
            400 => Some(CognitionError::InvalidInput("bad request".into())),
            401 => Some(CognitionError::Unauthorized),
            403 => Some(CognitionError::Forbidden),
            404 => Some(CognitionError::NotFound),
            409 => Some(CognitionError::Conflict),
            429 => Some(CognitionError::RateLimited),
            500..=599 => Some(CognitionError::ProviderUnavailable),
            _ => None,
        }
    }

    pub fn invalid_input(self, message: impl Into<String>) -> CognitionError {
        CognitionError::InvalidInput(message.into())
    }

    pub fn unsupported_operation(self, operation: impl Into<String>) -> CognitionError {
        CognitionError::UnsupportedOperation(operation.into())
    }

    pub fn provider_already_registered(self, provider: impl Into<String>) -> CognitionError {
        CognitionError::ProviderAlreadyRegistered(provider.into())
    }

    pub fn provider_not_registered(self, provider: impl Into<String>) -> CognitionError {
        CognitionError::ProviderNotRegistered(provider.into())
    }
}
