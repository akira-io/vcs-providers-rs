use crate::{BoxFuture, Request, Response, Transport, VcsResult, response};

#[derive(Clone, Copy, Debug, Default)]
pub struct EchoTransport;

impl Transport for EchoTransport {
    fn send(&self, request: Request) -> BoxFuture<'_, VcsResult<Response>> {
        Box::pin(async move {
            let mut response = response().status(200);

            for header in request.headers() {
                response = response.header(header.name().as_str(), header.value().as_str());
            }

            Ok(response.build())
        })
    }
}
