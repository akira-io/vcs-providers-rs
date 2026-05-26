use vcs_provider_core::ManagedPipelineProvider;

use crate::{BitbucketPipeline, BitbucketPipelineCollection, BitbucketProvider, DEFAULT_BASE_URL};

impl ManagedPipelineProvider for BitbucketProvider {
    fn pipeline_url(
        &self,
        pipeline: &vcs_provider_core::Pipeline,
    ) -> vcs_provider_core::RequestUrl {
        BitbucketPipeline::make(DEFAULT_BASE_URL, pipeline.clone()).url()
    }

    fn pipeline_list_url(
        &self,
        query: &vcs_provider_core::PipelineListQuery,
    ) -> vcs_provider_core::RequestUrl {
        BitbucketPipelineCollection::make(DEFAULT_BASE_URL).list(query)
    }

    fn pipeline_rerun_request(
        &self,
        _pipeline: &vcs_provider_core::Pipeline,
    ) -> vcs_provider_core::VcsResult<vcs_provider_core::Request> {
        Err(vcs_provider_core::error().invalid_input(
            "bitbucket pipeline rerun is not exposed by a validated pipeline endpoint",
        ))
    }

    fn pipeline_cancel_request(
        &self,
        pipeline: &vcs_provider_core::Pipeline,
    ) -> vcs_provider_core::VcsResult<vcs_provider_core::Request> {
        Ok(BitbucketPipeline::make(DEFAULT_BASE_URL, pipeline.clone()).cancel())
    }
}
