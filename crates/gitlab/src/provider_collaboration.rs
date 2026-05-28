use git_cognition_core::{ManagedCodeReviewProvider, ManagedIssueProvider, ManagedReleaseProvider};

use crate::{
    GitLabCodeReview, GitLabCodeReviewCollection, GitLabIssue, GitLabIssueCollection,
    GitLabProvider, GitLabRelease, GitLabReleaseCollection,
};

impl ManagedIssueProvider for GitLabProvider {
    fn issue_url(&self, issue: &git_cognition_core::Issue) -> git_cognition_core::RequestUrl {
        GitLabIssue::make(self.api_base_url(), issue.clone()).url()
    }

    fn issue_list_url(
        &self,
        query: &git_cognition_core::IssueListQuery,
    ) -> git_cognition_core::RequestUrl {
        GitLabIssueCollection::make(self.api_base_url()).list(query)
    }

    fn issue_create_request(
        &self,
        draft: &git_cognition_core::IssueDraft,
    ) -> git_cognition_core::Request {
        GitLabIssueCollection::make(self.api_base_url()).create(draft)
    }

    fn issue_update_request(
        &self,
        patch: &git_cognition_core::IssuePatch,
    ) -> git_cognition_core::Request {
        GitLabIssue::make(self.api_base_url(), patch.issue().clone()).update(patch)
    }

    fn issue_delete_request(
        &self,
        issue: &git_cognition_core::Issue,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(GitLabIssue::make(self.api_base_url(), issue.clone()).delete())
    }
}

impl ManagedCodeReviewProvider for GitLabProvider {
    fn code_review_url(
        &self,
        code_review: &git_cognition_core::CodeReview,
    ) -> git_cognition_core::RequestUrl {
        GitLabCodeReview::make(self.api_base_url(), code_review.clone()).url()
    }

    fn code_review_list_url(
        &self,
        query: &git_cognition_core::CodeReviewListQuery,
    ) -> git_cognition_core::RequestUrl {
        GitLabCodeReviewCollection::make(self.api_base_url()).list(query)
    }

    fn code_review_create_request(
        &self,
        draft: &git_cognition_core::CodeReviewDraft,
    ) -> git_cognition_core::Request {
        GitLabCodeReviewCollection::make(self.api_base_url()).create(draft)
    }

    fn code_review_update_request(
        &self,
        patch: &git_cognition_core::CodeReviewPatch,
    ) -> git_cognition_core::Request {
        GitLabCodeReview::make(self.api_base_url(), patch.code_review().clone()).update(patch)
    }

    fn code_review_merge_request(
        &self,
        code_review: &git_cognition_core::CodeReview,
    ) -> git_cognition_core::Request {
        GitLabCodeReview::make(self.api_base_url(), code_review.clone()).merge()
    }

    fn code_review_close_request(
        &self,
        code_review: &git_cognition_core::CodeReview,
    ) -> git_cognition_core::Request {
        let close_patch = code_review.patch().closed().get();

        GitLabCodeReview::make(self.api_base_url(), code_review.clone()).update(&close_patch)
    }

    fn code_review_delete_request(
        &self,
        code_review: &git_cognition_core::CodeReview,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(GitLabCodeReview::make(self.api_base_url(), code_review.clone()).delete())
    }
}

impl ManagedReleaseProvider for GitLabProvider {
    fn release_url(&self, release: &git_cognition_core::Release) -> git_cognition_core::RequestUrl {
        GitLabRelease::make(self.api_base_url(), release.clone()).url()
    }

    fn release_list_url(
        &self,
        query: &git_cognition_core::ReleaseListQuery,
    ) -> git_cognition_core::RequestUrl {
        GitLabReleaseCollection::make(self.api_base_url()).list(query)
    }

    fn release_create_request(
        &self,
        draft: &git_cognition_core::ReleaseDraft,
    ) -> git_cognition_core::Request {
        GitLabReleaseCollection::make(self.api_base_url()).create(draft)
    }

    fn release_update_request(
        &self,
        patch: &git_cognition_core::ReleasePatch,
    ) -> git_cognition_core::Request {
        GitLabRelease::make(self.api_base_url(), patch.release().clone()).update(patch)
    }

    fn release_delete_request(
        &self,
        release: &git_cognition_core::Release,
    ) -> git_cognition_core::Request {
        GitLabRelease::make(self.api_base_url(), release.clone()).delete()
    }
}
