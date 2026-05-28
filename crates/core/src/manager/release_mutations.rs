use crate::{ManagedRelease, ManagedReleaseBuilder, ProvidedReleaseId, ProvidedReleaseRepo};

use super::ManagedReleaseProvider;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedReleaseUpdateBuilder<Driver> {
    managed_release: ManagedRelease<Driver>,
    name: Option<String>,
    body: Option<String>,
}

impl<Driver> ManagedReleaseBuilder<Driver, ProvidedReleaseRepo, ProvidedReleaseId>
where
    Driver: ManagedReleaseProvider,
{
    pub fn name(self, name: impl Into<String>) -> ManagedReleaseUpdateBuilder<Driver> {
        self.get().name(name)
    }

    pub fn body(self, body: impl Into<String>) -> ManagedReleaseUpdateBuilder<Driver> {
        self.get().body(body)
    }
}

impl<Driver> ManagedRelease<Driver>
where
    Driver: ManagedReleaseProvider,
{
    pub fn name(self, name: impl Into<String>) -> ManagedReleaseUpdateBuilder<Driver> {
        ManagedReleaseUpdateBuilder::make(self).name(name)
    }

    pub fn body(self, body: impl Into<String>) -> ManagedReleaseUpdateBuilder<Driver> {
        ManagedReleaseUpdateBuilder::make(self).body(body)
    }
}

impl<Driver> ManagedReleaseUpdateBuilder<Driver>
where
    Driver: ManagedReleaseProvider,
{
    pub fn make(managed_release: ManagedRelease<Driver>) -> Self {
        Self {
            managed_release,
            name: None,
            body: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn update(self) -> crate::Request {
        let patch = self.patch();
        self.managed_release
            .manager
            .driver
            .release_update_request(&patch)
    }

    fn patch(&self) -> crate::ReleasePatch {
        let mut patch = self.managed_release.release.patch();

        if let Some(name) = self.name.clone() {
            patch = patch.name(name);
        }

        if let Some(body) = self.body.clone() {
            patch = patch.body(body);
        }

        patch.get()
    }
}
