use thehaven_interfaces::AuthSender;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::debug;
use serde::{Deserialize};

use thehaven_interfaces::Auth;

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
        let path = &req.path[1..req.path.len()];
        let segments: Vec<&str> = path.trim_end_matches('/').split('/').collect();

        match (req.method.as_str(), segments.as_slice()) {
            ("GET", ["api", "auth", "inittables"]) => {
                let auth_actor = AuthSender::to_actor(AUTH_ACTOR);
                
                if auth_actor.init_tables(&_ctx).await? {
                    HttpResponse::ok("OK")
                } else {
                    Ok(HttpResponse::not_found())
                }
            }
            ("POST", ["api", "auth", "register"]) => {
                let auth_actor = AuthSender::to_actor(AUTH_ACTOR);
                let register_request = deser(&req.body)?;
                
                let returned_user = auth_actor.register(&_ctx, &register_request).await?;
                HttpResponse::json(returned_user, 200)
            }
            ("POST", ["api", "auth", "login"]) => {
                let auth_actor = AuthSender::to_actor(AUTH_ACTOR);
                let login_request = deser(&req.body)?;
                
                let token = auth_actor.login(&_ctx, &login_request).await?;
                HttpResponse::json(token, 200)
            }
            ("GET", ["api", "auth", "getuserrole", userid]) => {
                let auth_actor = AuthSender::to_actor(AUTH_ACTOR);
                
                let role = auth_actor.get_user_role(&_ctx, &userid).await?;
                HttpResponse::json(role, 200)
            }
            ("GET", ["api", "auth", "getallroles"]) => {
                let auth_actor = AuthSender::to_actor(AUTH_ACTOR);
                
                let roles = auth_actor.get_roles(&_ctx).await?;
                HttpResponse::json(roles, 200)
            }
            (_, _) => Ok(HttpResponse::not_found()),
        }

    }
}

fn deser<'de, T: Deserialize<'de>>(raw: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(raw).map_err(|e| RpcError::Deser(format!("{}", e)))
}
