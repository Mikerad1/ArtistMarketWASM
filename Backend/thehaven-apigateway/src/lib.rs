use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::debug;

const AUTH_ACTOR: &str = "the_haven/auth";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct ThehavenApigatewayActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for ThehavenApigatewayActor {
    /// Returns a greeting, "Hello World", in the response body.
    /// If the request contains a query parameter 'name=NAME', the
    /// response is changed to "Hello NAME"
    async fn handle_request(
        &self,
        _ctx: &Context,
        req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        debug!("API request: {:?}", req);

        Ok(HttpResponse::not_found())
    }
}
