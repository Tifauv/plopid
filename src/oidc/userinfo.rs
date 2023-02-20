use rocket::get;
use rocket::{Responder, State};
use rocket_dyn_templates::Template;
use std::sync::RwLock;
use crate::services::registry;
use crate::web::error;


#[derive(Responder)]
pub enum OIDCUserInfoResponse {
	#[response(status = 200)]
	UserInfo(Template),
	#[response(status = 401)]
	Unauthorized(Template),
	#[response(status = 403)]
	Forbidden(Template),
	#[response(status = 500)]
	ServerError(Template),
}


#[get("/userinfo")]
pub fn endpoint(_service_registry: &State<RwLock<registry::ServiceRegistry>>) -> OIDCUserInfoResponse {
	OIDCUserInfoResponse::ServerError(error::error_page(
			500,
			"Not implemented",
			"The OIDC userinfo endpoint is not yet implemented"))
}
