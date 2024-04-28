use utoipa::{
    openapi::{
        self,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
    Modify, OpenApi,
};

use crate::types;
use crate::controller::user::user_controller;
use crate::controller::user::auth_controller;

#[derive(OpenApi)]
#[openapi(
    paths(
        user_controller::login,
        user_controller::current_user,
        auth_controller::refresh_access_token,
        auth_controller::verify_access_token
    ),
    components(
        schemas(
            utoipa::TupleUnit,
            types::GenericPostRequest,
            types::GenericPostResponse,
            types::GenericStringResponse,
            types::PostRequest,
            types::PostResponse,
        )
    ),
    tags((name = "BasicAPI", description = "A very Basic API")),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        // NOTE: we can unwrap safely since there already is components registered.
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "Token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}
