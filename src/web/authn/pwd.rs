use rocket::post;
use rocket::Responder;
use rocket_dyn_templates::Template;
use rocket::form::{Form, FromForm, Strict};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use crate::jwt::session;
use crate::web::error;


#[derive(FromForm)]
pub struct AuthnForm<'r> {
	u_login: &'r str,
	u_password: &'r str,
}


#[derive(serde::Serialize)]
struct AuthnSuccessContext<'a> {
    page_name: &'a str,
    user_name: &'a str,
    session_jwt: Option<String>,
}

#[derive(serde::Serialize)]
struct AuthnFailureContext<'a> {
    page_name: &'a str,
    session_jwt: Option<String>,
}

#[derive(Responder)]
pub enum PwdFormResponse {
	#[response(status = 200)]
	AuthnSuccess(Template),
	#[response(status = 401)]
	AuthnFailure(Template),
	#[response(status = 400)]
	BadRequest(Template),
	#[response(status = 500)]
	ServerError(Template),
}


#[post("/login_pwd?<session>", data="<p_authn>")]
pub fn login_pwd(session: Option<String>, p_authn: Form<Strict<AuthnForm<'_>>>) -> PwdFormResponse {
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
                Ok(_) => {
					if p_authn.u_login.eq("ptramo") && p_authn.u_password.eq("plopplopplop") {
						PwdFormResponse::AuthnSuccess(
							Template::render("authn_success", &AuthnSuccessContext {
								page_name: "Authentication succeeded",
								user_name: "Pierre TRAMO",
								session_jwt: Some(t),
							})
						)
					}
					else {
						PwdFormResponse::AuthnFailure(
							Template::render("authn_failure", &AuthnFailureContext {
								page_name: "Authentication failed",
								session_jwt: Some(t),
							})
						)
					}
                },
                Err(e) => PwdFormResponse::BadRequest(error::error_page(
                    400,
                    "Failed to decode the JWT token",
                    format!("{}", e).as_str()))
            }
        },
        None => {
			if p_authn.u_login.eq("ptramo") && p_authn.u_password.eq("plopplopplop") {
				PwdFormResponse::AuthnSuccess(
					Template::render("authn_success", &AuthnSuccessContext {
						page_name: "Authentication succeeded",
						user_name: "Pierre TRAMO",
						session_jwt: None,
					})
				)
			}
			else {
				PwdFormResponse::AuthnFailure(
					Template::render("authn_failure", &AuthnFailureContext {
						page_name: "Authentication failed",
						session_jwt: None,
					})
				)
			}
        }
    }

}
