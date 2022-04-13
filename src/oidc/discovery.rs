use rocket::get;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
pub struct OpenIDConfiguration<'r> {
	issuer: &'r str,
	authorization_endpoint: &'r str,
	token_endpoint: &'r str,
	userinfo_endpoint: &'r str,
	scopes_supported: Vec<&'r str>,
	grant_types_supported: Vec<&'r str>,
	response_types_supported: Vec<&'r str>,
}

#[get("/openid-configuration")]
pub fn oidc_discovery<'r>() -> Json<OpenIDConfiguration<'r>> {
	Json(OpenIDConfiguration {
		issuer                   : "https://localhost:8000/oidc",
		authorization_endpoint   : "https://localhost:8000/oidc/login",
		token_endpoint           : "https://localhost:8000/oidc/token",
		userinfo_endpoint        : "https://localhost:8000/oidc/userinfo",
		scopes_supported         : [ "openid" ].to_vec(),
		grant_types_supported    : [ "authorization_code" ].to_vec(),
		response_types_supported : [ "code" ].to_vec(),
	})
}
