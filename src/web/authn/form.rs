use rocket::get;
use rocket::Responder;
use rocket_dyn_templates::Template;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use crate::jwt::session;
use crate::web::error;


#[derive(serde::Serialize)]
struct LoginContext<'a> {
    page_name: &'a str,
    client_name: Option<String>,
    session_jwt: Option<String>,
}

#[derive(Responder)]
pub enum AuthnLoginResponse {
	#[response(status = 200)]
	GenericLoginForm(Template),
	#[response(status = 200)]
	ClientLoginForm(Template),
	#[response(status = 400)]
	BadRequest(Template),
	#[response(status = 500)]
	ServerError(Template),
}

#[get("/login?<session>")]
pub fn login(session: Option<String>) -> AuthnLoginResponse {
    // Check the session JWT
    match session {
        Some(t) => {
			// TODO fetch from configuration
			let cfg_jwt_internal_key   = b"secret";
			let cfg_jwt_oidc_issuer    = "https://localhost:8000/oidc".to_string();
			let cfg_jwt_authn_audience = "https://localhost:8000/authn".to_string();

			let mut validation = Validation::new(Algorithm::HS256);
			validation.set_required_spec_claims(&["exp", "aud", "iss"]);
			validation.set_audience(&[cfg_jwt_authn_audience]);
			validation.set_issuer(&[cfg_jwt_oidc_issuer]);
			
			match jsonwebtoken::decode::<session::OIDCSessionClaims>(
                &t,
                &DecodingKey::from_secret(cfg_jwt_internal_key),
                &validation) {
                Ok(d) => {
                    AuthnLoginResponse::ClientLoginForm(
                        Template::render("authn_login", &LoginContext {
                            page_name: "Authentication",
                            client_name: Some(d.claims.client_name),
                            session_jwt: Some(t),
                        })
                    )
                },
                Err(e) => AuthnLoginResponse::BadRequest(error::error_page(
                    400,
                    "Failed to decode the JWT token",
                    format!("{}", e).as_str()))
            }
        },
        None => AuthnLoginResponse::GenericLoginForm(
            Template::render("authn_login", &LoginContext {
                page_name: "Authentication",
                client_name: None,
                session_jwt: None,
            })),
    }
}
