use crate::{ManagedProvider, ManagedRepo, Request, Visibility};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepositoryUpdateBuilder<Driver> {
    managed_repo: ManagedRepo<Driver>,
    visibility: Option<Visibility>,
    description: Option<String>,
}

impl<Driver> ManagedRepo<Driver>
where
    Driver: ManagedProvider,
{
    pub fn visibility(self, visibility: Visibility) -> ManagedRepositoryUpdateBuilder<Driver> {
        ManagedRepositoryUpdateBuilder::make(self).visibility(visibility)
    }

    pub fn description(
        self,
        description: impl Into<String>,
    ) -> ManagedRepositoryUpdateBuilder<Driver> {
        ManagedRepositoryUpdateBuilder::make(self).description(description)
    }
}

impl<Driver> ManagedRepositoryUpdateBuilder<Driver>
where
    Driver: ManagedProvider,
{
    pub fn make(managed_repo: ManagedRepo<Driver>) -> Self {
        Self {
            managed_repo,
            visibility: None,
            description: None,
        }
    }

    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn update(self) -> Request {
        let patch = self.patch();
        self.managed_repo.manager.driver.repo_update_request(&patch)
    }

    fn patch(&self) -> crate::RepositoryPatch {
        let mut patch = self.managed_repo.repo.patch();

        if let Some(visibility) = self.visibility.clone() {
            patch = patch.visibility(visibility);
        }

        if let Some(description) = self.description.clone() {
            patch = patch.description(description);
        }

        patch.get()
    }
}
