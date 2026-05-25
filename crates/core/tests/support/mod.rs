use vcs_provider_core::{BoxFuture, Request, Response, ResponseStatus, Transport, VcsResult};

#[derive(Clone, Copy, Debug, Default)]
pub struct EchoTransport;

impl Transport for EchoTransport {
    fn send(&self, request: Request) -> BoxFuture<'_, VcsResult<Response>> {
        Box::pin(async move {
            Ok(Response::make(
                ResponseStatus::make(200),
                request.headers().to_vec(),
            ))
        })
    }
}
