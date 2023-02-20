use rocket::post;
use rocket::{Responder, State};
use rocket_dyn_templates::Template;
use std::sync::RwLock;
use crate::services::registry;
use crate::web::error;


#[derive(Responder)]
pub enum OIDCTokenResponse {
	#[response(status = 200)]
	AccessToken(Template),
	#[response(status = 500)]
	ServerError(Template),
}

#[post("/token")]
pub fn endpoint(_service_registry: &State<RwLock<registry::ServiceRegistry>>) -> OIDCTokenResponse {
	OIDCTokenResponse::ServerError(error::error_page(
			500,
			"Not implemented",
			"The OIDC token endpoint is not yet implemented"))
}
