use vcs_provider_core::{ManagedCodeReviewProvider, ManagedIssueProvider, ManagedReleaseProvider};

use crate::{
    GitLabCodeReview, GitLabCodeReviewCollection, GitLabIssue, GitLabIssueCollection,
    GitLabProvider, GitLabRelease, GitLabReleaseCollection,
};

impl ManagedIssueProvider for GitLabProvider {
    fn issue_url(&self, issue: &vcs_provider_core::Issue) -> vcs_provider_core::RequestUrl {
        GitLabIssue::make(self.api_base_url(), issue.clone()).url()
    }

    fn issue_list_url(
        &self,
        query: &vcs_provider_core::IssueListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitLabIssueCollection::make(self.api_base_url()).list(query)
    }

    fn issue_create_request(
        &self,
        draft: &vcs_provider_core::IssueDraft,
    ) -> vcs_provider_core::Request {
        GitLabIssueCollection::make(self.api_base_url()).create(draft)
    }

    fn issue_update_request(
        &self,
        patch: &vcs_provider_core::IssuePatch,
    ) -> vcs_provider_core::Request {
        GitLabIssue::make(self.api_base_url(), patch.issue().clone()).update(patch)
    }

    fn issue_delete_request(
        &self,
        issue: &vcs_provider_core::Issue,
    ) -> vcs_provider_core::VcsResult<vcs_provider_core::Request> {
        Ok(GitLabIssue::make(self.api_base_url(), issue.clone()).delete())
    }
}

impl ManagedCodeReviewProvider for GitLabProvider {
    fn code_review_url(
        &self,
        code_review: &vcs_provider_core::CodeReview,
    ) -> vcs_provider_core::RequestUrl {
        GitLabCodeReview::make(self.api_base_url(), code_review.clone()).url()
    }

    fn code_review_list_url(
        &self,
        query: &vcs_provider_core::CodeReviewListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitLabCodeReviewCollection::make(self.api_base_url()).list(query)
    }

    fn code_review_create_request(
        &self,
        draft: &vcs_provider_core::CodeReviewDraft,
    ) -> vcs_provider_core::Request {
        GitLabCodeReviewCollection::make(self.api_base_url()).create(draft)
    }

    fn code_review_update_request(
        &self,
        patch: &vcs_provider_core::CodeReviewPatch,
    ) -> vcs_provider_core::Request {
        GitLabCodeReview::make(self.api_base_url(), patch.code_review().clone()).update(patch)
    }

    fn code_review_merge_request(
        &self,
        code_review: &vcs_provider_core::CodeReview,
    ) -> vcs_provider_core::Request {
        GitLabCodeReview::make(self.api_base_url(), code_review.clone()).merge()
    }

    fn code_review_close_request(
        &self,
        code_review: &vcs_provider_core::CodeReview,
    ) -> vcs_provider_core::Request {
        let close_patch = code_review.patch().closed().get();

        GitLabCodeReview::make(self.api_base_url(), code_review.clone()).update(&close_patch)
    }

    fn code_review_delete_request(
        &self,
        code_review: &vcs_provider_core::CodeReview,
    ) -> vcs_provider_core::VcsResult<vcs_provider_core::Request> {
        Ok(GitLabCodeReview::make(self.api_base_url(), code_review.clone()).delete())
    }
}

impl ManagedReleaseProvider for GitLabProvider {
    fn release_url(&self, release: &vcs_provider_core::Release) -> vcs_provider_core::RequestUrl {
        GitLabRelease::make(self.api_base_url(), release.clone()).url()
    }

    fn release_list_url(
        &self,
        query: &vcs_provider_core::ReleaseListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitLabReleaseCollection::make(self.api_base_url()).list(query)
    }

    fn release_create_request(
        &self,
        draft: &vcs_provider_core::ReleaseDraft,
    ) -> vcs_provider_core::Request {
        GitLabReleaseCollection::make(self.api_base_url()).create(draft)
    }

    fn release_update_request(
        &self,
        patch: &vcs_provider_core::ReleasePatch,
    ) -> vcs_provider_core::Request {
        GitLabRelease::make(self.api_base_url(), patch.release().clone()).update(patch)
    }

    fn release_delete_request(
        &self,
        release: &vcs_provider_core::Release,
    ) -> vcs_provider_core::Request {
        GitLabRelease::make(self.api_base_url(), release.clone()).delete()
    }
}
