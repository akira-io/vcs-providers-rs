use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, CodeReviews, Issues, ManagedIssueProvider,
    ManagedProvider, MissingOwnerName, MissingRepositoryName, Pipelines, Provider,
    ProviderDescriptor, ProviderId, Releases, Repos, TransportNotConfiguredCodeReviews,
    TransportNotConfiguredIssues, TransportNotConfiguredPipelines, TransportNotConfiguredReleases,
    TransportNotConfiguredRepos, capabilities,
};

mod issues;
mod repos;

pub use issues::{GitHubIssue, GitHubIssueCollection};
pub use repos::{GitHubRepo, GitHubRepoCollection};

pub const PROVIDER_ID: &str = "github";
pub const DISPLAY_NAME: &str = "GitHub";
pub const DEFAULT_BASE_URL: &str = "https://api.github.com";

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubProvider;

impl GitHubProvider {
    pub fn repo(
        &self,
    ) -> vcs_provider_core::ManagedRepoBuilder<Self, MissingOwnerName, MissingRepositoryName> {
        vcs_provider_core::vcs(*self).repo()
    }

    pub fn issue(
        &self,
    ) -> vcs_provider_core::ManagedIssueBuilder<
        Self,
        vcs_provider_core::MissingIssueRepo,
        vcs_provider_core::MissingIssueId,
    > {
        vcs_provider_core::vcs(*self).issue()
    }

    pub fn pagination(&self) -> vcs_provider_core::PaginationBuilder {
        vcs_provider_core::pagination()
    }
}

impl ManagedProvider for GitHubProvider {
    fn repo_url(&self, repo: &vcs_provider_core::Repo) -> vcs_provider_core::RequestUrl {
        GitHubRepo::make(DEFAULT_BASE_URL, repo.clone()).url()
    }

    fn repo_branches_url(
        &self,
        repo: &vcs_provider_core::Repo,
        page: Option<&vcs_provider_core::PageRequest>,
    ) -> vcs_provider_core::RequestUrl {
        GitHubRepo::make(DEFAULT_BASE_URL, repo.clone()).branches(page)
    }

    fn repo_commits_url(
        &self,
        repo: &vcs_provider_core::Repo,
        page: Option<&vcs_provider_core::PageRequest>,
    ) -> vcs_provider_core::RequestUrl {
        GitHubRepo::make(DEFAULT_BASE_URL, repo.clone()).commits(page)
    }

    fn repo_list_url(
        &self,
        query: &vcs_provider_core::RepositoryListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitHubRepoCollection::make(DEFAULT_BASE_URL).list(query)
    }

    fn repo_search_url(
        &self,
        query: &vcs_provider_core::RepositorySearchQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitHubRepoCollection::make(DEFAULT_BASE_URL).search(query)
    }
}

impl ManagedIssueProvider for GitHubProvider {
    fn issue_url(&self, issue: &vcs_provider_core::Issue) -> vcs_provider_core::RequestUrl {
        GitHubIssue::make(DEFAULT_BASE_URL, issue.clone()).url()
    }

    fn issue_list_url(
        &self,
        query: &vcs_provider_core::IssueListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitHubIssueCollection::make(DEFAULT_BASE_URL).list(query)
    }
}

impl Provider for GitHubProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor::make(
            ProviderId::make(PROVIDER_ID),
            DISPLAY_NAME,
            capabilities().make([
                Capability::Repos,
                Capability::Issues,
                Capability::CodeReviews,
                Capability::Pipelines,
                Capability::Releases,
                Capability::Organizations,
                Capability::Discussions,
                Capability::Webhooks,
            ]),
        )
    }

    fn repos(&self) -> Box<dyn Repos> {
        Box::<TransportNotConfiguredRepos>::default()
    }

    fn issues(&self) -> Box<dyn Issues> {
        Box::<TransportNotConfiguredIssues>::default()
    }

    fn code_reviews(&self) -> Box<dyn CodeReviews> {
        Box::<TransportNotConfiguredCodeReviews>::default()
    }

    fn pipelines(&self) -> Box<dyn Pipelines> {
        Box::<TransportNotConfiguredPipelines>::default()
    }

    fn releases(&self) -> Box<dyn Releases> {
        Box::<TransportNotConfiguredReleases>::default()
    }

    fn default_base_url(&self) -> &str {
        DEFAULT_BASE_URL
    }

    fn auth_header_style(&self, auth_kind: AuthKind) -> AuthHeaderStyle {
        match auth_kind {
            AuthKind::Anonymous => AuthHeaderStyle::None,
            AuthKind::PersonalAccessToken => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::OAuth => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::AppInstallation => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::Jwt => AuthHeaderStyle::AuthorizationBearer,
        }
    }
}

pub fn github() -> GitHubProvider {
    GitHubProvider
}
