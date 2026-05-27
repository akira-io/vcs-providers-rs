use vcs_provider_core::ManagedPipelineProvider;

use crate::{GitHubPipeline, GitHubPipelineCollection, GitHubProvider};

impl ManagedPipelineProvider for GitHubProvider {
    fn pipeline_url(
        &self,
        pipeline: &vcs_provider_core::Pipeline,
    ) -> vcs_provider_core::RequestUrl {
        GitHubPipeline::make(self.api_base_url(), pipeline.clone()).url()
    }

    fn pipeline_list_url(
        &self,
        query: &vcs_provider_core::PipelineListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitHubPipelineCollection::make(self.api_base_url()).list(query)
    }

    fn pipeline_rerun_request(
        &self,
        pipeline: &vcs_provider_core::Pipeline,
    ) -> vcs_provider_core::VcsResult<vcs_provider_core::Request> {
        Ok(GitHubPipeline::make(self.api_base_url(), pipeline.clone()).rerun())
    }

    fn pipeline_cancel_request(
        &self,
        pipeline: &vcs_provider_core::Pipeline,
    ) -> vcs_provider_core::VcsResult<vcs_provider_core::Request> {
        Ok(GitHubPipeline::make(self.api_base_url(), pipeline.clone()).cancel())
    }
}
