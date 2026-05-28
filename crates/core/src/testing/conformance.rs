use crate::{
    AuthHeaderStyle, AuthKind, Capability, CognitionError, CognitionResult, Provider, ProviderId,
    error, provider,
};

#[path = "conformance/contracts.rs"]
mod contracts;

use contracts::check_provider_contracts;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderConformanceBuilder;

impl ProviderConformanceBuilder {
    pub fn provider<ProviderUnderTest>(
        self,
        provider: ProviderUnderTest,
    ) -> ProviderConformance<ProviderUnderTest>
    where
        ProviderUnderTest: Provider + Clone + 'static,
    {
        ProviderConformance {
            provider,
            expected_id: None,
            expected_display_name: None,
            supported_capabilities: Vec::new(),
            unsupported_capabilities: Vec::new(),
            auth_expectations: vec![(AuthKind::Anonymous, AuthHeaderStyle::None)],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderConformance<ProviderUnderTest> {
    provider: ProviderUnderTest,
    expected_id: Option<String>,
    expected_display_name: Option<String>,
    supported_capabilities: Vec<Capability>,
    unsupported_capabilities: Vec<Capability>,
    auth_expectations: Vec<(AuthKind, AuthHeaderStyle)>,
}

impl<ProviderUnderTest> ProviderConformance<ProviderUnderTest>
where
    ProviderUnderTest: Provider + Clone + 'static,
{
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.expected_id = Some(id.into());
        self
    }

    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.expected_display_name = Some(display_name.into());
        self
    }

    pub fn supports(mut self, capabilities: impl IntoIterator<Item = Capability>) -> Self {
        self.supported_capabilities.extend(capabilities);
        self
    }

    pub fn does_not_support(mut self, capabilities: impl IntoIterator<Item = Capability>) -> Self {
        self.unsupported_capabilities.extend(capabilities);
        self
    }

    pub fn auth(mut self, auth_kind: AuthKind, auth_header_style: AuthHeaderStyle) -> Self {
        self.auth_expectations.push((auth_kind, auth_header_style));
        self
    }

    pub fn check(self) -> CognitionResult<()> {
        self.check_descriptor()?;
        self.check_capabilities()?;
        self.check_auth()?;
        self.check_contracts()?;
        self.check_registry()?;

        Ok(())
    }

    fn check_descriptor(&self) -> CognitionResult<()> {
        let descriptor = self.provider.descriptor();

        if let Some(expected_id) = self.expected_id.as_deref() {
            assert_same("provider id", expected_id, descriptor.id().as_str())?;
        }

        if let Some(expected_display_name) = self.expected_display_name.as_deref() {
            assert_same(
                "provider display name",
                expected_display_name,
                descriptor.display_name(),
            )?;
        }

        Ok(())
    }

    fn check_capabilities(&self) -> CognitionResult<()> {
        let capabilities = self.provider.capabilities();

        for capability in Capability::all() {
            let expected_supported = self.supported_capabilities.contains(capability);
            let expected_unsupported = self.unsupported_capabilities.contains(capability);

            if expected_supported && expected_unsupported {
                return Err(error().invalid_input("capability expectation conflicts"));
            }

            if !expected_supported && !expected_unsupported {
                return Err(error().invalid_input("capability expectation missing"));
            }
        }

        for capability in &self.supported_capabilities {
            if !capabilities.supports(capability) {
                return Err(error().invalid_input("supported capability is missing"));
            }
        }

        for capability in &self.unsupported_capabilities {
            if capabilities.supports(capability) {
                return Err(error().invalid_input("unsupported capability is exposed"));
            }
        }

        Ok(())
    }

    fn check_auth(&self) -> CognitionResult<()> {
        for (auth_kind, expected_auth_header_style) in &self.auth_expectations {
            let actual_auth_header_style = self.provider.auth_header_style(*auth_kind);

            if &actual_auth_header_style != expected_auth_header_style {
                return Err(error().invalid_input("auth header style does not match"));
            }
        }

        Ok(())
    }

    fn check_contracts(&self) -> CognitionResult<()> {
        check_provider_contracts(&self.provider)
    }

    fn check_registry(&self) -> CognitionResult<()> {
        let descriptor = self.provider.descriptor();
        let id = self
            .expected_id
            .clone()
            .unwrap_or_else(|| descriptor.id().as_str().to_owned());
        let registry = provider().register(self.provider.clone())?.build();

        if !registry.contains_provider(&ProviderId::make(&id)) {
            return Err(error().invalid_input("provider registry did not contain provider"));
        }

        if let Some(capability) = self.supported_capabilities.first().cloned() {
            let providers = registry
                .providers_supporting(capability)
                .collect::<Vec<_>>();

            if providers.len() != 1 {
                return Err(error().invalid_input("provider registry capability filter failed"));
            }
        }

        let duplicate_result = provider()
            .register(self.provider.clone())?
            .register(self.provider.clone());

        match duplicate_result {
            Err(CognitionError::ProviderAlreadyRegistered(duplicate_id)) if duplicate_id == id => {
                Ok(())
            }
            Err(_) => {
                Err(error().invalid_input("provider registry returned wrong duplicate error"))
            }
            Ok(_) => Err(error().invalid_input("provider registry accepted duplicate id")),
        }
    }
}

fn assert_same(label: &str, expected: &str, actual: &str) -> CognitionResult<()> {
    if expected == actual {
        return Ok(());
    }

    Err(error().invalid_input(format!("{label} does not match")))
}
