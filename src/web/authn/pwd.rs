use rocket::post;
use rocket::{Responder, State};
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket::form::{Form, FromForm, Strict};
use std::sync::RwLock;
use crate::config;
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
	#[response(status = 303)]
	AuthnSuccess(Redirect),
	#[response(status = 401)]
	AuthnFailure(Template),
	#[response(status = 400)]
	BadSessionToken(Template),
}


#[post("/login_pwd?<session>", data="<p_authn>")]
pub fn login_pwd(
		session: String,
		p_authn: Form<Strict<AuthnForm<'_>>>,
		p_config: &State<RwLock<config::Config>>) -> PwdFormResponse {
    // Check the session JWT
	let config = p_config.read().unwrap();
	match session::check_token(&session, &config) {
		Ok(_) => {
			if p_authn.u_login.eq("ptramo") && p_authn.u_password.eq("plopplopplop") {
				PwdFormResponse::AuthnSuccess(
					Redirect::to(format!("/oidc/auth_redirect?session={}", session))
				)
			}
			else {
				PwdFormResponse::AuthnFailure(
					Template::render("authn_failure", &AuthnFailureContext {
						page_name: "Authentication failed",
						session_jwt: Some(session),
					})
				)
			}
		},
		Err(e) => PwdFormResponse::BadSessionToken(error::error_page(
			400,
			"Failed to decode the JWT token",
			format!("{}", e).as_str()))
	}
}
