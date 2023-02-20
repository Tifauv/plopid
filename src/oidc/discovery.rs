use rocket::get;
use rocket::State;
use rocket::serde::{Serialize, json::Json};
use std::sync::RwLock;
use crate::config;

#[derive(Serialize)]
pub struct OpenIDConfiguration<'r> {
	issuer: String,
	authorization_endpoint: String,
	token_endpoint: String,
	userinfo_endpoint: String,
	scopes_supported: Vec<&'r str>,
	grant_types_supported: Vec<&'r str>,
	response_types_supported: Vec<&'r str>,
}

#[get("/openid-configuration")]
pub fn endpoint<'r>(p_config: &'r State<RwLock<config::Config>>) -> Json<OpenIDConfiguration<'r>> {
	let config = p_config.read().unwrap();

	Json(OpenIDConfiguration {
		issuer                   : String::from(&config.jwt.oidc_issuer),
		authorization_endpoint   : format!("https://{}:{}/oidc/auth", config.server.host, config.server.port),
		token_endpoint           : format!("https://{}:{}/oidc/token", config.server.host, config.server.port),
		userinfo_endpoint        : format!("https://{}:{}/oidc/userinfo", config.server.host, config.server.port),
		scopes_supported         : [ "openid" ].to_vec(),
		grant_types_supported    : [ "authorization_code" ].to_vec(),
		response_types_supported : [ "code" ].to_vec(),
	})
}
