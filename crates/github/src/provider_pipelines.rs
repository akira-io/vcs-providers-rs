use git_cognition_core::ManagedPipelineProvider;

use crate::{GitHubPipeline, GitHubPipelineCollection, GitHubProvider};

impl ManagedPipelineProvider for GitHubProvider {
    fn pipeline_url(
        &self,
        pipeline: &git_cognition_core::Pipeline,
    ) -> git_cognition_core::RequestUrl {
        GitHubPipeline::make(self.api_base_url(), pipeline.clone()).url()
    }

    fn pipeline_list_url(
        &self,
        query: &git_cognition_core::PipelineListQuery,
    ) -> git_cognition_core::RequestUrl {
        GitHubPipelineCollection::make(self.api_base_url()).list(query)
    }

    fn pipeline_rerun_request(
        &self,
        pipeline: &git_cognition_core::Pipeline,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(GitHubPipeline::make(self.api_base_url(), pipeline.clone()).rerun())
    }

    fn pipeline_cancel_request(
        &self,
        pipeline: &git_cognition_core::Pipeline,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(GitHubPipeline::make(self.api_base_url(), pipeline.clone()).cancel())
    }
}
