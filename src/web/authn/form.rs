use rocket::get;
use rocket::{Responder, State};
use rocket_dyn_templates::Template;
use std::sync::RwLock;
use crate::config;
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
pub fn login(session: Option<String>, p_config: &State<RwLock<config::Config>>) -> AuthnLoginResponse {
    // Check the session JWT
    match session {
        Some(token) => {
			let config = p_config.read().unwrap();
			match session::check_token(&token, &config) {
                Ok(decoded) => {
                    AuthnLoginResponse::ClientLoginForm(
                        Template::render("authn_login", &LoginContext {
                            page_name: "Authentication",
                            client_name: Some(decoded.claims.client_name),
                            session_jwt: Some(token),
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
