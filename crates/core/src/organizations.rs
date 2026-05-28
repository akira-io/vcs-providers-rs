use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    BoxFuture, CognitionResult, ManagedOrganizationProvider, Page, PageRequest, ProviderId,
    Request, RequestHeader, Response, Transport, error, request, transport_not_configured,
};

pub trait Organizations: Send + Sync {
    fn list(&self) -> BoxFuture<'_, CognitionResult<Page<Organization>>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredOrganizations;

impl Organizations for TransportNotConfiguredOrganizations {
    fn list(&self) -> BoxFuture<'_, CognitionResult<Page<Organization>>> {
        transport_not_configured()
    }
}

pub trait OrganizationResponseMapper: Send + Sync {
    fn organizations(&self, response: &Response) -> CognitionResult<Page<Organization>>;
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OrganizationListQuery {
    page: Option<PageRequest>,
}

impl OrganizationListQuery {
    pub fn make(page: Option<PageRequest>) -> Self {
        Self { page }
    }

    pub fn page(&self) -> Option<&PageRequest> {
        self.page.as_ref()
    }
}

#[derive(Clone)]
pub struct TransportBackedOrganizations<Driver, Mapper> {
    driver: Driver,
    transport: Arc<dyn Transport>,
    mapper: Mapper,
    headers: Vec<RequestHeader>,
}

impl<Driver, Mapper> TransportBackedOrganizations<Driver, Mapper>
where
    Driver: ManagedOrganizationProvider,
    Mapper: OrganizationResponseMapper,
{
    pub fn make(driver: Driver, transport: Arc<dyn Transport>, mapper: Mapper) -> Self {
        Self {
            driver,
            transport,
            mapper,
            headers: Vec::new(),
        }
    }

    pub fn with_headers(mut self, headers: impl IntoIterator<Item = RequestHeader>) -> Self {
        self.headers.extend(headers);
        self
    }

    fn send_request<'a>(&'a self, request: Request) -> BoxFuture<'a, CognitionResult<Response>> {
        Box::pin(async move {
            let response = self.transport.send(self.apply_headers(request)).await?;

            if let Some(error) = error().from_response(&response) {
                return Err(error);
            }

            Ok(response)
        })
    }

    fn apply_headers(&self, request: Request) -> Request {
        self.headers
            .iter()
            .cloned()
            .fold(request, Request::with_header)
    }
}

impl<Driver, Mapper> Organizations for TransportBackedOrganizations<Driver, Mapper>
where
    Driver: ManagedOrganizationProvider + Send + Sync,
    Mapper: OrganizationResponseMapper,
{
    fn list(&self) -> BoxFuture<'_, CognitionResult<Page<Organization>>> {
        Box::pin(async move {
            let request = request()
                .get(self.driver.organization_list_url(None).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.organizations(&response)
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OrganizationKind {
    Personal,
    Organization,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    provider: ProviderId,
    id: String,
    login: String,
    kind: OrganizationKind,
}

impl Organization {
    pub fn make(
        provider: impl Into<String>,
        id: impl Into<String>,
        login: impl Into<String>,
        kind: OrganizationKind,
    ) -> Self {
        Self {
            provider: ProviderId::make(provider),
            id: id.into(),
            login: login.into(),
            kind,
        }
    }

    pub fn provider(&self) -> &ProviderId {
        &self.provider
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn login(&self) -> &str {
        &self.login
    }

    pub fn kind(&self) -> &OrganizationKind {
        &self.kind
    }
}
