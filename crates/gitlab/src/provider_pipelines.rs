use git_cognition_core::ManagedPipelineProvider;

use crate::{GitLabPipeline, GitLabPipelineCollection, GitLabProvider};

impl ManagedPipelineProvider for GitLabProvider {
    fn pipeline_url(
        &self,
        pipeline: &git_cognition_core::Pipeline,
    ) -> git_cognition_core::RequestUrl {
        GitLabPipeline::make(self.api_base_url(), pipeline.clone()).url()
    }

    fn pipeline_list_url(
        &self,
        query: &git_cognition_core::PipelineListQuery,
    ) -> git_cognition_core::RequestUrl {
        GitLabPipelineCollection::make(self.api_base_url()).list(query)
    }

    fn pipeline_rerun_request(
        &self,
        pipeline: &git_cognition_core::Pipeline,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(GitLabPipeline::make(self.api_base_url(), pipeline.clone()).rerun())
    }

    fn pipeline_cancel_request(
        &self,
        pipeline: &git_cognition_core::Pipeline,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(GitLabPipeline::make(self.api_base_url(), pipeline.clone()).cancel())
    }
}
