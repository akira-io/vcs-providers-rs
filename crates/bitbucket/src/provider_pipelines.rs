use git_cognition_core::ManagedPipelineProvider;

use crate::{BitbucketPipeline, BitbucketPipelineCollection, BitbucketProvider};

impl ManagedPipelineProvider for BitbucketProvider {
    fn pipeline_url(
        &self,
        pipeline: &git_cognition_core::Pipeline,
    ) -> git_cognition_core::RequestUrl {
        BitbucketPipeline::make(self.api_base_url(), pipeline.clone()).url()
    }

    fn pipeline_list_url(
        &self,
        query: &git_cognition_core::PipelineListQuery,
    ) -> git_cognition_core::RequestUrl {
        BitbucketPipelineCollection::make(self.api_base_url()).list(query)
    }

    fn pipeline_rerun_request(
        &self,
        _pipeline: &git_cognition_core::Pipeline,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Err(git_cognition_core::error().invalid_input(
            "bitbucket pipeline rerun is not exposed by a validated pipeline endpoint",
        ))
    }

    fn pipeline_cancel_request(
        &self,
        pipeline: &git_cognition_core::Pipeline,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(BitbucketPipeline::make(self.api_base_url(), pipeline.clone()).cancel())
    }
}
