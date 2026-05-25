use std::sync::Arc;

use crate::{
    AuthHeaderStyle, AuthKind, Capability, Provider, ProviderDescriptor, TelemetrySink, Transport,
    capabilities, provider,
};

use super::{
    IntoProvider, MissingProviderTransport, ProvidedProviderTransport,
    ProviderRuntimeWithProviderBuilder,
};

#[derive(Clone)]
pub struct RuntimeProviderConfigurationBuilder {
    base_url: String,
    token_auth_header_style: AuthHeaderStyle,
    telemetry_sink: Option<Arc<dyn TelemetrySink>>,
}

impl RuntimeProviderConfigurationBuilder {
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn bearer_auth(mut self) -> Self {
        self.token_auth_header_style = AuthHeaderStyle::AuthorizationBearer;
        self
    }

    pub fn telemetry(mut self, sink: impl TelemetrySink + 'static) -> Self {
        self.telemetry_sink = Some(Arc::new(sink));
        self
    }

    pub fn from(
        self,
        provider: impl IntoProvider,
    ) -> ProviderRuntimeWithProviderBuilder<MissingProviderTransport> {
        ProviderRuntimeWithProviderBuilder {
            provider: Arc::new(provider.into_provider()),
            transport: MissingProviderTransport,
            telemetry_sink: self.telemetry_sink,
        }
    }

    pub fn transport(
        self,
        transport: impl Transport + 'static,
    ) -> ProviderRuntimeWithProviderBuilder<ProvidedProviderTransport> {
        let telemetry_sink = self.telemetry_sink.clone();

        ProviderRuntimeWithProviderBuilder {
            provider: Arc::new(self.into_provider()),
            transport: MissingProviderTransport,
            telemetry_sink,
        }
        .transport(transport)
    }
}

impl Default for RuntimeProviderConfigurationBuilder {
    fn default() -> Self {
        Self {
            base_url: "https://api.example.test".into(),
            token_auth_header_style: AuthHeaderStyle::AuthorizationBearer,
            telemetry_sink: None,
        }
    }
}

impl IntoProvider for RuntimeProviderConfigurationBuilder {
    type Provider = RuntimeConfiguredProvider;

    fn into_provider(self) -> Self::Provider {
        RuntimeConfiguredProvider {
            base_url: self.base_url,
            token_auth_header_style: self.token_auth_header_style,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeConfiguredProvider {
    base_url: String,
    token_auth_header_style: AuthHeaderStyle,
}

impl Provider for RuntimeConfiguredProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        provider()
            .descriptor("runtime")
            .display_name("Runtime")
            .capabilities(capabilities().make([Capability::Repos]))
            .build()
    }

    fn repos(&self) -> Box<dyn crate::Repos> {
        Box::new(crate::TransportNotConfiguredRepos)
    }

    fn issues(&self) -> Box<dyn crate::Issues> {
        Box::new(crate::TransportNotConfiguredIssues)
    }

    fn code_reviews(&self) -> Box<dyn crate::CodeReviews> {
        Box::new(crate::TransportNotConfiguredCodeReviews)
    }

    fn pipelines(&self) -> Box<dyn crate::Pipelines> {
        Box::new(crate::TransportNotConfiguredPipelines)
    }

    fn releases(&self) -> Box<dyn crate::Releases> {
        Box::new(crate::TransportNotConfiguredReleases)
    }

    fn default_base_url(&self) -> &str {
        &self.base_url
    }

    fn auth_header_style(&self, auth_kind: AuthKind) -> AuthHeaderStyle {
        match auth_kind {
            AuthKind::Anonymous => AuthHeaderStyle::None,
            AuthKind::PersonalAccessToken => self.token_auth_header_style.clone(),
            AuthKind::OAuth => self.token_auth_header_style.clone(),
            AuthKind::AppInstallation => self.token_auth_header_style.clone(),
            AuthKind::Jwt => self.token_auth_header_style.clone(),
        }
    }
}
