use rocket::get;
use rocket::{Responder, State};
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use jsonwebtoken::{EncodingKey, Header};
use std::sync::RwLock;
use crate::config;
use crate::jwt::session;
use crate::services::registry;
use crate::services::client;
use crate::web::error;


#[derive(Debug, PartialEq)]
pub struct OIDCAuthRequest<'a> {
	scopes: Vec<String>,
	client: &'a client::RequestingClient,
	state: Option<String>,
	nonce: Option<String>,
}


#[derive(Clone, PartialEq)]
pub enum ResponseType {
	Code,
	IdToken,
	Unsupported,
}

impl ResponseType {
	pub fn from_str(p_response_type: &str) -> ResponseType {
		match p_response_type {
			"code"     => ResponseType::Code,
			"id_token" => ResponseType::IdToken,
			_          => ResponseType::Unsupported,
		}
	}
}

fn check_scope(p_scope: &String) -> bool {
	p_scope.split(" ").collect::<Vec<_>>().contains(&"openid")
}


#[derive(Responder)]
pub enum OIDCAuthResponse {
	#[response(status = 303)]
	RedirectLogin(Redirect),
	#[response(status = 400)]
	BadRequest(Template),
	#[response(status = 500)]
	ServerError(Template),
}

#[get("/auth?<scope>&<response_type>&<client_id>&<redirect_uri>&<state>&<nonce>")]
pub fn endpoint(
		scope:         String,
		response_type: String,
		client_id:     String,
		redirect_uri:  String,
		state:         Option<String>,
		nonce:         Option<String>,
		p_service_registry: &State<RwLock<registry::ServiceRegistry>>,
		p_config:           &State<RwLock<config::Config>>) -> OIDCAuthResponse {
	// TODO Detect the OIDC flow

	// Check the client
	let service_registry = p_service_registry.read().unwrap();
	let client = client::RequestingClient::new(client_id, redirect_uri);
	let client_name = match service_registry.authenticate(&client) {
		Err(_) => return OIDCAuthResponse::BadRequest(error::error_page(
				400,
				"Client authentication failure",
				"Either:<ul><li>the client_id has not been registered to this IdP</li> \
				<li>a client_secret has been registered but none was given</li> \
				<li>the given client_secret does not match the registered one</li> \
				<li>the given redirect_uri does not match any of the registered ones for that client</li></ul>")),
		Ok(c)  => c
	};

	// Check the OIDC minimal scope
	if !check_scope(&scope) {
		return OIDCAuthResponse::BadRequest(error::error_page(
				400,
				"Invalid scope",
				"The <strong>openid</strong> scope MUST be present for OIDC requests")) 
	}

	// Check the response type
	let request = OIDCAuthRequest {
		client: &client,
		scopes: scope.split(" ").map(str::to_string).collect(),
		state: state,
		nonce: nonce,
	};
	match ResponseType::from_str(&response_type) {
		/* Code Flow */
		ResponseType::Code => {
			let config = p_config.read().unwrap();
			let cfg_jwt_oidc_issuer    = String::from(&config.jwt.oidc_issuer);
			let cfg_jwt_internal_key   = config.jwt.internal_key.as_bytes();
			let cfg_jwt_authn_audience = String::from(&config.jwt.internal_authn_aud);
			
			let oidc_claims = session::OIDCSessionClaims {
				aud: cfg_jwt_authn_audience,
				exp: 10000000000, // TODO generate
				iss: cfg_jwt_oidc_issuer,
				client_name:  client_name,
				client_id:    request.client.client_id(),
				redirect_uri: request.client.redirect_uri(),
				scopes:       request.scopes,
				state:        request.state,
				nonce:        request.nonce,
			};
			match jsonwebtoken::encode(&Header::default(), &oidc_claims, &EncodingKey::from_secret(cfg_jwt_internal_key)) {
				Ok(t)  => OIDCAuthResponse::RedirectLogin(Redirect::to(format!("/authn/login?session={}", t))),
				Err(e) => OIDCAuthResponse::ServerError(error::error_page(
						500,
						"Failed to generate the JWT token",
						format!("{}", e).as_str())),
			}
		},
		/* Implicit & other flows */
		ResponseType::IdToken | ResponseType::Unsupported => {
			OIDCAuthResponse::BadRequest(error::error_page(
				400,
				"Invalid response_type",
				"The given response_type is currently not supported")) 
		}
	}
}


#[derive(Responder)]
pub enum OIDCRedirectResponse {
	#[response(status = 303)]
	RedirectToClient(Redirect),
	#[response(status = 400)]
	BadSessionToken(Template),
}

#[get("/auth_redirect?<session>")]
pub fn redirect(session: String, p_config: &State<RwLock<config::Config>>) -> OIDCRedirectResponse {
	// Check the session JWT
	let config = p_config.read().unwrap();
	match session::check_token(&session, &config) {
		Ok(token)  => {
			// TODO Generate access_code
			let access_code = "TODO_generate_access_code";
			
			// Build redirect URI
			let redirect_uri = format!("{}?code={}", token.claims.redirect_uri, access_code);
			
			OIDCRedirectResponse::RedirectToClient(Redirect::to(redirect_uri))
		},
		Err(e) => OIDCRedirectResponse::BadSessionToken(error::error_page(
			400,
			"Failed to decode the JWT token",
			format!("{}", e).as_str())),
	}
}
