use vcs_provider_core::ManagedPipelineProvider;

use crate::{DEFAULT_BASE_URL, GitLabPipeline, GitLabPipelineCollection, GitLabProvider};

impl ManagedPipelineProvider for GitLabProvider {
    fn pipeline_url(
        &self,
        pipeline: &vcs_provider_core::Pipeline,
    ) -> vcs_provider_core::RequestUrl {
        GitLabPipeline::make(DEFAULT_BASE_URL, pipeline.clone()).url()
    }

    fn pipeline_list_url(
        &self,
        query: &vcs_provider_core::PipelineListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitLabPipelineCollection::make(DEFAULT_BASE_URL).list(query)
    }

    fn pipeline_rerun_request(
        &self,
        pipeline: &vcs_provider_core::Pipeline,
    ) -> vcs_provider_core::VcsResult<vcs_provider_core::Request> {
        Ok(GitLabPipeline::make(DEFAULT_BASE_URL, pipeline.clone()).rerun())
    }

    fn pipeline_cancel_request(
        &self,
        pipeline: &vcs_provider_core::Pipeline,
    ) -> vcs_provider_core::VcsResult<vcs_provider_core::Request> {
        Ok(GitLabPipeline::make(DEFAULT_BASE_URL, pipeline.clone()).cancel())
    }
}
