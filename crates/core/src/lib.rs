use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

mod registry;

pub use registry::{ProviderRegistry, ProviderRegistryBuilder};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ProviderId(String);

impl ProviderId {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderDescriptor {
    id: ProviderId,
    display_name: String,
    capabilities: CapabilitySet,
}

impl ProviderDescriptor {
    pub fn make(
        id: ProviderId,
        display_name: impl Into<String>,
        capabilities: CapabilitySet,
    ) -> Self {
        Self {
            id,
            display_name: display_name.into(),
            capabilities,
        }
    }

    pub fn id(&self) -> &ProviderId {
        &self.id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn capabilities(&self) -> &CapabilitySet {
        &self.capabilities
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Capability {
    Repositories,
    Issues,
    CodeReviews,
    Pipelines,
    Releases,
    Organizations,
    Discussions,
    Webhooks,
    SelfHosted,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapabilitySet {
    capabilities: BTreeSet<Capability>,
}

impl CapabilitySet {
    pub fn make(capabilities: impl IntoIterator<Item = Capability>) -> Self {
        Self {
            capabilities: capabilities.into_iter().collect(),
        }
    }

    pub fn supports(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Capability> {
        self.capabilities.iter()
    }
}

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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum VcsError {
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    RateLimited,
    ProviderUnavailable,
    ProviderAlreadyRegistered(String),
    ProviderNotRegistered(String),
    InvalidInput(String),
}

pub type VcsResult<T> = Result<T, VcsError>;

pub trait ProviderDriver: Send + Sync {
    fn descriptor(&self) -> ProviderDescriptor;

    fn default_base_url(&self) -> &str;

    fn auth_header_style(&self, auth_kind: AuthKind) -> AuthHeaderStyle;
}

#[cfg(test)]
mod tests {
    use super::{Capability, CapabilitySet};

    #[test]
    fn capability_set_reports_supported_capabilities() {
        let capabilities = CapabilitySet::make([Capability::Repositories, Capability::Pipelines]);

        assert!(capabilities.supports(&Capability::Repositories));
        assert!(capabilities.supports(&Capability::Pipelines));
        assert!(!capabilities.supports(&Capability::Releases));
    }
}
